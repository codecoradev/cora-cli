use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::debug;

use crate::engine::types::{
    LLMConfig, ReviewIssue, ReviewResponse, TokenUsage,
};

/// OpenAI-compatible chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// Request body for /chat/completions.
#[derive(Debug, Clone, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<Value>,
}

/// Response from /chat/completions.
#[derive(Debug, Clone, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
    usage: Option<Usage>,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Clone, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// System prompt for code review.
const REVIEW_SYSTEM_PROMPT: &str = r#"You are an expert code reviewer. Your job is to analyze code diffs and identify issues.

Focus on these categories:
- Security vulnerabilities (injections, auth issues, data exposure)
- Performance problems (inefficient algorithms, memory leaks, N+1 queries)
- Bugs (logic errors, off-by-one, null/undefined handling)
- Best practices (idiomatic code, error handling, naming)

For each issue found, return a JSON array of objects with these fields:
- "file": string — the file path
- "line": number or null — the approximate line number
- "severity": "critical" | "major" | "minor" | "info"
- "issue_type": string — category (security, performance, bugs, best-practice, style)
- "title": string — short description (max 100 chars)
- "body": string — detailed explanation
- "suggested_fix": string or null — optional fix suggestion

If no issues are found, return an empty array: []

Return ONLY the JSON array. No markdown, no explanation, just the JSON."#;

/// System prompt for full project scanning.
const SCAN_SYSTEM_PROMPT: &str = r#"You are an expert code reviewer performing a full project scan. Analyze the provided code files and identify issues.

Focus on these categories:
- Security vulnerabilities
- Performance problems
- Bugs and logic errors
- Best practices and code quality

For each issue found, return a JSON array of objects with these fields:
- "file": string — the file path
- "line": number or null — the approximate line number
- "severity": "critical" | "major" | "minor" | "info"
- "issue_type": string — category
- "title": string — short description (max 100 chars)
- "body": string — detailed explanation
- "suggested_fix": string or null — optional fix suggestion

Also include a "summary" string at the end after a "|||" separator:
[...JSON array...]|||Summary text here.

If no issues are found, return: []|||No issues found.

Return ONLY this format. No markdown."#;

/// Send a chat completion request to an OpenAI-compatible API.
async fn chat_completion(
    config: &LLMConfig,
    system_prompt: &str,
    user_message: &str,
    spinner: Option<&ProgressBar>,
) -> Result<String> {
    let client = reqwest::Client::new();

    let url = format!(
        "{}/chat/completions",
        config.base_url.trim_end_matches('/')
    );

    if let Some(sp) = spinner {
        sp.set_message(format!("Sending to {} ({})…", config.provider, config.model));
    }

    let request = ChatRequest {
        model: config.model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".into(),
                content: system_prompt.into(),
            },
            ChatMessage {
                role: "user".into(),
                content: user_message.into(),
            },
        ],
        temperature: 0.2,
        max_tokens: 4096,
        response_format: None,
    };

    debug!(model = %config.model, url = %url, "sending LLM request");

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .context("LLM API request failed")?;

    let status = response.status();
    let body = response.text().await.context("failed to read LLM response body")?;

    if !status.is_success() {
        anyhow::bail!(
            "LLM API returned status {status}: {body}",
        );
    }

    if let Some(sp) = spinner {
        sp.set_message("Parsing response…");
    }

    let parsed: ChatResponse =
        serde_json::from_str(&body).context(format!("failed to parse LLM JSON response: {body}"))?;

    let content = parsed
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_default();

    debug!(tokens = ?parsed.usage, "LLM response received");
    tracing::Span::current().record("tokens_used", parsed.usage.as_ref().map(|u| u.total_tokens));

    Ok(content)
}

/// Create an animated spinner for LLM operations.
fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
    );
    spinner.set_message(message.to_string());
    spinner
}

/// Review a diff using the LLM. Returns a `ReviewResponse`.
pub async fn review_diff(
    llm_config: &LLMConfig,
    diff: &str,
    focus: &[String],
    rules: &[String],
) -> Result<ReviewResponse> {
    let spinner = create_spinner("Reviewing diff…");

    let user_prompt = build_review_prompt(diff, focus, rules);

    let raw = chat_completion(llm_config, REVIEW_SYSTEM_PROMPT, &user_prompt, Some(&spinner)).await?;

    let (issues, summary, tokens_used) = parse_review_response(&raw)?;

    spinner.finish_and_clear();
    Ok(ReviewResponse {
        issues,
        summary,
        tokens_used,
        should_block: false, // caller sets this based on config
    })
}

/// Scan a batch of file contents using the LLM. Returns issues found.
pub async fn scan_files(
    llm_config: &LLMConfig,
    files_content: &str,
    focus: &[String],
    rules: &[String],
) -> Result<(Vec<ReviewIssue>, Option<String>, Option<TokenUsage>)> {
    let spinner = create_spinner("Scanning files…");

    let mut user_prompt = String::new();
    if !focus.is_empty() {
        user_prompt.push_str(&format!("Focus areas: {}\n\n", focus.join(", ")));
    }
    if !rules.is_empty() {
        user_prompt.push_str(&format!(
            "Additional rules:\n{}\n\n",
            rules.iter().map(|r| format!("- {}", r)).collect::<Vec<_>>().join("\n")
        ));
    }
    user_prompt.push_str("Files to review:\n\n");
    user_prompt.push_str(files_content);

    let raw = chat_completion(llm_config, SCAN_SYSTEM_PROMPT, &user_prompt, Some(&spinner)).await?;

    parse_scan_response(&raw)
}

/// Build the user prompt for diff review.
fn build_review_prompt(diff: &str, focus: &[String], rules: &[String]) -> String {
    let mut prompt = String::new();

    if !focus.is_empty() {
        prompt.push_str(&format!("Focus areas: {}\n\n", focus.join(", ")));
    }

    if !rules.is_empty() {
        prompt.push_str("Additional review rules:\n");
        for rule in rules {
            prompt.push_str(&format!("- {rule}\n"));
        }
        prompt.push('\n');
    }

    prompt.push_str("Review the following diff:\n\n```diff\n");
    prompt.push_str(diff);
    prompt.push_str("\n```\n");

    prompt
}

/// Parse the LLM response into review issues.
/// Handles: raw JSON array, JSON wrapped in ```json fences, array|||summary format.
fn parse_review_response(
    raw: &str,
) -> Result<(Vec<ReviewIssue>, String, Option<TokenUsage>)> {
    let (json_str, summary) = extract_json_and_summary(raw);

    // Strip markdown code fences if present
    let json_str = strip_code_fences(&json_str);

    let issues: Vec<ReviewIssue> = serde_json::from_str(&json_str)
        .context("LLM response is not valid JSON array of issues")?;

    Ok((issues, summary, None))
}

/// Parse the LLM response for scan mode.
fn parse_scan_response(
    raw: &str,
) -> Result<(Vec<ReviewIssue>, Option<String>, Option<TokenUsage>)> {
    let (json_str, summary) = extract_json_and_summary(raw);
    let json_str = strip_code_fences(&json_str);

    let issues: Vec<ReviewIssue> = serde_json::from_str(&json_str)
        .context("LLM scan response is not valid JSON array of issues")?;

    let summary = if summary.is_empty() { None } else { Some(summary) };

    Ok((issues, summary, None))
}

/// Extract JSON and optional summary (after ||| separator).
fn extract_json_and_summary(raw: &str) -> (String, String) {
    if let Some(idx) = raw.find("|||") {
        let json_part = raw[..idx].trim().to_string();
        let summary_part = raw[idx + 3..].trim().to_string();
        (json_part, summary_part)
    } else {
        // Try to find the JSON array boundaries
        let trimmed = raw.trim();
        if trimmed.starts_with('[') {
            // Find the matching closing bracket
            let mut depth = 0;
            let mut end = 0;
            for (i, c) in trimmed.char_indices() {
                match c {
                    '[' => depth += 1,
                    ']' => {
                        depth -= 1;
                        if depth == 0 {
                            end = i + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if end > 0 {
                let json_part = trimmed[..end].to_string();
                let summary_part = trimmed[end..].trim().to_string();
                return (json_part, summary_part);
            }
        }
        (trimmed.to_string(), String::new())
    }
}

/// Strip ```json / ``` code fences from the response.
fn strip_code_fences(s: &str) -> String {
    let trimmed = s.trim();
    if let Some(stripped) = trimmed
        .strip_prefix("```json")
        .or_else(|| trimmed.strip_prefix("```"))
    {
        stripped.strip_suffix("```").unwrap_or(stripped).trim().to_string()
    } else {
        trimmed.to_string()
    }
}


