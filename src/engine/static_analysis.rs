/// Static analysis integration — run clippy and extract relevant output.
///
/// Used to inject compiler/linter context into the review prompt,
/// reducing false positives on verified-intentional changes.
use tracing::debug;

use crate::config::schema::StaticAnalysisConfig;

/// Maximum clippy output to inject (in characters).
/// Keeps prompt tokens reasonable — clippy output for a focused diff
/// should rarely exceed this.
const MAX_CLIPPY_OUTPUT_CHARS: usize = 4000;

/// Run static analysis and return formatted context string, or None.
///
/// Two modes:
/// 1. `clippy_output_file` — read pre-computed output from file
/// 2. `auto_clippy` — run `cargo clippy` and filter to changed files
pub fn collect_static_context(diff: &str, config: &StaticAnalysisConfig) -> Option<String> {
    if let Some(file_path) = &config.clippy_output_file {
        return read_clippy_file(file_path);
    }

    if config.auto_clippy {
        return run_clippy_for_diff(diff);
    }

    None
}

/// Read clippy output from a pre-computed file.
fn read_clippy_file(path: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    let trimmed = content.trim();

    if trimmed.is_empty() {
        debug!("clippy output file is empty, skipping");
        return None;
    }

    Some(trimmed.to_string())
}

/// Run `cargo clippy` and filter output to lines mentioning files in the diff.
fn run_clippy_for_diff(diff: &str) -> Option<String> {
    let changed_files = extract_changed_rust_files(diff);
    if changed_files.is_empty() {
        debug!("no Rust files changed in diff, skipping clippy");
        return None;
    }

    debug!(
        files = changed_files.len(),
        "running clippy for changed Rust files"
    );

    // Run cargo clippy with short output format, timeout 30 seconds
    let child = match std::process::Command::new("cargo")
        .args([
            "clippy",
            "--message-format=short",
            "--",
            "-W",
            "clippy::all",
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            debug!("clippy spawn failed: {}", e);
            return None;
        }
    };

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => {
            debug!("clippy wait failed: {}", e);
            return None;
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Combine stdout + stderr, clippy sometimes outputs to stderr
    let combined = format!("{}\n{}", stdout, stderr);

    // Filter to lines mentioning changed files
    let mut relevant_lines = Vec::new();
    for line in combined.lines() {
        let trimmed = line.trim();
        // Skip cargo build messages, empty lines, progress output
        if trimmed.is_empty()
            || trimmed.starts_with("Compiling")
            || trimmed.starts_with("Downloading")
            || trimmed.starts_with("Fresh")
            || trimmed.starts_with("Finished")
            || trimmed.starts_with("Updating")
            || trimmed.starts_with("Locking")
        {
            continue;
        }

        // Check if line mentions any changed file
        if changed_files.iter().any(|f| line.contains(f.as_str())) {
            relevant_lines.push(trimmed.to_string());
        }
    }

    if relevant_lines.is_empty() {
        debug!("no clippy warnings for changed files");
        return None;
    }

    let result = relevant_lines.join("\n");

    // Truncate if too long
    if result.len() > MAX_CLIPPY_OUTPUT_CHARS {
        let truncated = &result[..MAX_CLIPPY_OUTPUT_CHARS];
        debug!(
            original_len = result.len(),
            truncated_len = truncated.len(),
            "clippy output truncated"
        );
        Some(format!("{}\n... (truncated)", truncated))
    } else {
        Some(result)
    }
}

/// Extract changed Rust file paths from a diff.
/// Returns base paths like `src/engine/llm.rs` (no `a/` or `b/` prefix).
fn extract_changed_rust_files(diff: &str) -> Vec<String> {
    let mut files = Vec::new();

    for line in diff.lines() {
        let trimmed = line.trim();

        // Match diff headers: --- a/path.rs or +++ b/path.rs
        if let Some(path) = trimmed
            .strip_prefix("--- a/")
            .or_else(|| trimmed.strip_prefix("+++ b/"))
        {
            if path.ends_with(".rs") && !path.contains("target/") {
                files.push(path.to_string());
            }
        }
    }

    files.sort();
    files.dedup();
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_rust_files_from_diff() {
        let diff = "\
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,1 +1,2 @@
 use anyhow::Result;
--- a/src/engine/llm.rs
+++ b/src/engine/llm.rs
@@ -10,1 +10,2 @@
 pub async fn review_diff()";

        let files = extract_changed_rust_files(diff);
        assert_eq!(
            files,
            vec!["src/engine/llm.rs".to_string(), "src/main.rs".to_string()]
        );
    }

    #[test]
    fn extract_skips_non_rust() {
        let diff = "\
--- a/src/main.rs
+++ b/src/main.rs
--- a/package.json
+++ b/package.json";

        let files = extract_changed_rust_files(diff);
        assert_eq!(files, vec!["src/main.rs".to_string()]);
    }

    #[test]
    fn extract_skips_target_dir() {
        let diff = "--- a/target/debug/build.rs\n+++ b/target/debug/build.rs";
        let files = extract_changed_rust_files(diff);
        assert!(files.is_empty());
    }

    #[test]
    fn empty_config_returns_none() {
        let config = StaticAnalysisConfig::default();
        let result = collect_static_context("some diff", &config);
        assert!(result.is_none());
    }

    #[test]
    fn empty_diff_returns_none_for_auto_clippy() {
        let config = StaticAnalysisConfig {
            auto_clippy: true,
            clippy_output_file: None,
        };
        let result = collect_static_context("", &config);
        assert!(result.is_none());
    }
}
