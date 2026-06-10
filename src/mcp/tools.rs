//! MCP tool handlers — implement the actual tool logic.
//!
//! Each handler takes JSON params and returns a ToolResult.

use crate::engine::diff_parser;
use crate::engine::profiles;
use crate::engine::rules;
use crate::engine::secrets_scanner;
use crate::engine::security_scanner;

use super::protocol::{Tool, ToolResult};

/// List all available MCP tools.
pub fn list_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "cora.list_rules".to_string(),
            description: "List all active review rules, quality profiles, and security patterns for this project.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
        Tool {
            name: "cora.check_snippet".to_string(),
            description: "Check a code snippet against cora's deterministic rules (secrets, security patterns). No LLM call.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "code": { "type": "string", "description": "Code snippet to check" },
                    "language": { "type": "string", "description": "Language of the snippet (e.g., 'rs', 'py', 'go')" }
                },
                "required": ["code"]
            }),
        },
        Tool {
            name: "cora.get_quality_gate".to_string(),
            description: "Get the current quality gate configuration and thresholds.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
        Tool {
            name: "cora.get_config".to_string(),
            description: "Get the effective cora configuration for this project (without secrets).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "section": { "type": "string", "description": "Config section to get (e.g., 'quality_gate', 'provider', 'rules')" }
                },
                "required": []
            }),
        },
        Tool {
            name: "cora.list_profiles".to_string(),
            description: "List all available quality profiles (built-in and custom).".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
    ]
}

/// Dispatch a tool call to the appropriate handler.
pub fn handle_tool_call(name: &str, params: &serde_json::Value) -> ToolResult {
    match name {
        "cora.list_rules" => handle_list_rules(),
        "cora.check_snippet" => handle_check_snippet(params),
        "cora.get_quality_gate" => handle_get_quality_gate(),
        "cora.get_config" => handle_get_config(params),
        "cora.list_profiles" => handle_list_profiles(),
        _ => ToolResult::error(format!("Unknown tool: {name}")),
    }
}

fn handle_list_rules() -> ToolResult {
    let mut sections = Vec::new();

    // Built-in rule engine rules
    sections.push("## Rule Engine".to_string());
    let builtin_rules = rules::builtin::builtin_rules();
    for rule in &builtin_rules {
        sections.push(format!(
            "- **{}**: {} (severity: {:?})",
            rule.id, rule.message, rule.severity
        ));
    }

    // Secret patterns
    sections.push("\n## Secret Patterns".to_string());
    sections.push(format!(
        "{} built-in secret detection patterns (AWS, GitHub, OpenAI, Anthropic, etc.)",
        "12"
    ));

    // Security patterns
    sections.push("\n## Security Patterns".to_string());
    sections.push("11 static security patterns:".to_string());
    for pattern in security_scanner::PATTERNS {
        sections.push(format!(
            "- **{}**: {} ({:?})",
            pattern.id, pattern.name, pattern.severity
        ));
    }

    ToolResult::text(sections.join("\n"))
}

fn handle_check_snippet(params: &serde_json::Value) -> ToolResult {
    let code = match params.get("code").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return ToolResult::error("Missing required parameter: code"),
    };

    let lang = params
        .get("language")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    // Create a fake diff chunk from the snippet
    let fake_diff = format!(
        "diff --git a/snippet.{lang} b/snippet.{lang}\n--- a/snippet.{lang}\n+++ b/snippet.{lang}\n@@ -0,0 +1,{count} @@\n{added}",
        count = code.lines().count(),
        added = code
            .lines()
            .map(|l| format!("+{l}"))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    let chunks = diff_parser::parse_diff(&fake_diff);

    let mut findings = Vec::new();

    // Run secrets scanner
    let secrets = secrets_scanner::scan_secrets(&chunks, 10);
    findings.extend(secrets);

    // Run security scanner
    let security = security_scanner::scan_security(&chunks, 10);
    findings.extend(security);

    if findings.is_empty() {
        ToolResult::text("✅ No issues found in snippet by deterministic scanners.")
    } else {
        let mut lines = vec![format!("Found {} issue(s):\n", findings.len())];
        for f in &findings {
            lines.push(format!(
                "- **{}** ({}): {} — {}",
                f.rule_id,
                f.severity.label(),
                f.title,
                f.body
            ));
        }
        ToolResult::text(lines.join("\n"))
    }
}

fn handle_get_quality_gate() -> ToolResult {
    match load_project_config() {
        Ok(config) => {
            let qg = &config.quality_gate;
            let output = format!(
                "## Quality Gate\n- **Enabled**: {}\n- **Thresholds**:\n  - max_critical: {}\n  - max_major: {}\n  - max_minor: {}\n  - max_security: {}\n- **Categories**: {}",
                qg.enabled,
                qg.thresholds.max_critical,
                qg.thresholds.max_major,
                qg.thresholds.max_minor,
                qg.thresholds.max_security,
                qg.categories.len(),
            );
            ToolResult::text(output)
        }
        Err(e) => ToolResult::error(format!("Failed to load config: {e}")),
    }
}

fn handle_get_config(params: &serde_json::Value) -> ToolResult {
    let section = params.get("section").and_then(|v| v.as_str());

    match load_project_config() {
        Ok(config) => {
            let output = match section {
                Some("provider") => {
                    format!(
                        "Provider: {} ({})",
                        config.provider.provider, config.provider.model
                    )
                }
                Some("quality_gate") => format!(
                    "Enabled: {}, max_critical: {}, max_security: {}",
                    config.quality_gate.enabled,
                    config.quality_gate.thresholds.max_critical,
                    config.quality_gate.thresholds.max_security,
                ),
                Some("rules") => format!("{} custom rules configured", config.rules.len()),
                Some("focus") => format!("Focus areas: {}", config.focus.join(", ")),
                _ => {
                    // Return safe overview (no secrets)
                    serde_json::to_string_pretty(&serde_json::json!({
                        "provider": config.provider.provider,
                        "model": config.provider.model,
                        "focus": config.focus,
                        "quality_gate_enabled": config.quality_gate.enabled,
                        "rules_count": config.rules.len(),
                    }))
                    .unwrap_or_else(|_| "Failed to serialize config".to_string())
                }
            };
            ToolResult::text(output)
        }
        Err(e) => ToolResult::error(format!("Failed to load config: {e}")),
    }
}

fn handle_list_profiles() -> ToolResult {
    let mut lines = vec!["## Available Quality Profiles\n".to_string()];

    for name in profiles::BUILTIN_PROFILES {
        if let Some(p) = profiles::load_builtin(name) {
            lines.push(format!(
                "### {}\n{}\nFocus areas: {}\n",
                p.name,
                p.description,
                p.focus_areas
                    .iter()
                    .map(|a| a.id.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }
    }

    ToolResult::text(lines.join("\n"))
}

/// Load project config safely (no API keys exposed).
fn load_project_config() -> anyhow::Result<crate::config::schema::Config> {
    let mut config = crate::config::schema::Config::default();
    let cwd = std::env::current_dir()?;

    if let Some((_, cora)) = crate::config::loader::find_cora_file(&cwd)? {
        cora.merge_into(&mut config)?;
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_tools_returns_tools() {
        let tools = list_tools();
        assert!(tools.len() >= 5);
        assert!(tools.iter().any(|t| t.name == "cora.list_rules"));
        assert!(tools.iter().any(|t| t.name == "cora.check_snippet"));
    }

    #[test]
    fn handle_unknown_tool() {
        let result = handle_tool_call("cora.unknown", &serde_json::json!({}));
        assert!(result.is_error);
    }

    #[test]
    fn handle_list_rules() {
        let result = handle_tool_call("cora.list_rules", &serde_json::json!({}));
        assert!(!result.is_error);
        assert!(result.content[0].text.contains("Rule Engine"));
        assert!(result.content[0].text.contains("Security Patterns"));
    }

    #[test]
    fn handle_check_snippet_clean() {
        let result = handle_tool_call(
            "cora.check_snippet",
            &serde_json::json!({"code": "let x = 1;", "language": "rs"}),
        );
        assert!(!result.is_error);
        assert!(result.content[0].text.contains("No issues"));
    }

    #[test]
    fn handle_check_snippet_secret() {
        let result = handle_tool_call(
            "cora.check_snippet",
            &serde_json::json!({"code": "key = 'AKIAIOSFODNN7EXAMPLE'", "language": "py"}),
        );
        assert!(!result.is_error);
        assert!(result.content[0].text.contains("issue"));
    }

    #[test]
    fn handle_check_snippet_missing_code() {
        let result = handle_tool_call("cora.check_snippet", &serde_json::json!({}));
        assert!(result.is_error);
    }

    #[test]
    fn handle_list_profiles() {
        let result = handle_tool_call("cora.list_profiles", &serde_json::json!({}));
        assert!(!result.is_error);
        assert!(result.content[0].text.contains("security-first"));
        assert!(result.content[0].text.contains("rust-strict"));
    }
}
