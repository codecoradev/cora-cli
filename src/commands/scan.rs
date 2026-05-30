use std::path::Path;

use anyhow::Result;
use colored::Colorize;
use tracing::debug;

use crate::config::schema::Config;
use crate::engine::scanner::{batch_files, format_batch_for_prompt, walk_project};
use crate::formatters::{formatter_for, OutputFormat};

/// Scan command options.
pub struct ScanOptions {
    /// Root directory to scan.
    pub path: Option<String>,
    /// Include glob patterns.
    pub include: Vec<String>,
    /// Exclude glob patterns.
    pub exclude: Vec<String>,
    /// Additional file extensions to include.
    pub extensions: Vec<String>,
}

/// Execute the scan command.
///
/// Walks the project directory, filters files, batches them, calls the LLM,
/// and formats the output.
pub async fn execute_scan(
    config: &Config,
    llm_config: &crate::engine::LLMConfig,
    opts: &ScanOptions,
    format: OutputFormat,
) -> Result<i32> {
    let root = match &opts.path {
        Some(p) => Path::new(p).to_path_buf(),
        None => std::env::current_dir()?,
    };

    if !root.is_dir() {
        anyhow::bail!("scan path '{}' is not a directory", root.display());
    }

    // Merge include/exclude with config ignore patterns
    let include = opts.include.clone();
    let mut exclude = config.ignore.files.clone();
    exclude.extend(opts.exclude.clone());

    debug!(root = %root.display(), "starting scan");

    // 1. Walk and collect files
    let files = walk_project(&root, &include, &exclude, &opts.extensions)?;
    if files.is_empty() {
        println!("{}", "No files to scan.".yellow());
        return Ok(0);
    }

    println!(
        "{} {} files to review…",
        "🔍".to_string(),
        files.len().to_string().cyan(),
    );

    // 2. Calculate total lines
    let total_lines: usize = files.iter().map(|f| f.lines).sum();

    // 3. Batch files
    let batches = batch_files(&files, 60_000, 20);
    debug!(batches = batches.len(), "batched files");

    // 4. Process batches and collect issues
    let mut all_issues = Vec::new();
    let mut total_tokens = None;

    for (batch_idx, batch) in batches.iter().enumerate() {
        let files_content = format_batch_for_prompt(batch);
        let batch_label = if batches.len() > 1 {
            format!(" (batch {}/{})", batch_idx + 1, batches.len())
        } else {
            String::new()
        };

        println!("  Reviewing{}…", batch_label);

        let (issues, _summary, tokens) =
            crate::engine::llm::scan_files(llm_config, &files_content, &config.focus, &config.rules)
                .await?;

        all_issues.extend(issues);
        if tokens.is_some() {
            total_tokens = tokens;
        }
    }

    // 5. Build response and format
    let issue_count = all_issues.len();
    let min_severity = config.hook.min_severity_level();
    let should_block = all_issues.iter().any(|i| i.severity >= min_severity);

    let response = crate::engine::ScanResponse {
        issues: all_issues,
        files_scanned: files.len(),
        lines_scanned: total_lines,
        summary: format!(
            "Scanned {} files ({} lines), found {} issues.",
            files.len(),
            total_lines,
            issue_count
        ),
        tokens_used: total_tokens,
        should_block,
    };

    let formatter = formatter_for(format);
    let output = formatter.format_scan(&response)?;
    println!("{}", output);

    if response.should_block && config.hook.mode == "block" {
        Ok(2)
    } else {
        Ok(0)
    }
}
