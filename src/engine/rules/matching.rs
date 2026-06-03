/// Matching engine for rules against diff lines.
use tracing::debug;

use super::types::CustomRule;

/// Check if a rule applies to the given file language.
pub fn matches_language(rule: &CustomRule, file_lang: &str) -> bool {
    rule.languages.iter().any(|lang| {
        let lang_lower = lang.to_lowercase();
        lang_lower == "all" || lang_lower == file_lang
    })
}

/// Check if a file path should be excluded from a rule based on glob patterns.
pub fn matches_exclude(rule: &CustomRule, file_path: &str) -> bool {
    // Also check built-in default exclude paths
    let all_excludes: Vec<&str> = rule
        .exclude
        .iter()
        .map(String::as_str)
        .chain(DEFAULT_EXCLUDE_PATHS.iter().copied())
        .collect();

    for pattern in &all_excludes {
        // Simple glob matching: treat * as any chars, support directory prefixes
        if glob_matches(pattern, file_path) {
            return true;
        }
    }

    false
}

/// Check if a rule's regex pattern matches a line of code.
pub fn match_rule_against_line(rule: &CustomRule, line: &str) -> bool {
    match regex::Regex::new(&rule.pattern) {
        Ok(re) => re.is_match(line),
        Err(e) => {
            debug!(rule_id = %rule.id, error = %e, "invalid regex in rule, skipping");
            false
        }
    }
}

/// Simple glob matching: supports `*` wildcard and directory prefix checks.
fn glob_matches(pattern: &str, path: &str) -> bool {
    // Handle "*_test.*" style patterns
    if pattern.starts_with('*') && pattern.contains('.') {
        // e.g., "*_test.*" — match files ending with "_test.something"
        let suffix = &pattern[1..]; // "_test.*"
        if let Some(dot_idx) = suffix.find('.') {
            let name_part = &suffix[..dot_idx]; // "_test"
            let ext_part = &suffix[dot_idx..]; // ".*"
            let file_name = path.rsplit('/').next().unwrap_or(path);
            if file_name.ends_with(name_part) {
                let remaining = &file_name[file_name.len() - name_part.len()..];
                if remaining == name_part {
                    // Check extension
                    if ext_part == ".*" {
                        return true;
                    }
                    if let Some(file_ext) = file_name.rsplit_once(name_part).map(|(_, ext)| ext) {
                        if file_ext.starts_with('.')
                            && ext_part
                                .strip_prefix('.')
                                .is_some_and(|e| file_ext.ends_with(e))
                        {
                            return true;
                        }
                    }
                }
            }
        }
    }

    // Handle "dir/" prefix patterns (directory-based exclusion)
    if pattern.ends_with('/') {
        return path.starts_with(pattern) || path.contains(&pattern.to_string());
    }

    // Handle "**" glob in patterns like "tests/**"
    if pattern.contains("**") {
        let prefix = pattern.split("**").next().unwrap_or("");
        return path.starts_with(prefix);
    }

    // Handle simple substring patterns
    if pattern.starts_with('*') && pattern.ends_with('*') {
        let inner = &pattern[1..pattern.len() - 1];
        return path.contains(inner);
    }

    // Exact or prefix match
    path == pattern || path.starts_with(&format!("{pattern}/"))
}

/// Default paths to exclude from rule matching (test directories, fixtures, etc.).
const DEFAULT_EXCLUDE_PATHS: &[&str] = &[
    "tests/",
    "test/",
    "__tests__/",
    "spec/",
    "fixtures/",
    "examples/",
    // File patterns handled in glob_matches: *_test.*, *_spec.*
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::Severity;

    fn make_rule(pattern: &str, languages: &[&str], exclude: &[&str]) -> CustomRule {
        CustomRule {
            id: "test-rule".to_string(),
            pattern: pattern.to_string(),
            severity: Severity::Minor,
            message: "test".to_string(),
            languages: languages.iter().map(|s| s.to_string()).collect(),
            exclude: exclude.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn matches_language_all() {
        let rule = make_rule("test", &["all"], &[]);
        assert!(matches_language(&rule, "rs"));
        assert!(matches_language(&rule, "py"));
        assert!(matches_language(&rule, "js"));
    }

    #[test]
    fn matches_language_specific() {
        let rule = make_rule("test", &["rs"], &[]);
        assert!(matches_language(&rule, "rs"));
        assert!(!matches_language(&rule, "py"));
    }

    #[test]
    fn matches_language_case_insensitive() {
        let rule = make_rule("test", &["RS"], &[]);
        assert!(matches_language(&rule, "rs"));
    }

    #[test]
    fn matches_exclude_test_dir() {
        let rule = make_rule("test", &["all"], &[]);
        assert!(matches_exclude(&rule, "tests/main.rs"));
        assert!(matches_exclude(&rule, "test/helper.py"));
    }

    #[test]
    fn not_excluded_src_dir() {
        let rule = make_rule("test", &["all"], &[]);
        assert!(!matches_exclude(&rule, "src/main.rs"));
    }

    #[test]
    fn matches_exclude_custom() {
        let rule = make_rule("test", &["all"], &["vendor/"]);
        assert!(matches_exclude(&rule, "vendor/lib.rs"));
    }

    #[test]
    fn match_rule_against_line_simple() {
        let rule = make_rule(r"println!", &["all"], &[]);
        assert!(match_rule_against_line(&rule, "println!(\"hello\")"));
        assert!(!match_rule_against_line(&rule, "let x = 1;"));
    }

    #[test]
    fn match_rule_invalid_regex_returns_false() {
        let rule = make_rule(r"(?P<invalid", &["all"], &[]);
        assert!(!match_rule_against_line(&rule, "anything"));
    }

    #[test]
    fn glob_matches_star_test_star() {
        assert!(glob_matches("*_test.*", "src/helper_test.rs"));
        assert!(glob_matches("*_test.*", "foo_test.js"));
    }

    #[test]
    fn glob_matches_dir_prefix() {
        assert!(glob_matches("tests/**", "tests/integration/main.rs"));
        assert!(glob_matches("spec/", "spec/unit/test.rs"));
    }

    #[test]
    fn glob_matches_custom_exclude() {
        assert!(glob_matches("vendor/**", "vendor/lib.rs"));
    }
}
