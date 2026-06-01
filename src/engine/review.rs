use anyhow::Result;
use tracing::{debug, instrument};

use crate::config::schema::Config;
use crate::engine::llm;
use crate::engine::types::{LLMConfig, ReviewIssue, ReviewResponse, ScanResponse};

/// Load a custom system prompt from a file path.
/// Returns the file content, or None if the file doesn't exist or can't be read.
fn load_system_prompt_file(path: &str) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(content) => Some(content),
        Err(e) => {
            tracing::warn!(
                path = path,
                error = %e,
                "failed to read system_prompt_file, using default prompt"
            );
            None
        }
    }
}

/// Resolve the effective system prompt: inline override > file override > None (use default).
pub fn resolve_system_prompt(
    inline: Option<&str>,
    file_path: Option<&str>,
) -> Option<String> {
    if let Some(prompt) = inline {
        Some(prompt.to_string())
    } else if let Some(path) = file_path {
        load_system_prompt_file(path)
    } else {
        None
    }
}

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

    // Extract valid file paths for post-parse filtering
    let valid_files = llm::extract_file_paths_from_diff(diff);

    // Resolve custom system prompt for review
    let review_prompt = resolve_system_prompt(
        config.review_system_prompt_override.as_deref(),
        config.review_system_prompt_file.as_deref(),
    );

    let mut response = if stream {
        llm::review_diff_stream(
            llm_config,
            diff,
            &config.focus,
            &config.rules,
            &config.response_format,
            review_prompt.as_deref(),
        )
        .await?
    } else {
        llm::review_diff(
            llm_config,
            diff,
            &config.focus,
            &config.rules,
            &config.response_format,
            review_prompt.as_deref(),
        )
        .await?
    };

    // Filter out issues with invalid file paths (hallucination guard)
    if !valid_files.is_empty() {
        let before = response.issues.len();
        response
            .issues
            .retain(|issue| is_valid_file_path(&issue.file, &valid_files));
        let filtered = before - response.issues.len();
        if filtered > 0 {
            debug!(
                filtered,
                remaining = response.issues.len(),
                "filtered issues with invalid file paths"
            );
        }
    }

    // Apply ignore rules: filter out issues matching ignored patterns
    response.issues = apply_ignore_rules(response.issues, &config.ignore.rules);

    // Calculate should_block based on min_severity
    let min_severity = config.hook.min_severity_level();
    // Ord order is Critical(0) < Major(1) < Minor(2) < Info(3), so "at or above
    // min_severity" means Ord value <= min_severity.
    response.should_block = response
        .issues
        .iter()
        .any(|issue| issue.severity <= min_severity);

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

    // Resolve custom system prompt for scan
    let scan_prompt = resolve_system_prompt(
        config.scan_system_prompt_override.as_deref(),
        config.scan_system_prompt_file.as_deref(),
    );

    let (issues, summary, tokens_used) =
        llm::scan_files(
            llm_config,
            files_content,
            &config.focus,
            &config.rules,
            &config.response_format,
            scan_prompt.as_deref(),
        )
        .await?;

    // Apply ignore rules
    let issues = apply_ignore_rules(issues, &config.ignore.rules);

    // Calculate should_block
    let min_severity = config.hook.min_severity_level();
    // Ord order: Critical(0) < Major(1) < Minor(2) < Info(3)
    let should_block = issues.iter().any(|issue| issue.severity <= min_severity);

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

/// Check if a file path from an LLM issue matches any of the valid diff file paths.
/// Uses exact match only — the LLM should report paths exactly as they appear in the diff.
fn is_valid_file_path(issue_file: &str, valid_files: &[String]) -> bool {
    valid_files.iter().any(|f| f == issue_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_prompt_inline_takes_priority() {
        let result = resolve_system_prompt(Some("inline prompt"), Some("file.md"));
        assert_eq!(result.as_deref(), Some("inline prompt"));
    }

    #[test]
    fn resolve_prompt_file_fallback() {
        let dir = std::env::temp_dir().join("cora_test_prompt");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("prompt.md");
        std::fs::write(&path, "file prompt content").unwrap();
        let result = resolve_system_prompt(None, Some(path.to_str().unwrap()));
        assert_eq!(result.as_deref(), Some("file prompt content"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn resolve_prompt_none_when_both_missing() {
        let result = resolve_system_prompt(None, None);
        assert!(result.is_none());
    }

    #[test]
    fn resolve_prompt_none_when_file_missing() {
        let result = resolve_system_prompt(None, Some("/nonexistent/prompt.md"));
        assert!(result.is_none());
    }
}
