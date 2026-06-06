/// Built-in rules for the rule engine.
use crate::engine::Severity;
use crate::engine::rules::types::CustomRule;

/// Returns the full list of built-in rules.
pub fn builtin_rules() -> Vec<CustomRule> {
    vec![
        // --- Security ---
        CustomRule {
            id: "sec-hardcoded-secret".to_string(),
            pattern: r#"(?i)(?:password|api_?key|token|secret)\s*=\s*"[^"]+""#
                .to_string(),
            severity: Severity::Critical,
            message: "Possible hardcoded secret/credential detected. Use environment variables or a secrets manager.".to_string(),
            languages: vec!["all".to_string()],
            exclude: vec!["rules/".to_string(), "tests/".to_string(), "test/".to_string()],
        },
        CustomRule {
            id: "sec-sql-concat".to_string(),
            pattern: r##"format!\("SELECT|f"SELECT|f"INSERT|f"UPDATE|f"DELETE|query\s*\+="##
                .to_string(),
            severity: Severity::Critical,
            message: "Possible SQL injection via string concatenation in query. Use parameterized queries.".to_string(),
            languages: vec!["all".to_string()],
            exclude: vec!["rules/".to_string(), "tests/".to_string(), "test/".to_string()],
        },
        CustomRule {
            id: "sec-hardcoded-url".to_string(),
            pattern: r"http://[a-zA-Z0-9][\w.\-]+(:\d+)?(/\S*)?"
                .to_string(),
            severity: Severity::Major,
            message: "Insecure HTTP URL detected (not https). Use HTTPS for all external connections.".to_string(),
            languages: vec!["all".to_string()],
            exclude: vec!["rules/".to_string(), "tests/".to_string(), "test/".to_string()],
        },
        CustomRule {
            id: "sec-tls-disabled".to_string(),
            pattern: r"tls_built_in_root_certs\(false\)|verify\s*=\s*False|InsecureRequestWarning|ACCEPT_INVALID_CERTS|dangerAcceptAnyServerCert".to_string(),
            severity: Severity::Critical,
            message: "TLS verification disabled. This allows man-in-the-middle attacks.".to_string(),
            languages: vec!["rs".to_string(), "py".to_string(), "go".to_string()],
            exclude: vec!["rules/".to_string(), "tests/".to_string(), "test/".to_string()],
        },
        // --- Bugs ---
        CustomRule {
            id: "bug-unwrap".to_string(),
            pattern: r"\.unwrap\(\)".to_string(),
            severity: Severity::Minor,
            message: "Use of `.unwrap()` can panic in production. Handle the error properly.".to_string(),
            languages: vec!["rs".to_string()],
            exclude: vec!["tests/".to_string(), "test/".to_string()],
        },
        CustomRule {
            id: "bug-expect".to_string(),
            pattern: r#"\.expect\(""#
                .to_string(),
            severity: Severity::Minor,
            message: "Use of `.expect()` can panic in production. Consider proper error handling.".to_string(),
            languages: vec!["rs".to_string()],
            exclude: vec!["tests/".to_string(), "test/".to_string()],
        },
        CustomRule {
            id: "bug-println".to_string(),
            pattern: r"(?:println!|dbg!|print!\s*\()".to_string(),
            severity: Severity::Minor,
            message: "Debug output macro found. Remove `println!`/`dbg!`/`print!` before merging.".to_string(),
            languages: vec!["rs".to_string()],
            exclude: vec!["tests/".to_string(), "test/".to_string()],
        },
        CustomRule {
            id: "bug-todo".to_string(),
            pattern: r"(?i)\b(?:TODO|FIXME|HACK|XXX)\b".to_string(),
            severity: Severity::Info,
            message: "TODO/FIXME/HACK/XXX comment found. Consider resolving before merge.".to_string(),
            languages: vec!["all".to_string()],
            exclude: vec![],
        },
        CustomRule {
            id: "bug-console-log".to_string(),
            pattern: r"console\.(?:log|debug|info)\s*\(".to_string(),
            severity: Severity::Minor,
            message: "Console logging statement found. Remove before merging to production.".to_string(),
            languages: vec!["js".to_string(), "ts".to_string()],
            exclude: vec!["tests/".to_string(), "test/".to_string()],
        },
        CustomRule {
            id: "bug-hardcoded-port".to_string(),
            pattern: r#"(?::"8080"|:"3000"|:"5000")"#.to_string(),
            severity: Severity::Info,
            message: "Hardcoded port number detected. Consider using environment variables or config.".to_string(),
            languages: vec!["all".to_string()],
            exclude: vec![],
        },
        // --- Quality ---
        CustomRule {
            id: "qual-error-ignore".to_string(),
            pattern: r"let\s+_\s*=\s*\w+::\w+\s*\(".to_string(),
            severity: Severity::Minor,
            message: "Error result discarded with `let _ =`. Consider handling the error explicitly.".to_string(),
            languages: vec!["all".to_string()],
            exclude: vec![],
        },
        CustomRule {
            id: "qual-clone".to_string(),
            pattern: r"\.clone\(\)".to_string(),
            severity: Severity::Info,
            message: "Use of `.clone()` detected. Consider borrowing or ownership transfer.".to_string(),
            languages: vec!["rs".to_string()],
            exclude: vec![],
        },
    ]
}

/// Post-match filter for rules that need additional validation after regex match.
/// Returns `true` to suppress a finding that the regex matched but should be ignored.
pub fn post_match_filter(rule_id: &str, line: &str) -> bool {
    match rule_id {
        "sec-hardcoded-url" => {
            let lower = line.to_lowercase();
            lower.contains("http://localhost")
                || lower.contains("http://127.0.0.1")
                || lower.contains("http://0.0.0.0")
                || lower.contains("http://[::1]")
        }
        _ => false,
    }
}
