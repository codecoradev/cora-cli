//! Markdown awareness for the review/scan pipeline.
//!
//! Markdown files (`.md`, `.mdx`, `.markdown`) frequently contain fenced code
//! blocks that are **documentation examples**, not executable code. Treating
//! their contents like real source code produces false positives — e.g. a
//! `git push` line inside a ` ```bash ` block flagged as SQL injection (#329).
//!
//! This module detects fenced code blocks (` ``` ` / `~~~`) within a diff chunk
//! and exposes which new-file line numbers fall inside them, so findings can be
//! filtered out.

use std::collections::HashSet;

use crate::engine::diff_parser::{DiffLineType, FileChunk};

/// True if the path looks like a Markdown documentation file.
pub fn is_markdown(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".md") || lower.ends_with(".mdx") || lower.ends_with(".markdown")
}

/// Compute the set of new-file line numbers that fall **inside** fenced code
/// blocks for a single diff chunk.
///
/// Only lines present in the *new* file are considered (Add + Context). Removed
/// lines are skipped so they cannot corrupt fence state. The opening fence does
/// not need to be part of the diff itself — context lines carry it too, which
/// keeps this reliable when only the body of a code block was edited.
///
/// Fence state is tracked across all hunks in the chunk in document order. This
/// is exact when a code block lives within contiguous lines and a best-effort
/// approximation when a fence spans non-contiguous hunks.
pub fn lines_inside_code_blocks(chunk: &FileChunk) -> HashSet<u32> {
    let mut inside = HashSet::new();
    let mut in_fence = false;

    for hunk in &chunk.chunks {
        for line in &hunk.lines {
            // Removed lines don't exist in the new file — skip to keep fence
            // state faithful to the post-change document.
            if line.line_type == DiffLineType::Remove {
                continue;
            }

            let trimmed = line.content.trim_start();
            let is_fence = trimmed.starts_with("```") || trimmed.starts_with("~~~");
            if is_fence {
                in_fence = !in_fence;
                continue;
            }

            if in_fence {
                if let Some(ln) = line.new_line_no {
                    if ln > 0 {
                        inside.insert(ln);
                    }
                }
            }
        }
    }

    inside
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::diff_parser::{DiffHunk, DiffLine};

    fn chunk(file: &str, lines: &[(DiffLineType, &str, u32)]) -> FileChunk {
        FileChunk {
            old_path: None,
            new_path: Some(file.to_string()),
            language: "markdown".to_string(),
            chunks: vec![DiffHunk {
                old_start: 1,
                old_count: 1,
                new_start: 1,
                new_count: lines.len() as u32,
                header: String::new(),
                lines: lines
                    .iter()
                    .map(|(lt, content, ln)| DiffLine {
                        line_type: *lt,
                        content: content.to_string(),
                        old_line_no: None,
                        new_line_no: Some(*ln),
                    })
                    .collect(),
            }],
            is_binary: false,
            is_deleted: false,
            is_new: false,
        }
    }

    #[test]
    fn detects_markdown_extensions() {
        assert!(is_markdown("README.md"));
        assert!(is_markdown("docs/guide.MDX"));
        assert!(is_markdown("notes.markdown"));
        assert!(!is_markdown("src/main.rs"));
        assert!(!is_markdown("config.yaml"));
    }

    #[test]
    fn lines_inside_backtick_fence() {
        let c = chunk(
            "doc.md",
            &[
                (DiffLineType::Add, "Intro", 1),
                (DiffLineType::Add, "```bash", 2),
                (DiffLineType::Add, "git push origin vX.Y.Z", 3),
                (DiffLineType::Add, "git tag vX.Y.Z", 4),
                (DiffLineType::Add, "```", 5),
                (DiffLineType::Add, "Done.", 6),
            ],
        );
        let inside = lines_inside_code_blocks(&c);
        assert!(inside.contains(&3));
        assert!(inside.contains(&4));
        assert!(!inside.contains(&1));
        assert!(!inside.contains(&6));
        // Fence lines themselves are not "inside".
        assert!(!inside.contains(&2));
        assert!(!inside.contains(&5));
    }

    #[test]
    fn lines_inside_tilde_fence() {
        let c = chunk(
            "doc.md",
            &[
                (DiffLineType::Add, "~~~sql", 1),
                (DiffLineType::Add, "SELECT 1;", 2),
                (DiffLineType::Add, "~~~", 3),
            ],
        );
        let inside = lines_inside_code_blocks(&c);
        assert!(inside.contains(&2));
    }

    #[test]
    fn no_fence_yields_empty() {
        let c = chunk(
            "doc.md",
            &[
                (DiffLineType::Add, "Just prose.", 1),
                (DiffLineType::Add, "No code here.", 2),
            ],
        );
        assert!(lines_inside_code_blocks(&c).is_empty());
    }

    #[test]
    fn context_line_fence_covers_added_body() {
        // Only the body line is added; the fences are pre-existing context.
        let c = chunk(
            "doc.md",
            &[
                (DiffLineType::Context, "```bash", 2),
                (DiffLineType::Add, "git push origin vX.Y.Z", 3),
                (DiffLineType::Context, "```", 4),
            ],
        );
        let inside = lines_inside_code_blocks(&c);
        assert!(
            inside.contains(&3),
            "added body inside context fence must be detected"
        );
    }

    #[test]
    fn removed_lines_do_not_corrupt_fence_state() {
        // A removed closing fence must not leave the tracker stuck "inside".
        let c = chunk(
            "doc.md",
            &[
                (DiffLineType::Add, "```bash", 1),
                (DiffLineType::Remove, "old line", 0), // removed; old file only
                (DiffLineType::Add, "git push", 2),
                (DiffLineType::Add, "```", 3),
                (DiffLineType::Add, "prose after", 4),
            ],
        );
        let inside = lines_inside_code_blocks(&c);
        assert!(inside.contains(&2));
        assert!(
            !inside.contains(&4),
            "prose after a closed fence must not be flagged"
        );
    }

    #[test]
    fn indented_fence_marker_works() {
        // Some templates indent fences; the marker is detected after trim.
        let c = chunk(
            "doc.md",
            &[
                (DiffLineType::Add, "  ```", 1),
                (DiffLineType::Add, "  code", 2),
                (DiffLineType::Add, "  ```", 3),
            ],
        );
        let inside = lines_inside_code_blocks(&c);
        assert!(inside.contains(&2));
    }
}
