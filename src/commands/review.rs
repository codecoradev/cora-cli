use anyhow::{Context, Result};
use colored::Colorize;
use tracing::debug;

use crate::config::schema::Config;
use crate::engine::Severity;
use crate::formatters::{OutputFormat, formatter_for};
use crate::git;

/// Exit codes for the review command.
pub const EXIT_OK: i32 = 0;
pub const EXIT_ERROR: i32 = 1;
pub const EXIT_BLOCKED: i32 = 2;

/// Review command options.
pub struct ReviewOptions {
    /// Review staged changes.
    pub staged: bool,
    /// Review unpushed changes.
    pub unpushed: bool,
    /// Review diff against a base branch.
    pub base: Option<String>,
    /// Review diff from a git commit reference (e.g. HEAD, HEAD~3..HEAD, abc123).
    pub commit: Option<String>,
    /// Read diff from a file instead of git.
    pub diff_file: Option<String>,
    /// Review unstaged changes.
    pub unstaged: bool,
    /// Maximum diff size before refusing (0 = use config default).
    pub max_diff_size: Option<usize>,
    /// Stream LLM response tokens to stdout in real-time.
    pub stream: bool,
    /// Suppress all output except the formatted review result.
    pub quiet: bool,
    /// Minimum severity level for filtering issues.
    pub severity: Option<String>,
}

/// Result of a review command execution.
pub struct ReviewResult {
    /// Exit code (0 = ok, 2 = blocked).
    pub exit_code: i32,
    /// The formatted output string.
    pub output: String,
}

/// Execute the review command.
///
/// Gets the diff, validates its size, calls the LLM engine, formats output,
/// and returns the appropriate exit code along with the formatted output.
pub async fn execute_review(
    config: &Config,
    llm_config: &crate::engine::LLMConfig,
    opts: &ReviewOptions,
    format: OutputFormat,
) -> Result<ReviewResult> {
    // 1. Get the diff
    let diff = get_diff(opts, config)?;

    if diff.trim().is_empty() {
        if opts.quiet {
            return Ok(ReviewResult {
                exit_code: EXIT_OK,
                output: String::new(),
            });
        }
        let output = format!("{}\n", "No changes to review.".yellow());
        return Ok(ReviewResult {
            exit_code: EXIT_OK,
            output,
        });
    }

    // 2. Validate size
    let max_size = opts.max_diff_size.unwrap_or(config.hook.max_diff_size);
    if diff.len() > max_size {
        anyhow::bail!(
            "Diff too large ({} chars, max {}). Use --base to review a specific branch, or increase hook.max_diff_size.",
            diff.len(),
            max_size
        );
    }

    debug!(
        diff_len = diff.len(),
        stream = opts.stream,
        "running review"
    );

    // 3. Call the LLM engine
    let response =
        crate::engine::review::review_diff_with_stream(config, llm_config, &diff, opts.stream)
            .await?;

    // 4. Filter by severity if specified
    let min_severity = if let Some(ref sev) = opts.severity {
        Severity::from_str_lossy(sev)
    } else {
        config.hook.min_severity_level()
    };

    let mut filtered_response = response.clone();
    filtered_response.issues.retain(|i| i.severity <= min_severity);

    // 5. Format output
    let formatter = formatter_for(format);
    let output = formatter.format_review(&filtered_response)?;

    // 6. Return exit code
    let exit_code = if response.should_block && config.hook.mode == "block" {
        EXIT_BLOCKED
    } else {
        EXIT_OK
    };

    Ok(ReviewResult { exit_code, output })
}

/// Get the diff based on the provided options.
fn get_diff(opts: &ReviewOptions, _config: &Config) -> Result<String> {
    if let Some(ref diff_file) = opts.diff_file {
        let path = std::path::Path::new(diff_file);
        if !path.exists() {
            anyhow::bail!("diff file not found: {}", diff_file);
        }
        return std::fs::read_to_string(path)
            .with_context(|| format!("failed to read diff file: {}", diff_file));
    }
    if let Some(ref commit_ref) = opts.commit {
        return git::get_commit_diff(commit_ref);
    }
    if opts.staged {
        git::get_staged_diff()
    } else if opts.unpushed {
        git::get_unpushed_diff()
    } else if let Some(ref base) = opts.base {
        git::get_branch_diff(base)
    } else if opts.unstaged {
        git::get_unstaged_diff()
    } else {
        // Default: try staged first, fall back to unpushed, then unstaged
        match git::get_staged_diff() {
            Ok(d) if !d.trim().is_empty() => Ok(d),
            _ => match git::get_unpushed_diff() {
                Ok(d) if !d.trim().is_empty() => Ok(d),
                _ => git::get_unstaged_diff(),
            },
        }
    }
}
