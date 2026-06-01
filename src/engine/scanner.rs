use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Result;
use glob::Pattern;
use indicatif::{ProgressBar, ProgressStyle};
use tracing::debug;
use walkdir::WalkDir;

/// A file to be scanned, with its relative path and content.
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// Relative path from the project root.
    pub path: String,
    /// The file content.
    pub content: String,
    /// Number of lines.
    pub lines: usize,
}

/// Maximum characters per batch to avoid exceeding LLM token limits.
const MAX_BATCH_CHARS: usize = 60_000;

/// Maximum files per batch.
const MAX_FILES_PER_BATCH: usize = 20;

/// File extensions to include in scans by default (source code).
const DEFAULT_EXTENSIONS: &[&str] = &[
    "rs", "py", "js", "ts", "tsx", "jsx", "go", "java", "kt", "rb", "c", "cpp", "h", "hpp", "cs",
    "php", "swift", "scala", "vue", "svelte", "sh", "bash", "zsh", "ps1", "toml", "yaml", "yml",
    "json", "sql", "graphql", "proto", "md", "rst", "txt", "html", "css", "scss", "less",
];

/// Walk a project directory, respecting .gitignore, and collect scannable files.
///
/// Uses `include` and `exclude` glob patterns to filter results.
pub fn walk_project(
    root: &Path,
    include_patterns: &[String],
    exclude_patterns: &[String],
    extra_extensions: &[String],
) -> Result<Vec<FileEntry>> {
    debug!(
        root = %root.display(),
        "walking project directory"
    );

    // Build extension set
    let mut extensions: BTreeSet<String> =
        DEFAULT_EXTENSIONS.iter().map(|s| s.to_string()).collect();
    for ext in extra_extensions {
        extensions.insert(ext.trim_start_matches('.').to_lowercase());
    }

    let include_globs: Vec<Pattern> = include_patterns
        .iter()
        .filter_map(|p| Pattern::new(p).ok())
        .collect();

    let exclude_globs: Vec<Pattern> = exclude_patterns
        .iter()
        .filter_map(|p| Pattern::new(p).ok())
        .collect();

    // Load .gitignore if present
    let gitignore = load_gitignore(root);

    let mut entries = Vec::new();

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
    );
    spinner.set_message("Scanning files…");

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            // Skip hidden files/directories
            e.file_name()
                .to_str()
                .map(|s| !s.starts_with('.'))
                .unwrap_or(true)
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        let relative = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        // Check gitignore
        if is_ignored_by_gitignore(&relative, &gitignore) {
            continue;
        }

        // Check exclude patterns
        if exclude_globs.iter().any(|g| g.matches(&relative)) {
            continue;
        }

        // Check include patterns (if any specified)
        let has_include = !include_globs.is_empty();
        if has_include && !include_globs.iter().any(|g| g.matches(&relative)) {
            continue;
        }

        // Check extension
        let has_extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| extensions.contains(&e.to_lowercase()));
        if has_extension == Some(false) {
            continue;
        }

        // Read file content (skip binary / unreadable)
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                debug!(file = %relative, error = %e, "skipping unreadable file");
                continue;
            }
        };

        // Skip empty files
        if content.trim().is_empty() {
            continue;
        }

        // Skip very large files (> 200KB)
        if content.len() > 200_000 {
            debug!(file = %relative, "skipping large file");
            continue;
        }

        let lines = content.lines().count();
        entries.push(FileEntry {
            path: relative,
            content,
            lines,
        });
    }

    spinner.finish_and_clear();
    debug!(files = entries.len(), "found scannable files");

    Ok(entries)
}

/// Split files into batches to avoid exceeding LLM token limits.
pub fn batch_files(files: &[FileEntry], max_chars: usize, max_files: usize) -> Vec<Vec<FileEntry>> {
    let mut batches = Vec::new();
    let mut current_batch = Vec::new();
    let mut current_size: usize = 0;

    for file in files {
        let file_size = file.content.len() + file.path.len() + 20; // + header overhead

        if (current_batch.len() >= max_files || current_size + file_size > max_chars)
            && !current_batch.is_empty()
        {
            batches.push(std::mem::take(&mut current_batch));
            current_size = 0;
        }

        current_size += file_size;
        current_batch.push(file.clone());
    }

    if !current_batch.is_empty() {
        batches.push(current_batch);
    }

    batches
}

/// Format a batch of files for the LLM prompt.
pub fn format_batch_for_prompt(files: &[FileEntry]) -> String {
    let mut output = String::new();

    for file in files {
        output.push_str(&format!("=== {} ===\n", file.path));
        for (i, line) in file.content.lines().enumerate() {
            output.push_str(&format!("{:>5} | {}\n", i + 1, line));
        }
        output.push('\n');
    }

    output
}

/// Load .gitignore patterns from a project root.
fn load_gitignore(root: &Path) -> Vec<String> {
    let gitignore_path = root.join(".gitignore");
    if !gitignore_path.is_file() {
        return Vec::new();
    }

    std::fs::read_to_string(&gitignore_path)
        .unwrap_or_default()
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with('#')
        })
        .map(|s| s.to_string())
        .collect()
}

/// Simple check if a path matches any gitignore pattern.
fn is_ignored_by_gitignore(path: &str, patterns: &[String]) -> bool {
    for pattern in patterns {
        let p = pattern.trim();
        if p.is_empty() {
            continue;
        }
        // Simple glob matching
        if let Ok(glob) = Pattern::new(&format!("**/{p}")) {
            if glob.matches(path) {
                return true;
            }
        }
        // Direct prefix match for directory patterns
        if p.ends_with('/') && path.starts_with(p) {
            return true;
        }
        // Exact file match
        if path == p || path.ends_with(&format!("/{p}")) {
            return true;
        }
    }
    false
}
