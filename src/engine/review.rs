use crate::error::CoraError;
use tracing::{debug, instrument};

use crate::config::schema::Config;
use crate::engine::llm;
use crate::engine::types::{LLMConfig, ReviewIssue, ReviewResponse};

/// Load a custom system prompt from a file path.
/// Returns the file content, or None if the file doesn't exist, can't be read,
/// or is outside the project root (path traversal guard).
fn load_system_prompt_file(path: &str) -> Option<String> {
    let Ok(canonical) = std::fs::canonicalize(path) else {
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
) -> std::result::Result<ReviewResponse, CoraError> {
    review_diff_inner(config, llm_config, diff, stream, use_cache, quiet).await
}

async fn review_diff_inner(
    config: &Config,
    llm_config: &LLMConfig,
    diff: &str,
    stream: bool,
    use_cache: bool,
    quiet: bool,
) -> std::result::Result<ReviewResponse, CoraError> {
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

    // Collect static analysis context (clippy output, etc.)
    let static_context =
        crate::engine::static_analysis::collect_static_context(diff, &config.static_analysis);

    // Parse diff and run rule engine
    let diff_chunks = crate::engine::diff_parser::parse_diff(diff);
    let rule_findings = crate::engine::rules::run_rules(&diff_chunks, &config.rules_config);

    // Run deterministic secrets pre-scan
    let secrets_findings = crate::engine::secrets_scanner::scan_secrets(
        &diff_chunks,
        config.rules_config.max_findings,
    );

    // Run deterministic security pattern scan (weak crypto, injection, etc.)
    let security_findings = crate::engine::security_scanner::scan_security(
        &diff_chunks,
        config.rules_config.max_findings,
    );

    let rule_context = crate::engine::rules::format_rule_context(&rule_findings);
    let secrets_context = crate::engine::rules::format_rule_context(&secrets_findings);
    let security_context = crate::engine::rules::format_rule_context(&security_findings);
    // Keep a clone for merging after LLM (rule_findings may be consumed in error fallback)
    let rule_findings_clone = rule_findings.clone();
    let secrets_findings_clone = secrets_findings.clone();
    let security_findings_clone = security_findings.clone();

    // Combine static analysis + rule engine + secrets + security findings for LLM prompt
    let combined_context = match (
        static_context.as_deref(),
        rule_context.as_str(),
        secrets_context.as_str(),
        security_context.as_str(),
    ) {
        (Some(sa), rc, sc, sec) if !rc.is_empty() && !sc.is_empty() && !sec.is_empty() => {
            Some(format!("{sa}\n\n{rc}\n\n{sc}\n\n{sec}"))
        }
        (Some(sa), rc, sc, _) if !rc.is_empty() && !sc.is_empty() => {
            Some(format!("{sa}\n\n{rc}\n\n{sc}"))
        }
        (Some(sa), rc, _, sec) if !rc.is_empty() && !sec.is_empty() => {
            Some(format!("{sa}\n\n{rc}\n\n{sec}"))
        }
        (Some(sa), _, sc, sec) if !sc.is_empty() && !sec.is_empty() => {
            Some(format!("{sa}\n\n{sc}\n\n{sec}"))
        }
        (Some(sa), rc, _, _) if !rc.is_empty() => Some(format!("{sa}\n\n{rc}")),
        (Some(sa), _, sc, _) if !sc.is_empty() => Some(format!("{sa}\n\n{sc}")),
        (Some(sa), _, _, sec) if !sec.is_empty() => Some(format!("{sa}\n\n{sec}")),
        (Some(sa), _, _, _) => Some(sa.to_string()),
        (_, rc, sc, sec) if !rc.is_empty() && !sc.is_empty() && !sec.is_empty() => {
            Some(format!("{rc}\n\n{sc}\n\n{sec}"))
        }
        (_, rc, sc, _) if !rc.is_empty() && !sc.is_empty() => Some(format!("{rc}\n\n{sc}")),
        (_, rc, _, sec) if !rc.is_empty() && !sec.is_empty() => Some(format!("{rc}\n\n{sec}")),
        (_, _, sc, sec) if !sc.is_empty() && !sec.is_empty() => Some(format!("{sc}\n\n{sec}")),
        (_, rc, _, _) if !rc.is_empty() => Some(rc.to_string()),
        (_, _, sc, _) if !sc.is_empty() => Some(sc.to_string()),
        (_, _, _, sec) if !sec.is_empty() => Some(sec.to_string()),
        _ => None,
    };

    // Build context chain (cross-file dependency extraction)
    let context_chain = crate::engine::context::build_context_chain(
        &diff_chunks,
        &config.context_chain,
        std::env::current_dir().unwrap_or_default().as_path(),
        &config.ignore.rules,
    );

    let final_context = if !context_chain.text.is_empty() {
        match combined_context {
            Some(ctx) => Some(format!(
                "{ctx}\n\n## Cross-file Context\n{context_chain_text}",
                context_chain_text = context_chain.text
            )),
            None => Some(format!("## Cross-file Context\n{}", context_chain.text)),
        }
    } else {
        combined_context
    };

    // Inject language-specific context (reuses parsed diff_chunks)
    let lang_context =
        crate::engine::language_analyzer::build_language_context_from_chunks(&diff_chunks);
    let final_context = if !lang_context.is_empty() {
        match final_context {
            Some(ctx) => Some(format!("{lang_context}\n\n{ctx}")),
            None => Some(lang_context),
        }
    } else {
        final_context
    };

    // Inject profile instructions into the context
    let final_context = match (&config.profile, final_context) {
        (Some(profile), Some(ctx)) => {
            let profile_prompt = crate::engine::profiles::build_profile_prompt(profile);
            Some(format!("## Quality Profile\n{profile_prompt}\n\n{ctx}"))
        }
        (Some(profile), None) => {
            let profile_prompt = crate::engine::profiles::build_profile_prompt(profile);
            Some(format!("## Quality Profile\n{profile_prompt}"))
        }
        (None, ctx) => ctx,
    };

    // Call LLM — but preserve deterministic rule findings even on LLM failure
    let llm_result: Result<ReviewResponse, CoraError> = if stream {
        llm::review_diff_stream(
            llm_config,
            diff,
            &config.focus,
            &config.rules,
            &config.response_format,
            review_prompt.as_deref(),
            final_context.as_deref(),
        )
        .await
    } else {
        llm::review_diff(
            llm_config,
            diff,
            &config.focus,
            &config.rules,
            &config.response_format,
            review_prompt.as_deref(),
            quiet,
            final_context.as_deref(),
        )
        .await
    };

    let mut response = match llm_result {
        Ok(resp) => resp,
        Err(e) => {
            // LLM failed — return deterministic findings only (don't silently swallow them)
            if !rule_findings.is_empty() || !secrets_findings.is_empty() {
                let n_rules = rule_findings.len();
                let n_secrets = secrets_findings.len();
                debug!(
                    error = %e,
                    rule_findings = n_rules,
                    secrets_findings = n_secrets,
                    "LLM call failed, returning deterministic findings only"
                );
                let mut all_deterministic =
                    crate::engine::rules::merge_rule_findings(vec![], rule_findings);
                all_deterministic =
                    crate::engine::rules::merge_rule_findings(all_deterministic, secrets_findings);
                all_deterministic =
                    crate::engine::rules::merge_rule_findings(all_deterministic, security_findings);
                let mut fallback = ReviewResponse {
                    issues: all_deterministic,
                    summary: format!(
                        "LLM review failed: {e}. Showing {n_rules} rule + {n_secrets} secrets findings."
                    ),
                    tokens_used: None,
                    should_block: false,
                };
                fallback.issues = apply_ignore_rules(fallback.issues, &config.ignore.rules);
                let min_sev = config.hook.min_severity_level();
                fallback.should_block = fallback
                    .issues
                    .iter()
                    .any(|issue| issue.severity <= min_sev);
                return Ok(fallback);
            }
            return Err(e);
        }
    };

    // Merge rule findings + secrets findings + security findings with LLM issues
    if !rule_findings_clone.is_empty() {
        response.issues =
            crate::engine::rules::merge_rule_findings(response.issues, rule_findings_clone);
    }
    if !secrets_findings_clone.is_empty() {
        response.issues =
            crate::engine::rules::merge_rule_findings(response.issues, secrets_findings_clone);
    }
    if !security_findings_clone.is_empty() {
        response.issues =
            crate::engine::rules::merge_rule_findings(response.issues, security_findings_clone);
    }

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
            let issue_type_lower = issue.issue_type.clone().unwrap_or_default().to_lowercase();
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
