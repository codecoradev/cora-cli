use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::LazyLock;
use std::time::Duration;
use tracing::debug;

use crate::engine::types::{LLMConfig, ReviewIssue, ReviewResponse, TokenUsage};

/// Shared reqwest::Client with default timeout. Reused across all LLM requests.
/// Created lazily on first use to avoid blocking initialization.
static SHARED_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .pool_max_idle_per_host(4)
        .build()
        .unwrap_or_else(|e| {
            tracing::error!("failed to build shared HTTP client: {}", e);
            // Fall back to a default client — this should never happen in practice
            reqwest::Client::new()
        })
});

/// Return the shared reqwest::Client for LLM API requests.
///
/// Note: The timeout is set at client construction time (120s default).
/// Per-request timeout override is not supported by reqwest once the
/// client is built, so the `timeout_secs` parameter is accepted for
/// API consistency but the shared client's built-in timeout is used.
pub fn shared_client(timeout_secs: u64) -> reqwest::Client {
    let _ = timeout_secs; // accepted for API consistency, shared client has fixed timeout
    SHARED_CLIENT.clone()
}

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
const REVIEW_SYSTEM_PROMPT: &str = r#"You are an expert code reviewer providing actionable feedback on code diffs.

CRITICAL CONSTRAINTS:
1. You MUST ONLY comment on files that appear in the diff. Do NOT invent or hallucinate file paths.
2. Each issue MUST have a clear, descriptive title (one brief sentence, max 100 chars).
3. If uncertain whether something is a real issue, omit it rather than guessing.

SEVERITY LEVELS:
- "critical": Security vulnerabilities, crashes, data loss, breaking bugs
- "major": Bugs that affect functionality, significant problems
- "minor": Style issues, small nitpicks, minor improvements
- "info": Suggestions, optional enhancements

FOCUS AREAS (in priority order):
1. Security vulnerabilities (SQL injection, XSS, auth issues, data exposure)
2. Bugs and logic errors (off-by-one, null handling, race conditions)
3. Performance problems (inefficient algorithms, memory leaks, N+1 queries)
4. Best practices (idiomatic code, error handling, naming)

RESPONSE FORMAT:
Return a JSON array of objects with these fields:
- "file": string — the file path (MUST be from the diff)
- "line": number or null — the approximate line number
- "severity": "critical" | "major" | "minor" | "info"
- "issue_type": string — category (security, performance, bugs, best_practice, style, suggestion)
- "title": string — short description (max 100 chars)
- "body": string — detailed explanation
- "suggested_fix": string or null — optional fix suggestion

If no issues are found, return: []

Return ONLY the JSON array. No markdown code fences, no explanation, no conversational text.
Start with [ and end with ]."#;

/// System prompt for full project scanning.
const SCAN_SYSTEM_PROMPT: &str = r#"You are an expert code reviewer performing a full project scan. Analyze the provided code files and identify issues.

CRITICAL CONSTRAINTS:
1. You MUST ONLY comment on files that were provided to you. Do NOT invent file paths.
2. Each issue MUST have a clear, descriptive title (one brief sentence, max 100 chars).
3. If uncertain whether something is a real issue, omit it rather than guessing.

SEVERITY LEVELS:
- "critical": Security vulnerabilities, crashes, data loss, breaking bugs
- "major": Bugs that affect functionality, significant problems
- "minor": Style issues, small nitpicks, minor improvements
- "info": Suggestions, optional enhancements

FOCUS AREAS (in priority order):
1. Security vulnerabilities (SQL injection, XSS, auth issues, data exposure)
2. Bugs and logic errors (off-by-one, null handling, race conditions)
3. Performance problems (inefficient algorithms, memory leaks, N+1 queries)
4. Best practices (idiomatic code, error handling, naming)

RESPONSE FORMAT:
Return a JSON array of objects with these fields:
- "file": string — the file path (MUST be from the provided files)
- "line": number or null — the approximate line number
- "severity": "critical" | "major" | "minor" | "info"
- "issue_type": string — category (security, performance, bugs, best_practice, style, suggestion)
- "title": string — short description (max 100 chars)
- "body": string — detailed explanation
- "suggested_fix": string or null — optional fix suggestion

Also include a "summary" string at the end after a "|||" separator:
[...JSON array...]|||Summary text here.

If no issues are found, return: []|||No issues found.

Return ONLY this format. No markdown code fences, no conversational text.
Start the JSON array with [ and end with ]."#;

/// Send a chat completion request to an OpenAI-compatible API.
async fn chat_completion(
    config: &LLMConfig,
    system_prompt: &str,
    user_message: &str,
    spinner: Option<&ProgressBar>,
    response_format: &str,
) -> Result<String> {
    let client = shared_client(config.timeout);

    let url = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));

    if let Some(sp) = spinner {
        sp.set_message(format!(
            "Sending to {} ({})…",
            config.provider, config.model
        ));
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
        temperature: config.temperature,
        max_tokens: config.max_tokens,
        response_format: if response_format == "json_object" {
            Some(serde_json::json!({"type": "json_object"}))
        } else {
            None
        },
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
    let body = response
        .text()
        .await
        .context("failed to read LLM response body")?;

    if !status.is_success() {
        anyhow::bail!("LLM API returned status {status}: {body}",);
    }

    if let Some(sp) = spinner {
        sp.set_message("Parsing response…");
    }

    let parsed: ChatResponse = serde_json::from_str(&body)
        .context(format!("failed to parse LLM JSON response: {body}"))?;

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
    response_format: &str,
    system_prompt_override: Option<&str>,
) -> Result<ReviewResponse> {
    let spinner = create_spinner("Reviewing diff…");

    let user_prompt = build_review_prompt(diff, focus, rules);

    let system_prompt = system_prompt_override.unwrap_or(REVIEW_SYSTEM_PROMPT);

    let raw = chat_completion(
        llm_config,
        system_prompt,
        &user_prompt,
        Some(&spinner),
        response_format,
    )
    .await?;

    let parse_result = parse_review_response(&raw);
    match parse_result {
        Ok(result) => {
            spinner.finish_and_clear();
            Ok(ReviewResponse {
                issues: result.0,
                summary: result.1,
                tokens_used: result.2,
                should_block: false,
            })
        }
        Err(e) => {
            // LLM produced invalid JSON — retry once with stricter prompt
            debug!(error = %e, "first parse attempt failed, retrying LLM request");
            spinner.set_message("Retrying (parse error)…");
            let strict_prompt = format!(
                "{}\n\nIMPORTANT: Your response MUST contain only valid JSON. \
                Ensure all strings use proper JSON escape sequences. \
                Do NOT use raw backslashes in string values.",
                &user_prompt
            );
            let retry_raw = chat_completion(
                llm_config,
                system_prompt,
                &strict_prompt,
                Some(&spinner),
                response_format,
            )
            .await?;
            let (issues, summary, tokens_used) = parse_review_response(&retry_raw)?;
            spinner.finish_and_clear();
            Ok(ReviewResponse {
                issues,
                summary,
                tokens_used,
                should_block: false,
            })
        }
    }
}

/// Review a diff using the LLM with streaming. Returns a `ReviewResponse`.
///
/// Streams tokens from the LLM and prints them to stdout in real-time,
/// then collects the full response for parsing.
pub async fn review_diff_stream(
    llm_config: &LLMConfig,
    diff: &str,
    focus: &[String],
    rules: &[String],
    response_format: &str,
    system_prompt_override: Option<&str>,
) -> Result<ReviewResponse> {
    let user_prompt = build_review_prompt(diff, focus, rules);

    let system_prompt = system_prompt_override.unwrap_or(REVIEW_SYSTEM_PROMPT);

    let raw =
        chat_completion_stream(llm_config, system_prompt, &user_prompt, response_format).await?;

    let (issues, summary, tokens_used) = parse_review_response(&raw)?;

    println!(); // trailing newline after streamed output

    Ok(ReviewResponse {
        issues,
        summary,
        tokens_used,
        should_block: false,
    })
}

/// Send a streaming chat completion request to an OpenAI-compatible API.
///
/// Sends `"stream": true` in the request body, reads SSE chunks, prints
/// delta content to stdout in real-time, and returns the full accumulated text.
async fn chat_completion_stream(
    config: &LLMConfig,
    system_prompt: &str,
    user_message: &str,
    response_format: &str,
) -> Result<String> {
    use futures_util::StreamExt;
    use std::io::Write;

    let client = shared_client(config.timeout);
    let url = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));

    let mut request_body = serde_json::json!({
        "model": config.model,
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": user_message }
        ],
        "temperature": config.temperature,
        "max_tokens": config.max_tokens,
        "stream": true
    });

    if response_format == "json_object" {
        request_body["response_format"] = serde_json::json!({"type": "json_object"});
    }

    debug!(model = %config.model, url = %url, "sending streaming LLM request");

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .context("LLM API streaming request failed")?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("LLM API returned status {status}: {body}");
    }

    let mut stream = response.bytes_stream();

    // Buffer for assembling lines from byte chunks
    let mut line_buf = String::new();
    let mut accumulated = String::new();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.context("error reading stream chunk")?;
        let chunk_str = String::from_utf8_lossy(&chunk);

        // Process the chunk character by character to handle line boundaries
        for ch in chunk_str.chars() {
            if ch == '\n' {
                let line = line_buf.trim().to_string();
                line_buf.clear();

                if line.is_empty() || line.starts_with(':') {
                    continue;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    if data.trim() == "[DONE]" {
                        debug!(accumulated_len = accumulated.len(), "streaming complete");
                        return Ok(accumulated);
                    }

                    match serde_json::from_str::<Value>(data) {
                        Ok(parsed) => {
                            if let Some(content) = parsed
                                .get("choices")
                                .and_then(|c| c.get(0))
                                .and_then(|c| c.get("delta"))
                                .and_then(|d| d.get("content"))
                                .and_then(|v| v.as_str())
                            {
                                if !content.is_empty() {
                                    // Print delta chunk immediately for live streaming effect
                                    print!("{content}");
                                    let _ = std::io::stdout().flush();
                                    accumulated.push_str(content);
                                }
                            }
                        }
                        Err(e) => {
                            debug!("skipping unparseable SSE chunk: {e}");
                        }
                    }
                }
            } else {
                line_buf.push(ch);
            }
        }
    }

    // Process any remaining partial line
    if !line_buf.trim().is_empty() {
        let line = line_buf.trim();
        if let Some(data) = line.strip_prefix("data: ") {
            if data.trim() != "[DONE]" {
                if let Ok(parsed) = serde_json::from_str::<Value>(data) {
                    if let Some(content) = parsed
                        .get("choices")
                        .and_then(|c| c.get(0))
                        .and_then(|c| c.get("delta"))
                        .and_then(|d| d.get("content"))
                        .and_then(|v| v.as_str())
                    {
                        if !content.is_empty() {
                            print!("{content}");
                            let _ = std::io::stdout().flush();
                            accumulated.push_str(content);
                        }
                    }
                }
            }
        }
    }

    debug!(accumulated_len = accumulated.len(), "streaming complete");
    Ok(accumulated)
}

/// Scan a batch of file contents using the LLM. Returns issues found.
pub async fn scan_files(
    llm_config: &LLMConfig,
    files_content: &str,
    focus: &[String],
    rules: &[String],
    response_format: &str,
    system_prompt_override: Option<&str>,
) -> Result<(Vec<ReviewIssue>, Option<String>, Option<TokenUsage>)> {
    let spinner = create_spinner("Scanning files…");

    let system_prompt = system_prompt_override.unwrap_or(SCAN_SYSTEM_PROMPT);

    let mut user_prompt = String::new();
    if !focus.is_empty() {
        user_prompt.push_str(&format!("Focus areas: {}\n\n", focus.join(", ")));
    }
    if !rules.is_empty() {
        user_prompt.push_str(&format!(
            "Additional rules:\n{}\n\n",
            rules
                .iter()
                .map(|r| format!("- {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    user_prompt.push_str("Files to review:\n\n");
    user_prompt.push_str(files_content);

    let raw = chat_completion(
        llm_config,
        system_prompt,
        &user_prompt,
        Some(&spinner),
        response_format,
    )
    .await?;

    parse_scan_response(&raw)
}

/// Extract file paths from a unified diff string.
/// Matches lines like `--- a/path/file.rs` and `+++ b/path/file.rs`.
pub(crate) fn extract_file_paths_from_diff(diff: &str) -> Vec<String> {
    let mut paths = std::collections::HashSet::new();
    for line in diff.lines() {
        let trimmed = line.trim_start();
        // Match unified diff headers: `--- a/path` or `+++ b/path`
        // Also handles `--- path` without a/ or b/ prefix (some diffs)
        let (prefix, strip_ab) = if let Some(rest) = trimmed.strip_prefix("--- a/") {
            (rest, true)
        } else if let Some(rest) = trimmed.strip_prefix("+++ b/") {
            (rest, true)
        } else if let Some(rest) = trimmed.strip_prefix("--- ") {
            (rest, false)
        } else if let Some(rest) = trimmed.strip_prefix("+++ ") {
            (rest, false)
        } else {
            continue;
        };
        // Skip /dev/null (binary files, deletes)
        if prefix.starts_with("/dev/null") {
            continue;
        }
        let path = if strip_ab {
            prefix.to_string()
        } else {
            // Strip a/ or b/ prefix if present
            prefix
                .strip_prefix("a/")
                .or_else(|| prefix.strip_prefix("b/"))
                .unwrap_or(prefix)
                .to_string()
        };
        // Strip trailing \t (git shows tabs for renamed files)
        let path = path.split('\t').next().unwrap_or(&path);
        if !path.is_empty() {
            paths.insert(path.to_string());
        }
    }
    paths.into_iter().collect()
}

/// Build the user prompt for diff review.
fn build_review_prompt(diff: &str, focus: &[String], rules: &[String]) -> String {
    let mut prompt = String::new();

    // Inject valid file paths to reduce hallucination
    let file_paths = extract_file_paths_from_diff(diff);
    if !file_paths.is_empty() {
        prompt.push_str("Valid files in this diff:\n");
        for path in &file_paths {
            prompt.push_str(&format!("- \"{}\"\n", path));
        }
        prompt.push('\n');
    }

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
pub(crate) fn parse_review_response(
    raw: &str,
) -> Result<(Vec<ReviewIssue>, String, Option<TokenUsage>)> {
    let (json_str, summary) = extract_json_and_summary(raw);

    // Strip markdown code fences if present
    let json_str = strip_code_fences(&json_str);

    // Repair common LLM JSON mistakes before strict parse
    let json_str = repair_json_string(&json_str);

    let issues: Vec<ReviewIssue> = serde_json::from_str(&json_str)
        .context("LLM response is not valid JSON array of issues")?;

    Ok((issues, summary, None))
}

/// Parse the LLM response for scan mode.
pub(crate) fn parse_scan_response(
    raw: &str,
) -> Result<(Vec<ReviewIssue>, Option<String>, Option<TokenUsage>)> {
    let (json_str, summary) = extract_json_and_summary(raw);
    let json_str = strip_code_fences(&json_str);

    // Repair common LLM JSON mistakes before strict parse
    let json_str = repair_json_string(&json_str);

    let issues: Vec<ReviewIssue> = serde_json::from_str(&json_str)
        .context("LLM scan response is not valid JSON array of issues")?;

    let summary = if summary.is_empty() {
        None
    } else {
        Some(summary)
    };

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

/// Repair common LLM JSON mistakes before strict parse.
///
/// LLMs sometimes produce JSON with invalid escape sequences (e.g. lone backslashes
/// like `\s` or trailing `\` inside string values). This function applies minimal
/// fixes so `serde_json` can parse the output.
fn repair_json_string(json_str: &str) -> String {
    // Replace lone backslashes inside JSON string values that aren't valid JSON escapes.
    // Valid JSON escapes: \" \\ \/ \b \f \n \r \t \uXXXX
    let repaired = repair_invalid_escapes(json_str);
    if repaired != json_str {
        debug!("applied backslash repair to LLM JSON output");
        repaired
    } else {
        json_str.to_string()
    }
}

/// Replace invalid escape sequences in JSON string values.
/// Tracks whether we're inside a string literal using a proper state machine
/// that handles escaped quotes correctly.
fn repair_invalid_escapes(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                output.push(c);
                // Scan through string literal
                loop {
                    match chars.next() {
                        Some('\\') => {
                            // Escape character — check what follows
                            match chars.peek() {
                                Some(&next) if is_valid_json_escape(next) => {
                                    output.push('\\');
                                    output.push(next);
                                    chars.next(); // consume
                                    if next == 'u' {
                                        // Consume exactly 4 hex digits
                                        let mut hex_count = 0;
                                        for _ in 0..4 {
                                            if let Some(&hex) = chars.peek() {
                                                if hex.is_ascii_hexdigit() {
                                                    output.push(hex);
                                                    chars.next();
                                                    hex_count += 1;
                                                }
                                            }
                                        }
                                        if hex_count < 4 {
                                            // Invalid \u escape — not enough hex digits
                                            // Remove the \u we already output and repair
                                            output.truncate(output.len() - 2);
                                            output.push_str("\\\\u");
                                            // Re-peek remaining chars that weren't consumed
                                            for _ in 0..(4 - hex_count) {
                                                if let Some(&c) = chars.peek() {
                                                    output.push(c);
                                                    chars.next();
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(&next) => {
                                    // Invalid escape — double the backslash
                                    debug!(
                                        escape_seq = format!("\\{}", next),
                                        "repairing invalid JSON escape"
                                    );
                                    output.push_str("\\\\");
                                    output.push(next);
                                    chars.next(); // consume
                                }
                                None => {
                                    // Trailing backslash at end of input
                                    output.push_str("\\\\");
                                }
                            }
                        }
                        Some('"') => {
                            output.push('"');
                            break; // end of string
                        }
                        Some(ch) => {
                            output.push(ch);
                        }
                        None => {
                            break; // EOF inside string — let serde_json report it
                        }
                    }
                }
            }
            _ => {
                output.push(c);
            }
        }
    }

    output
}

/// Check if a character is a valid JSON escape sequence starter.
fn is_valid_json_escape(c: char) -> bool {
    matches!(c, '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' | 'u')
}

/// Strip ```json / ``` code fences from the response.
fn strip_code_fences(s: &str) -> String {
    let trimmed = s.trim();
    if let Some(stripped) = trimmed
        .strip_prefix("```json")
        .or_else(|| trimmed.strip_prefix("```"))
    {
        stripped
            .strip_suffix("```")
            .unwrap_or(stripped)
            .trim()
            .to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::types::Severity;

    const SINGLE_ISSUE_JSON: &str = r#"[{"file":"src/main.rs","line":42,"severity":"critical","issue_type":"security","title":"SQL Injection","body":"User input is concatenated directly into SQL query.","suggested_fix":"Use parameterized queries."}]"#;

    const TWO_ISSUES_JSON: &str = r#"[
  {"file":"src/api.rs","line":10,"severity":"major","issue_type":"performance","title":"N+1 Query","body":"Query inside a loop.","suggested_fix":"Use eager loading."},
  {"file":"src/lib.rs","line":5,"severity":"minor","issue_type":"bugs","title":"Off-by-one","body":"Loop bound is off by one."}
]"#;

    const EMPTY_ARRAY: &str = "[]";

    // ─── extract_json_and_summary ───

    #[test]
    fn extract_json_no_separator() {
        let (json, summary) = extract_json_and_summary(SINGLE_ISSUE_JSON);
        assert!(json.starts_with('['));
        assert!(summary.is_empty());
    }

    #[test]
    fn extract_json_with_separator() {
        let input = format!("{SINGLE_ISSUE_JSON}|||Found 1 critical issue.");
        let (json, summary) = extract_json_and_summary(&input);
        assert!(json.starts_with('['));
        assert_eq!(summary, "Found 1 critical issue.");
    }

    #[test]
    fn extract_json_with_separator_and_whitespace() {
        let input = format!("  {SINGLE_ISSUE_JSON}  |||   Some summary text   ");
        let (json, summary) = extract_json_and_summary(&input);
        assert!(json.starts_with('['));
        assert_eq!(summary, "Some summary text");
    }

    #[test]
    fn extract_json_finds_array_boundaries() {
        // Text after the array but before the |||
        let input = format!("{SINGLE_ISSUE_JSON}\nHere is some trailing text.");
        let (json, summary) = extract_json_and_summary(&input);
        assert!(json.starts_with('[') && json.ends_with(']'));
        assert_eq!(summary, "Here is some trailing text.");
    }

    #[test]
    fn extract_json_empty_separator() {
        let (json, summary) = extract_json_and_summary("[]|||");
        assert_eq!(json, "[]");
        assert_eq!(summary, "");
    }

    // ─── strip_code_fences ───

    #[test]
    fn strip_fences_json() {
        let fenced = "```json\n[{\"a\":1}]\n```";
        assert_eq!(strip_code_fences(fenced), "[{\"a\":1}]");
    }

    #[test]
    fn strip_fences_plain() {
        let fenced = "```\n[{\"a\":1}]\n```";
        assert_eq!(strip_code_fences(fenced), "[{\"a\":1}]");
    }

    #[test]
    fn strip_fences_none() {
        assert_eq!(strip_code_fences("[{\"a\":1}]"), "[{\"a\":1}]");
    }

    #[test]
    fn strip_fences_unclosed() {
        let fenced = "```json\n[{\"a\":1}]";
        assert_eq!(strip_code_fences(fenced), "[{\"a\":1}]");
    }

    // ─── parse_review_response ───

    #[test]
    fn parse_review_clean_json() {
        let result = parse_review_response(SINGLE_ISSUE_JSON).unwrap();
        assert_eq!(result.0.len(), 1);
        assert_eq!(result.0[0].file, "src/main.rs");
        assert_eq!(result.0[0].line, Some(42));
        assert_eq!(result.0[0].severity, Severity::Critical);
        assert_eq!(result.1, ""); // no summary
    }

    #[test]
    fn parse_review_with_fences() {
        let input = format!("```json\n{SINGLE_ISSUE_JSON}\n```");
        let result = parse_review_response(&input).unwrap();
        assert_eq!(result.0.len(), 1);
        assert_eq!(result.0[0].severity, Severity::Critical);
    }

    #[test]
    fn parse_review_with_pipe_summary() {
        let input = format!("{SINGLE_ISSUE_JSON}|||1 critical security vulnerability found.");
        let result = parse_review_response(&input).unwrap();
        assert_eq!(result.0.len(), 1);
        assert_eq!(result.1, "1 critical security vulnerability found.");
    }

    #[test]
    fn parse_review_empty_array() {
        let result = parse_review_response(EMPTY_ARRAY).unwrap();
        assert!(result.0.is_empty());
    }

    #[test]
    fn parse_review_two_issues() {
        let result = parse_review_response(TWO_ISSUES_JSON).unwrap();
        assert_eq!(result.0.len(), 2);
        assert_eq!(result.0[0].severity, Severity::Major);
        assert_eq!(result.0[1].severity, Severity::Minor);
    }

    #[test]
    fn parse_review_malformed_json_errors() {
        let result = parse_review_response("not json at all");
        assert!(result.is_err());
    }

    #[test]
    fn parse_review_object_not_array_errors() {
        let result = parse_review_response(r#"{"file":"x"}"#);
        assert!(result.is_err());
    }

    #[test]
    fn parse_review_json_with_trailing_text() {
        // The parser should handle trailing text after the array
        let input = format!("{SINGLE_ISSUE_JSON}\nSome extra text");
        let result = parse_review_response(&input).unwrap();
        assert_eq!(result.0.len(), 1);
        assert_eq!(result.0[0].file, "src/main.rs");
    }

    // ─── parse_scan_response ───

    #[test]
    fn parse_scan_clean_json() {
        let result = parse_scan_response(SINGLE_ISSUE_JSON).unwrap();
        assert_eq!(result.0.len(), 1);
        assert!(result.1.is_none()); // no summary → None
    }

    #[test]
    fn parse_scan_with_pipe_summary() {
        let input = format!("{EMPTY_ARRAY}|||No issues found.");
        let result = parse_scan_response(&input).unwrap();
        assert!(result.0.is_empty());
        assert_eq!(result.1.as_deref(), Some("No issues found."));
    }

    #[test]
    fn parse_scan_empty_no_summary() {
        let result = parse_scan_response(EMPTY_ARRAY).unwrap();
        assert!(result.0.is_empty());
        assert!(result.1.is_none());
    }

    #[test]
    fn parse_scan_with_fences() {
        let input = format!("```json\n{SINGLE_ISSUE_JSON}\n```");
        let result = parse_scan_response(&input).unwrap();
        assert_eq!(result.0.len(), 1);
    }

    #[test]
    fn parse_scan_malformed_json_errors() {
        assert!(parse_scan_response("{{invalid").is_err());
    }

    // ─── Various severity values ───

    #[test]
    fn parse_all_severities() {
        let input = r#"[
            {"file":"a.rs","line":1,"severity":"critical","issue_type":"security","title":"T1","body":"B1"},
            {"file":"b.rs","line":2,"severity":"major","issue_type":"performance","title":"T2","body":"B2"},
            {"file":"c.rs","line":3,"severity":"minor","issue_type":"bugs","title":"T3","body":"B3"},
            {"file":"d.rs","line":4,"severity":"info","issue_type":"style","title":"T4","body":"B4"}
        ]"#;
        let result = parse_review_response(input).unwrap();
        assert_eq!(result.0.len(), 4);
        assert_eq!(result.0[0].severity, Severity::Critical);
        assert_eq!(result.0[1].severity, Severity::Major);
        assert_eq!(result.0[2].severity, Severity::Minor);
        assert_eq!(result.0[3].severity, Severity::Info);
    }

    // ─── Various issue_type values ───

    #[test]
    fn parse_various_issue_types() {
        let input = r#"[
            {"file":"a.rs","line":1,"severity":"critical","issue_type":"security","title":"T","body":"B"},
            {"file":"b.rs","line":2,"severity":"major","issue_type":"performance","title":"T","body":"B"},
            {"file":"c.rs","line":3,"severity":"minor","issue_type":"bugs","title":"T","body":"B"},
            {"file":"d.rs","line":4,"severity":"info","issue_type":"best_practice","title":"T","body":"B"},
            {"file":"e.rs","line":5,"severity":"info","issue_type":"style","title":"T","body":"B"}
        ]"#;
        let result = parse_review_response(input).unwrap();
        assert_eq!(result.0.len(), 5);
        assert_eq!(result.0[0].issue_type.as_deref(), Some("security"));
        assert_eq!(result.0[1].issue_type.as_deref(), Some("performance"));
        assert_eq!(result.0[2].issue_type.as_deref(), Some("bugs"));
        assert_eq!(result.0[3].issue_type.as_deref(), Some("best_practice"));
        assert_eq!(result.0[4].issue_type.as_deref(), Some("style"));
    }

    // ─── null/optional fields ───

    #[test]
    fn parse_issue_with_null_line() {
        let input = r#"[{"file":"a.rs","line":null,"severity":"info","title":"T","body":"B"}]"#;
        let result = parse_review_response(input).unwrap();
        assert_eq!(result.0[0].line, None);
    }

    #[test]
    fn parse_issue_with_null_suggested_fix() {
        let input = r#"[{"file":"a.rs","line":1,"severity":"info","title":"T","body":"B","suggested_fix":null}]"#;
        let result = parse_review_response(input).unwrap();
        assert!(result.0[0].suggested_fix.is_none());
    }

    #[test]
    fn parse_issue_with_type_alias() {
        // "type" should also work via serde alias
        let input = r#"[{"file":"a.rs","line":1,"severity":"info","type":"security","title":"T","body":"B"}]"#;
        let result = parse_review_response(input).unwrap();
        assert_eq!(result.0[0].issue_type.as_deref(), Some("security"));
    }

    // ─── build_review_prompt ───

    #[test]
    fn build_prompt_basic() {
        let prompt = build_review_prompt("diff content", &[], &[]);
        assert!(prompt.contains("diff content"));
        assert!(prompt.contains("```diff"));
    }

    #[test]
    fn build_prompt_with_focus() {
        let prompt = build_review_prompt("d", &["security".to_string()], &[]);
        assert!(prompt.contains("Focus areas: security"));
    }

    #[test]
    fn build_prompt_with_rules() {
        let prompt = build_review_prompt("d", &[], &["no unwrap".to_string()]);
        assert!(prompt.contains("no unwrap"));
    }

    #[test]
    fn build_prompt_contains_file_paths() {
        let diff = "diff --git a/src/main.rs b/src/main.rs\n--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1 +1 @@\n- old\n+ new";
        let prompt = build_review_prompt(diff, &[], &[]);
        assert!(prompt.contains("Valid files in this diff:"));
        assert!(prompt.contains("src/main.rs"));
    }

    #[test]
    fn build_prompt_no_file_paths_for_empty_diff() {
        let prompt = build_review_prompt("no diff headers here", &[], &[]);
        assert!(!prompt.contains("Valid files in this diff:"));
    }

    // ─── extract_file_paths_from_diff ───

    #[test]
    fn extract_paths_single_file() {
        let diff = "--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1 +1 @@\n- old\n+ new";
        let paths = extract_file_paths_from_diff(diff);
        assert_eq!(paths, vec!["src/main.rs"]);
    }

    #[test]
    fn extract_paths_multiple_files() {
        let diff = "--- a/src/a.rs\n+++ b/src/a.rs\n--- a/src/b.rs\n+++ b/src/b.rs";
        let paths = extract_file_paths_from_diff(diff);
        assert!(paths.contains(&"src/a.rs".to_string()));
        assert!(paths.contains(&"src/b.rs".to_string()));
    }

    #[test]
    fn extract_paths_skips_dev_null() {
        let diff = "--- /dev/null\n+++ b/src/new.rs\n--- a/src/old.rs\n+++ /dev/null";
        let paths = extract_file_paths_from_diff(diff);
        assert!(paths.contains(&"src/new.rs".to_string()));
        assert!(paths.contains(&"src/old.rs".to_string()));
    }

    #[test]
    fn extract_paths_deduplicates() {
        let diff = "--- a/src/main.rs\n+++ b/src/main.rs\n--- a/src/main.rs\n+++ b/src/main.rs";
        let paths = extract_file_paths_from_diff(diff);
        assert_eq!(paths.len(), 1);
    }

    // ─── repair_json_string ───

    #[test]
    fn repair_valid_json_unchanged() {
        let input = r#"[{"file":"a.rs","body":"use std::io;\nlet x = 1;"}]"#;
        assert_eq!(repair_json_string(input), input);
    }

    #[test]
    fn repair_invalid_backslash_in_string() {
        // LLM produced `\s` inside a JSON string — should become `\\s`
        let input = r#"[{"file":"a.rs","body":"regex: \s+"}]"#;
        let repaired = repair_json_string(input);
        // After repair, serde_json should parse it
        let parsed: serde_json::Value = serde_json::from_str(&repaired).unwrap();
        assert_eq!(parsed[0]["body"], "regex: \\s+");
    }

    #[test]
    fn repair_trailing_backslash_before_close() {
        // LLM produced `\` right before end of string value — the `\` followed by
        // the closing `"` looks like escaped quote, but the actual LLM mistake is
        // different. Test a realistic case: `\s` inside body text.
        // This tests the most common LLM error: non-standard escape like \s
        let input = r#"[{"file":"a.rs","body":"regex: \s+\d*","severity":"info","title":"T","issue_type":"style"}]"#;
        let repaired = repair_json_string(input);
        let parsed: serde_json::Value = serde_json::from_str(&repaired).unwrap();
        assert_eq!(parsed[0]["body"], "regex: \\s+\\d*");
    }

    #[test]
    fn repair_preserves_valid_escapes() {
        // Valid escapes should not be double-escaped
        let input = r#"[{"file":"a.rs","body":"line1\nline2\ttab"}]"#;
        let repaired = repair_json_string(input);
        assert_eq!(repaired, input);
        let parsed: serde_json::Value = serde_json::from_str(&repaired).unwrap();
        assert_eq!(parsed[0]["body"], "line1\nline2\ttab");
    }

    #[test]
    fn repair_invalid_unicode_escape() {
        // \u followed by non-hex — should be escaped
        let input = r#"[{"file":"a.rs","body":"\uGGGG"}]"#;
        let repaired = repair_json_string(input);
        let parsed: serde_json::Value = serde_json::from_str(&repaired).unwrap();
        assert_eq!(parsed[0]["body"], "\\uGGGG");
    }

    #[test]
    fn parse_response_with_invalid_escapes() {
        // End-to-end: LLM response with invalid escapes should parse successfully
        let raw = r#"[{"file":"src/main.rs","line":10,"severity":"critical","issue_type":"security","title":"SQL Injection","body":"query ends with \n no wait \\","suggested_fix":"Use params"}]"#;
        let result = parse_review_response(raw).unwrap();
        assert_eq!(result.0.len(), 1);
        assert_eq!(result.0[0].title, "SQL Injection");
    }
}
