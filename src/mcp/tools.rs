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
        // ─── Review & Security ───
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
        // ─── Config ───
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
        // ─── Code Intelligence ───
        Tool {
            name: "cora.search_symbols".to_string(),
            description: "Search the symbol index for code intelligence. Returns matching symbols with file location, kind, and signature. Requires `cora index` to be run first.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search query (symbol name or keyword)" },
                    "kind": { "type": "string", "description": "Filter by kind: function, struct, enum, trait, method, constant, module" },
                    "file": { "type": "string", "description": "Filter by file path prefix" },
                    "language": { "type": "string", "description": "Filter by language (rs, py, ts, go, etc.)" },
                    "limit": { "type": "integer", "description": "Max results (default 50)", "default": 50 }
                },
                "required": ["query"]
            }),
        },
        Tool {
            name: "cora.find_callers".to_string(),
            description: "Find all callers of a symbol (who calls this function/method?). Uses reverse call graph traversal.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "symbol": { "type": "string", "description": "Symbol name to find callers for" },
                    "limit": { "type": "integer", "description": "Max results (default 50)", "default": 50 }
                },
                "required": ["symbol"]
            }),
        },
        Tool {
            name: "cora.find_impact".to_string(),
            description: "Analyze the blast radius of changing a symbol. Returns all affected symbols up to the specified depth.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "symbol": { "type": "string", "description": "Symbol name to analyze" },
                    "depth": { "type": "integer", "description": "Traversal depth (default 3)", "default": 3 }
                },
                "required": ["symbol"]
            }),
        },
        Tool {
            name: "cora.find_affected_tests".to_string(),
            description: "Find test files affected by source code changes. Uses call graph + naming conventions.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "files": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Changed source files"
                    }
                },
                "required": ["files"]
            }),
        },
        Tool {
            name: "cora.index_status".to_string(),
            description: "Check if a symbol index exists and get statistics (total symbols, files, languages).".to_string(),
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
        // Review & Security
        "cora.list_rules" => handle_list_rules(),
        "cora.check_snippet" => handle_check_snippet(params),
        // Config
        "cora.get_quality_gate" => handle_get_quality_gate(),
        "cora.get_config" => handle_get_config(params),
        "cora.list_profiles" => handle_list_profiles(),
        // Code Intelligence
        "cora.search_symbols" => handle_search_symbols(params),
        "cora.find_callers" => handle_find_callers(params),
        "cora.find_impact" => handle_find_impact(params),
        "cora.find_affected_tests" => handle_find_affected_tests(params),
        "cora.index_status" => handle_index_status(),
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

// ─── Code Intelligence Handlers ───

/// Open the index database, returning helpful error if not found.
fn open_index_db() -> anyhow::Result<rusqlite::Connection> {
    let cwd = std::env::current_dir()?;
    let db_path = crate::index::default_db_path(&cwd);
    if !db_path.exists() {
        anyhow::bail!("No symbol index found. Run 'cora index' first to build the index.");
    }
    crate::index::open_index(&db_path)
}

fn handle_search_symbols(params: &serde_json::Value) -> ToolResult {
    let query_text = match params.get("query").and_then(|v| v.as_str()) {
        Some(q) => q,
        None => return ToolResult::error("Missing required parameter: query"),
    };

    let conn = match open_index_db() {
        Ok(c) => c,
        Err(e) => return ToolResult::error(e.to_string()),
    };

    let kind = params
        .get("kind")
        .and_then(|v| v.as_str())
        .map(crate::index::SymbolKind::from_str);
    let file_prefix = params
        .get("file")
        .and_then(|v| v.as_str())
        .map(String::from);
    let language = params
        .get("language")
        .and_then(|v| v.as_str())
        .map(String::from);
    let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(50) as usize;

    let query = crate::index::SymbolQuery {
        text: Some(query_text.to_string()),
        kind,
        file_prefix,
        language,
        limit,
    };

    match crate::index::search(&conn, &query) {
        Ok(results) => {
            if results.is_empty() {
                return ToolResult::text(format!("No symbols found matching '{query_text}'."));
            }
            let json: Vec<serde_json::Value> = results
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "name": r.symbol.name,
                        "kind": r.symbol.kind.as_str(),
                        "file": r.symbol.file,
                        "line": r.symbol.line,
                        "signature": r.symbol.signature,
                        "language": r.symbol.language,
                        "score": r.score,
                    })
                })
                .collect();
            ToolResult::text(serde_json::to_string_pretty(&json).unwrap_or_default())
        }
        Err(e) => ToolResult::error(format!("Search failed: {e}")),
    }
}

fn handle_find_callers(params: &serde_json::Value) -> ToolResult {
    let symbol = match params.get("symbol").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return ToolResult::error("Missing required parameter: symbol"),
    };
    let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(50) as usize;

    let conn = match open_index_db() {
        Ok(c) => c,
        Err(e) => return ToolResult::error(e.to_string()),
    };

    match crate::index::graph::find_callers(&conn, symbol, limit) {
        Ok(callers) => {
            if callers.is_empty() {
                return ToolResult::text(format!("No callers found for '{symbol}'."));
            }
            let json: Vec<serde_json::Value> = callers
                .iter()
                .map(|c| {
                    serde_json::json!({
                        "caller": c.caller,
                        "file": c.file,
                        "line": c.line,
                    })
                })
                .collect();
            ToolResult::text(serde_json::to_string_pretty(&json).unwrap_or_default())
        }
        Err(e) => ToolResult::error(format!("Find callers failed: {e}")),
    }
}

fn handle_find_impact(params: &serde_json::Value) -> ToolResult {
    let symbol = match params.get("symbol").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return ToolResult::error("Missing required parameter: symbol"),
    };
    let depth = params.get("depth").and_then(|v| v.as_u64()).unwrap_or(3) as u32;

    let conn = match open_index_db() {
        Ok(c) => c,
        Err(e) => return ToolResult::error(e.to_string()),
    };

    match crate::index::graph::impact_analysis(&conn, symbol, depth) {
        Ok(impact) => {
            if impact.is_empty() {
                return ToolResult::text(format!("No impact found for '{symbol}'."));
            }
            let json: Vec<serde_json::Value> = impact
                .iter()
                .map(|n| {
                    serde_json::json!({
                        "symbol": n.symbol,
                        "file": n.file,
                        "line": n.line,
                        "depth": n.depth,
                    })
                })
                .collect();
            ToolResult::text(serde_json::to_string_pretty(&json).unwrap_or_default())
        }
        Err(e) => ToolResult::error(format!("Impact analysis failed: {e}")),
    }
}

fn handle_find_affected_tests(params: &serde_json::Value) -> ToolResult {
    let files: Vec<String> = match params.get("files").and_then(|v| v.as_array()) {
        Some(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        None => {
            return ToolResult::error("Missing required parameter: files (array of file paths)");
        }
    };
    if files.is_empty() {
        return ToolResult::error("Parameter 'files' must not be empty");
    }

    let conn = match open_index_db() {
        Ok(c) => c,
        Err(e) => return ToolResult::error(e.to_string()),
    };

    let patterns = ["test", "spec", "_test", "_spec"];
    let mut affected: std::collections::HashSet<String> = std::collections::HashSet::new();

    for file in &files {
        // Strategy 1: call graph
        let symbols_in_file: Vec<String> = {
            let mut stmt = match conn.prepare("SELECT DISTINCT name FROM symbols WHERE file = ?1") {
                Ok(s) => s,
                Err(e) => return ToolResult::error(format!("DB error: {e}")),
            };
            let rows = match stmt.query_map(rusqlite::params![file], |row| row.get::<_, String>(0))
            {
                Ok(r) => r,
                Err(e) => return ToolResult::error(format!("DB error: {e}")),
            };
            rows.filter_map(|r| r.ok()).collect()
        };

        for sym_name in &symbols_in_file {
            if let Ok(callers) = crate::index::graph::find_callers(&conn, sym_name, 100) {
                for caller in callers {
                    if patterns.iter().any(|p| caller.file.contains(*p)) {
                        affected.insert(caller.file.clone());
                    }
                }
            }
        }

        // Strategy 2: naming convention
        let stem = file
            .rsplit('/')
            .next()
            .unwrap_or(file)
            .rsplit('.')
            .next()
            .unwrap_or("");
        let test_names = [
            format!("{stem}_test.rs"),
            format!("tests/{stem}.rs"),
            format!("{stem}_test.go"),
            format!("test_{stem}.py"),
            format!("{stem}.test.ts"),
            format!("{stem}.spec.ts"),
        ];
        for tn in &test_names {
            if let Ok(mut stmt) = conn.prepare("SELECT DISTINCT path FROM files WHERE path LIKE ?1")
            {
                let pattern = format!("%{tn}");
                if let Ok(rows) =
                    stmt.query_map(rusqlite::params![pattern], |row| row.get::<_, String>(0))
                {
                    for row in rows.map_while(Result::ok) {
                        affected.insert(row);
                    }
                }
            }
        }
    }

    let mut sorted: Vec<String> = affected.into_iter().collect();
    sorted.sort();

    let json = serde_json::json!({
        "affected_tests": sorted,
        "count": sorted.len(),
    });
    ToolResult::text(serde_json::to_string_pretty(&json).unwrap_or_default())
}

fn handle_index_status() -> ToolResult {
    let conn = match open_index_db() {
        Ok(c) => c,
        Err(e) => return ToolResult::error(e.to_string()),
    };

    match crate::index::index_stats(&conn) {
        Ok(stats) => {
            let json = serde_json::json!({
                "exists": true,
                "total_symbols": stats.total_symbols,
                "total_files": stats.total_files,
                "db_size_bytes": stats.db_size_bytes,
                "symbols_by_kind": stats.symbols_by_kind,
                "symbols_by_language": stats.symbols_by_language,
            });
            ToolResult::text(serde_json::to_string_pretty(&json).unwrap_or_default())
        }
        Err(e) => ToolResult::error(format!("Failed to get stats: {e}")),
    }
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

    // ─── Code Intelligence Tool Tests ───

    #[test]
    fn list_tools_includes_code_intel() {
        let tools = list_tools();
        assert!(tools.iter().any(|t| t.name == "cora.search_symbols"));
        assert!(tools.iter().any(|t| t.name == "cora.find_callers"));
        assert!(tools.iter().any(|t| t.name == "cora.find_impact"));
        assert!(tools.iter().any(|t| t.name == "cora.find_affected_tests"));
        assert!(tools.iter().any(|t| t.name == "cora.index_status"));
    }

    #[test]
    fn handle_index_status_no_index() {
        // From test dir, there should be no index
        let result = handle_tool_call("cora.index_status", &serde_json::json!({}));
        // May error if no index — that's acceptable
        assert!(result.is_error || result.content[0].text.contains("total_symbols"));
    }

    #[test]
    fn handle_search_symbols_missing_query() {
        let result = handle_tool_call("cora.search_symbols", &serde_json::json!({}));
        assert!(result.is_error);
    }

    #[test]
    fn handle_find_callers_missing_symbol() {
        let result = handle_tool_call("cora.find_callers", &serde_json::json!({}));
        assert!(result.is_error);
    }

    #[test]
    fn handle_find_impact_missing_symbol() {
        let result = handle_tool_call("cora.find_impact", &serde_json::json!({}));
        assert!(result.is_error);
    }

    #[test]
    fn handle_find_affected_tests_missing_files() {
        let result = handle_tool_call("cora.find_affected_tests", &serde_json::json!({}));
        assert!(result.is_error);
    }

    #[test]
    fn handle_find_affected_tests_empty_files() {
        let result = handle_tool_call(
            "cora.find_affected_tests",
            &serde_json::json!({"files": []}),
        );
        assert!(result.is_error);
    }
}
