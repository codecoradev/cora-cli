use anyhow::Result;
use colored::Colorize;
use tracing::debug;

use crate::config::schema::Config;
use crate::formatters::{formatter_for, OutputFormat};
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
    /// Review unstaged changes.
    pub unstaged: bool,
    /// Maximum diff size before refusing (0 = use config default).
    pub max_diff_size: Option<usize>,
}

/// Execute the review command.
///
/// Gets the diff, validates its size, calls the LLM engine, formats output,
/// and returns the appropriate exit code.
pub async fn execute_review(
    config: &Config,
    llm_config: &crate::engine::LLMConfig,
    opts: &ReviewOptions,
    format: OutputFormat,
) -> Result<i32> {
    // 1. Get the diff
    let diff = get_diff(opts, config)?;

    if diff.trim().is_empty() {
        println!("{}", "No changes to review.".yellow());
        return Ok(EXIT_OK);
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

    debug!(diff_len = diff.len(), "running review");

    // 3. Call the LLM engine
    let response = crate::engine::review::review_diff(config, llm_config, &diff).await?;

    // 4. Format and print output
    let formatter = formatter_for(format);
    let output = formatter.format_review(&response)?;
    println!("{}", output);

    // 5. Return exit code
    if response.should_block && config.hook.mode == "block" {
        Ok(EXIT_BLOCKED)
    } else {
        Ok(EXIT_OK)
    }
}

/// Get the diff based on the provided options.
fn get_diff(opts: &ReviewOptions, _config: &Config) -> Result<String> {
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
