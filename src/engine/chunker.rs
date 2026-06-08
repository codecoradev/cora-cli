//! Diff chunker — splits large diffs into review-sized chunks.
//!
//! When a diff exceeds `max_diff_size`, this module splits it into
//! smaller chunks (grouped by file) that each fit within the limit.
//! Each chunk is a valid unified diff that can be reviewed independently.

use crate::engine::diff_parser;
use tracing::debug;

/// A chunk of diff ready for review.
#[derive(Debug, Clone)]
pub struct DiffChunk {
    /// Chunk index (0-based).
    pub index: usize,
    /// Total number of chunks.
    pub total: usize,
    /// Label for progress reporting (e.g., "src/config/*").
    pub label: String,
    /// The unified diff text for this chunk.
    pub diff: String,
    /// Files included in this chunk.
    pub file_count: usize,
}

/// Split a large diff into chunks that each fit within `max_size` bytes.
///
/// Grouping strategy: files are grouped by top-level directory prefix.
/// If a single file exceeds `max_size`, it gets its own chunk (truncated).
pub fn chunk_diff(diff: &str, max_size: usize) -> Vec<DiffChunk> {
    let file_chunks = diff_parser::parse_diff(diff);

    if file_chunks.is_empty() {
        return vec![DiffChunk {
            index: 0,
            total: 1,
            label: "empty".to_string(),
            diff: diff.to_string(),
            file_count: 0,
        }];
    }

    // Reconstruct per-file diff blocks from original diff
    let file_diffs = split_raw_diff_by_file(diff, &file_chunks);

    // Group files into chunks that fit within max_size
    let groups = group_by_size(&file_diffs, max_size);

    let total = groups.len();
    groups
        .into_iter()
        .enumerate()
        .map(|(i, group)| {
            let label = derive_label(&group);
            let combined_diff = group.iter().map(|f| f.diff.as_str()).collect::<String>();
            let file_count = group.len();
            DiffChunk {
                index: i,
                total,
                label,
                diff: combined_diff,
                file_count,
            }
        })
        .collect()
}

/// A single file's diff block with metadata.
struct FileDiff {
    /// Top-level directory prefix (e.g., "src", "docs").
    dir_prefix: String,
    /// The raw diff text for this file.
    diff: String,
    /// Byte size of the diff.
    size: usize,
}

/// Split raw diff text into per-file diff blocks.
fn split_raw_diff_by_file(diff: &str, file_chunks: &[diff_parser::FileChunk]) -> Vec<FileDiff> {
    let mut file_diffs = Vec::new();
    let lines: Vec<&str> = diff.lines().collect();
    let mut line_idx = 0;

    for fc in file_chunks {
        let path = fc
            .new_path
            .as_deref()
            .or(fc.old_path.as_deref())
            .unwrap_or("unknown");

        // Find the start of this file's diff in the raw text
        // Look for "--- a/<path>" or "--- /dev/null" for new files
        let start = find_file_start(&lines, line_idx, path);

        if let Some(start) = start {
            // Find end: next file header or end of diff
            let end = find_file_end(&lines, start + 1);
            let file_diff_text = lines[start..end].join("\n");

            file_diffs.push(FileDiff {
                dir_prefix: get_dir_prefix(path),
                diff: file_diff_text,
                size: lines[start..end].join("\n").len(),
            });
            line_idx = end;
        } else {
            // Fallback: couldn't find in raw text, reconstruct from parsed data
            debug!(path = path, "could not locate file in raw diff, skipping");
        }
    }

    // Handle case where parsing found nothing
    if file_diffs.is_empty() && !diff.is_empty() {
        file_diffs.push(FileDiff {
            dir_prefix: "all".to_string(),
            diff: diff.to_string(),
            size: diff.len(),
        });
    }

    file_diffs
}

/// Find the start line of a file's diff block.
fn find_file_start(lines: &[&str], start_from: usize, path: &str) -> Option<usize> {
    // Look for "--- a/<path>" or "--- /dev/null"
    for i in start_from..lines.len() {
        let line = lines[i];
        if line.starts_with("--- ") {
            // Check if next line is "+++ b/<path>"
            if i + 1 < lines.len() {
                let next = lines[i + 1];
                if next.starts_with("+++ ") {
                    let matched = if line.contains("/dev/null") {
                        next.contains(path)
                    } else {
                        line.contains(path) || next.contains(path)
                    };
                    if matched {
                        return Some(i);
                    }
                }
            }
        }
    }
    None
}

/// Find the end of a file's diff block (start of next file or end of input).
fn find_file_end(lines: &[&str], start_from: usize) -> usize {
    for i in start_from..lines.len() {
        // A new file starts with "--- " followed by "+++ "
        if lines[i].starts_with("--- ") && i + 1 < lines.len() && lines[i + 1].starts_with("+++ ")
        {
            // Check it's not a hunk line that starts with "---"
            // File headers have "--- a/" pattern, hunks have "---1,5" pattern
            if lines[i].starts_with("--- a/") || lines[i].starts_with("--- /dev/null") {
                return i;
            }
        }
    }
    lines.len()
}

/// Group file diffs into chunks that fit within max_size.
fn group_by_size(file_diffs: &[FileDiff], max_size: usize) -> Vec<Vec<&FileDiff>> {
    let mut groups: Vec<Vec<&FileDiff>> = Vec::new();
    let mut current_group: Vec<&FileDiff> = Vec::new();
    let mut current_size: usize = 0;

    for fd in file_diffs {
        // If adding this file exceeds max_size, start a new group
        if !current_group.is_empty() && current_size + fd.size > max_size {
            groups.push(current_group);
            current_group = Vec::new();
            current_size = 0;
        }

        // Even if a single file exceeds max_size, it gets its own group
        current_size += fd.size;
        current_group.push(fd);
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}

/// Extract top-level directory prefix from a file path.
fn get_dir_prefix(path: &str) -> String {
    let path = path.trim_start_matches("a/").trim_start_matches("b/");
    path.split('/').next().unwrap_or("root").to_string()
}

/// Derive a human-readable label for a chunk group.
fn derive_label(group: &[&FileDiff]) -> String {
    if group.len() == 1 {
        let path = group[0].dir_prefix.clone();
        format!("{}/*", path)
    } else {
        let dirs: Vec<&str> = group.iter().map(|f| f.dir_prefix.as_str()).collect();
        let unique: Vec<&str> = {
            let mut d = dirs.clone();
            d.sort();
            d.dedup();
            d
        };
        if unique.len() == 1 {
            format!("{}/*", unique[0])
        } else if unique.len() <= 3 {
            unique.join(", ")
        } else {
            format!("{} dirs", unique.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_diff() -> &'static str {
        "\
diff --git a/src/main.rs b/src/main.rs
index abc123..def456 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,5 +1,6 @@
 fn main() {
-    println!(\"hello\");
+    println!(\"hello world\");
+    // added line
 }

diff --git a/src/lib.rs b/src/lib.rs
new file mode 100644
--- /dev/null
+++ b/src/lib.rs
@@ -0,0 +1,3 @@
+pub fn add(a: i32, b: i32) -> i32 {
+    a + b
+}

diff --git a/docs/README.md b/docs/README.md
--- a/docs/README.md
+++ b/docs/README.md
@@ -1,3 +1,4 @@
 # Title
+Added line
 Some text
 End
"
    }

    #[test]
    fn chunk_diff_single_chunk_when_small() {
        let chunks = chunk_diff(sample_diff(), 100_000);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].total, 1);
    }

    #[test]
    fn chunk_diff_splits_when_exceeds_max() {
        // Very small max to force splitting
        let chunks = chunk_diff(sample_diff(), 50);
        assert!(chunks.len() > 1, "Should split into multiple chunks");
    }

    #[test]
    fn chunk_labels_are_human_readable() {
        let chunks = chunk_diff(sample_diff(), 50);
        for chunk in &chunks {
            assert!(!chunk.label.is_empty());
        }
    }

    #[test]
    fn empty_diff_returns_single_chunk() {
        let chunks = chunk_diff("", 1000);
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn get_dir_prefix_strips_a_prefix() {
        assert_eq!(get_dir_prefix("a/src/main.rs"), "src");
        assert_eq!(get_dir_prefix("b/docs/README.md"), "docs");
        assert_eq!(get_dir_prefix("lib.rs"), "lib.rs");
    }

    #[test]
    fn group_by_size_respects_max() {
        let files = vec![
            FileDiff {
                dir_prefix: "src".to_string(),
                diff: "a".repeat(100),
                size: 100,
            },
            FileDiff {
                dir_prefix: "docs".to_string(),
                diff: "b".repeat(100),
                size: 100,
            },
            FileDiff {
                dir_prefix: "src".to_string(),
                diff: "c".repeat(50),
                size: 50,
            },
        ];
        let groups = group_by_size(&files, 150);
        // First two files = 200 > 150, so first file alone
        // Second file (100) + third (50) = 150 <= 150, together
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn derive_label_single_file() {
        let files = vec![&FileDiff {
            dir_prefix: "src".to_string(),
            diff: String::new(),
            size: 0,
        }];
        assert_eq!(derive_label(&files), "src/*");
    }

    #[test]
    fn derive_label_multiple_dirs() {
        let files = vec![
            &FileDiff {
                dir_prefix: "src".to_string(),
                diff: String::new(),
                size: 0,
            },
            &FileDiff {
                dir_prefix: "docs".to_string(),
                diff: String::new(),
                size: 0,
            },
        ];
        assert_eq!(derive_label(&files), "docs, src");
    }
}
