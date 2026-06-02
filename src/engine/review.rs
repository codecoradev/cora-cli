use anyhow::Result;
use tracing::{debug, instrument};

use crate::config::schema::Config;
use crate::engine::llm;
use crate::engine::types::{LLMConfig, ReviewIssue, ReviewResponse};

/// Load a custom system prompt from a file path.
/// Returns the file content, or None if the file doesn't exist, can't be read,
/// or is outside the project root (path traversal guard).
fn load_system_prompt_file(path: &str) -> Option<String> {
    let canonical = if let Ok(p) = std::fs::canonicalize(path) { p } else {
        tracing::debug!(path = path, "system_prompt_file does not exist");
        return None;
    };
    let project_root = std::env::current_dir().ok()?;
    let project_root = std::fs::canonicalize(&project_root).ok()?;

    if !canonical.starts_with(&project_root) {
        tracing::warn!(
            path = path,
            "system_prompt_file is outside project root, ignoring (potential path traversal)"
        );
        return None;
    }

    match std::fs::read_to_string(&canonical) {
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
pub fn resolve_system_prompt(inline: Option<&str>, file_path: Option<&str>) -> Option<String> {
    if let Some(prompt) = inline {
        Some(prompt.to_string())
    } else if let Some(path) = file_path {
        load_system_prompt_file(path)
    } else {
        None
    }
}

/// Run a code review on the given diff string with optional streaming and cache control.
///
/// When `stream` is true, LLM tokens are printed to stdout in real-time.
/// When `use_cache` is false, the cache is bypassed.
#[instrument(skip_all)]
pub async fn review_diff_with_cache(
    config: &Config,
    llm_config: &LLMConfig,
    diff: &str,
    stream: bool,
    use_cache: bool,
    quiet: bool,
) -> Result<ReviewResponse> {
    review_diff_inner(config, llm_config, diff, stream, use_cache, quiet).await
}

async fn review_diff_inner(
    config: &Config,
    llm_config: &LLMConfig,
    diff: &str,
    stream: bool,
    use_cache: bool,
    quiet: bool,
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

    // Check cache before calling LLM
    if use_cache {
        if let Some(cached) = crate::engine::cache::get_cached_review(
            diff,
            &llm_config.model,
            llm_config.temperature,
            config.cache_ttl,
        ) {
            debug!("returning cached review response");
            return Ok(cached);
        }
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
            quiet,
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

    // Save fully-processed response to cache (after filtering)
    if use_cache {
        if let Err(e) = crate::engine::cache::save_cached_review(
            diff,
            &llm_config.model,
            llm_config.temperature,
            &response,
        ) {
            debug!("failed to save review to cache: {}", e);
        }
    }

    Ok(response)
}

/// Filter out issues whose `issue_type` matches any ignored rule pattern.
fn apply_ignore_rules(mut issues: Vec<ReviewIssue>, ignore_rules: &[String]) -> Vec<ReviewIssue> {
    if ignore_rules.is_empty() {
        return issues;
    }

    issues.retain(|issue| {
        !ignore_rules.iter().any(|pattern| {
            let pattern_lower = pattern.to_lowercase();
            let issue_type_lower = issue
                .issue_type.clone()
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
        // Use a file within the project root so the path traversal guard allows it
        let test_file = std::path::PathBuf::from(".cora-test-prompt.tmp");
        std::fs::write(&test_file, "file prompt content").unwrap();
        let result = resolve_system_prompt(None, Some(".cora-test-prompt.tmp"));
        assert_eq!(result.as_deref(), Some("file prompt content"));
        let _ = std::fs::remove_file(&test_file);
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

    #[test]
    fn reject_path_traversal_outside_project() {
        // /etc/passwd exists but is outside project root — should be rejected
        let result = resolve_system_prompt(None, Some("/etc/passwd"));
        assert!(
            result.is_none(),
            "system_prompt_file outside project root should be rejected"
        );
    }
}
