use anyhow::{Context, Result};
use colored::Colorize;
use tracing::debug;

use crate::config::schema::Config;
use crate::engine::Severity;
use crate::formatters::{OutputFormat, formatter_for};
use crate::git;
use crate::progress::{ProgressReporter, TokenInfo, diff_stats};

/// Exit codes for the review command.
pub const EXIT_OK: i32 = 0;
pub const EXIT_BLOCKED: i32 = 2;

/// Review command options.
#[allow(clippy::struct_excessive_bools)]
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
    /// Disable review caching.
    pub no_cache: bool,
    /// CI mode: skip diff size limit, hard gate on any findings.
    pub ci: bool,
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
#[allow(clippy::cast_possible_truncation)]
pub async fn execute_review(
    config: &Config,
    llm_config: &crate::engine::LLMConfig,
    opts: &ReviewOptions,
    format: OutputFormat,
    progress: &ProgressReporter,
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

    // 2. Emit parsing_diff event if progress enabled
    if progress.is_enabled() {
        let (files_changed, lines_changed) = diff_stats(&diff);
        progress.parsing_diff(files_changed, lines_changed);
    }

    // 3. Validate size (skip in CI mode)
    let max_size = opts.max_diff_size.unwrap_or(config.hook.max_diff_size);
    if !opts.ci && diff.len() > max_size {
        if config.hook.on_violation == "disallow" {
            // Return blocked result instead of error
            return Ok(ReviewResult {
                exit_code: EXIT_BLOCKED,
                output: format!(
                    "{}\n",
                    format!(
                        "❌ Diff too large ({} chars, max {}). Commit blocked. \
                         Use --base to review a specific branch, increase hook.max_diff_size, \
                         or run: git commit --no-verify",
                        diff.len(), max_size
                    )
                    .red()
                    .bold(),
                ),
            });
        }
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

    // 4. Emit calling_llm event
    if progress.is_enabled() {
        progress.calling_llm(&llm_config.provider, &llm_config.model);
    }

    let llm_start = std::time::Instant::now();

    // 5. Call the LLM engine
    let response = match crate::engine::review::review_diff_with_cache(
        config,
        llm_config,
        &diff,
        opts.stream,
        !opts.no_cache,
        opts.quiet,
    )
    .await
    {
        Ok(resp) => {
            if progress.is_enabled() {
                // SAFETY: elapsed ms fits u64 for any reasonable review duration
                let duration_ms = llm_start.elapsed().as_millis() as u64;
                let tokens = resp
                    .tokens_used
                    .as_ref()
                    .map_or_else(TokenInfo::zero, TokenInfo::from_usage);
                progress.llm_response(&tokens, duration_ms);
            }
            resp
        }
        Err(e) => {
            if progress.is_enabled() {
                progress.error(&e.to_string(), "calling_llm");
            }
            return Err(e.into());
        }
    };

    // 6. Filter by severity if specified
    let min_severity = if let Some(ref sev) = opts.severity {
        Severity::from_str_lossy(sev)
    } else {
        config.hook.min_severity_level()
    };

    let mut filtered_response = response.clone();
    // Keep issues at or above min_severity (Critical=worst, Info=lowest in Ord but
    // Ord order is Critical(0) < Major(1) < Minor(2) < Info(3), so we invert: keep
    // where severity Ord value <= min_severity Ord value).
    filtered_response
        .issues
        .retain(|i| i.severity <= min_severity);

    // 7. Format output
    let formatter = formatter_for(format);
    let output = formatter.format_review(&filtered_response)?;

    // 8. Return exit code
    let exit_code = if opts.ci {
        // CI mode: hard gate — any finding blocks
        if !filtered_response.issues.is_empty() {
            EXIT_BLOCKED
        } else {
            EXIT_OK
        }
    } else if response.should_block && config.hook.mode == "block" {
        EXIT_BLOCKED
    } else {
        EXIT_OK
    };

    // 9. Emit complete event
    if progress.is_enabled() {
        let tokens = response
            .tokens_used
            .as_ref()
            .map_or_else(TokenInfo::zero, TokenInfo::from_usage);
        progress.complete(
            filtered_response.issues.len(),
            exit_code == EXIT_BLOCKED,
            &tokens,
        );
    }

    Ok(ReviewResult { exit_code, output })
}

/// Get the diff based on the provided options.
fn get_diff(opts: &ReviewOptions, _config: &Config) -> Result<String> {
    if let Some(ref diff_file) = opts.diff_file {
        let path = std::path::Path::new(diff_file);
        if !path.exists() {
            anyhow::bail!("diff file not found: {diff_file}");
        }
        return std::fs::read_to_string(path)
            .with_context(|| format!("failed to read diff file: {diff_file}"));
    }
    if let Some(ref commit_ref) = opts.commit {
        return git::get_commit_diff(commit_ref).map_err(Into::into);
    }
    if opts.staged {
        git::get_staged_diff().map_err(Into::into)
    } else if opts.unpushed {
        git::get_unpushed_diff().map_err(Into::into)
    } else if let Some(ref base) = opts.base {
        git::get_branch_diff(base).map_err(Into::into)
    } else if opts.unstaged {
        git::get_unstaged_diff().map_err(Into::into)
    } else {
        // Default: try staged first, fall back to unpushed, then unstaged
        match git::get_staged_diff() {
            Ok(d) if !d.trim().is_empty() => Ok(d),
            _ => match git::get_unpushed_diff() {
                Ok(d) if !d.trim().is_empty() => Ok(d),
                _ => git::get_unstaged_diff().map_err(Into::into),
            },
        }
    }
}
