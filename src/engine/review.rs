use anyhow::Result;
use tracing::{debug, instrument};

use crate::config::schema::Config;
use crate::engine::llm;
use crate::engine::types::{LLMConfig, ReviewIssue, ReviewResponse, ScanResponse};

/// Run a code review on the given diff string.
///
/// Builds the prompt from the diff + config focus/rules, calls the LLM,
/// parses the response, and sets `should_block` based on the config's
/// `hook.min_severity` threshold.
#[instrument(skip_all)]
pub async fn review_diff(
    config: &Config,
    llm_config: &LLMConfig,
    diff: &str,
) -> Result<ReviewResponse> {
    review_diff_inner(config, llm_config, diff, false).await
}

/// Run a code review on the given diff string with optional streaming.
///
/// When `stream` is true, LLM tokens are printed to stdout in real-time.
#[instrument(skip_all)]
pub async fn review_diff_with_stream(
    config: &Config,
    llm_config: &LLMConfig,
    diff: &str,
    stream: bool,
) -> Result<ReviewResponse> {
    review_diff_inner(config, llm_config, diff, stream).await
}

async fn review_diff_inner(
    config: &Config,
    llm_config: &LLMConfig,
    diff: &str,
    stream: bool,
) -> Result<ReviewResponse> {
    debug!(
        diff_len = diff.len(),
        stream = stream,
        "starting diff review"
    );

    if diff.trim().is_empty() {
        return Ok(ReviewResponse {
            issues: vec![],
            summary: "No changes to review.".to_string(),
            tokens_used: None,
            should_block: false,
        });
    }

    let mut response = if stream {
        llm::review_diff_stream(llm_config, diff, &config.focus, &config.rules).await?
    } else {
        llm::review_diff(llm_config, diff, &config.focus, &config.rules).await?
    };

    // Apply ignore rules: filter out issues matching ignored patterns
    response.issues = apply_ignore_rules(response.issues, &config.ignore.rules);

    // Calculate should_block based on min_severity
    let min_severity = config.hook.min_severity_level();
    response.should_block = response
        .issues
        .iter()
        .any(|issue| issue.severity >= min_severity);

    debug!(
        issues = response.issues.len(),
        should_block = response.should_block,
        "review complete"
    );

    Ok(response)
}

/// Scan a full project or set of files.
///
/// Walks the directory, filters files, batches them, calls the LLM,
/// and aggregates results.
#[instrument(skip_all)]
pub async fn scan_project(
    config: &Config,
    llm_config: &LLMConfig,
    files_content: &str,
    files_count: usize,
    lines_count: usize,
) -> Result<ScanResponse> {
    debug!(
        files = files_count,
        lines = lines_count,
        "starting project scan"
    );

    if files_content.trim().is_empty() {
        return Ok(ScanResponse {
            issues: vec![],
            files_scanned: 0,
            lines_scanned: 0,
            summary: "No files to scan.".to_string(),
            tokens_used: None,
            should_block: false,
        });
    }

    let (issues, summary, tokens_used) =
        llm::scan_files(llm_config, files_content, &config.focus, &config.rules).await?;

    // Apply ignore rules
    let issues = apply_ignore_rules(issues, &config.ignore.rules);

    // Calculate should_block
    let min_severity = config.hook.min_severity_level();
    let should_block = issues.iter().any(|issue| issue.severity >= min_severity);

    let default_summary = format!(
        "Scanned {} files, found {} issues.",
        files_count,
        issues.len()
    );
    Ok(ScanResponse {
        issues,
        files_scanned: files_count,
        lines_scanned: lines_count,
        summary: summary.unwrap_or(default_summary),
        tokens_used,
        should_block,
    })
}

/// Filter out issues whose issue_type matches any ignored rule pattern.
fn apply_ignore_rules(mut issues: Vec<ReviewIssue>, ignore_rules: &[String]) -> Vec<ReviewIssue> {
    if ignore_rules.is_empty() {
        return issues;
    }

    issues.retain(|issue| {
        !ignore_rules.iter().any(|pattern| {
            let pattern_lower = pattern.to_lowercase();
            let issue_type_lower = issue
                .issue_type
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default()
                .to_lowercase();
            issue_type_lower.contains(&pattern_lower)
                || issue.title.to_lowercase().contains(&pattern_lower)
        })
    });

    issues
}
