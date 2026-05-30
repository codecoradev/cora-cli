use serde::{Deserialize, Serialize};
use std::fmt;

/// Issue severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    Major,
    Minor,
    Info,
}

impl Severity {
    /// Parse from string (case-insensitive)
    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => Severity::Critical,
            "major" => Severity::Major,
            "minor" => Severity::Minor,
            _ => Severity::Info,
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Critical => write!(f, "critical"),
            Severity::Major => write!(f, "major"),
            Severity::Minor => write!(f, "minor"),
            Severity::Info => write!(f, "info"),
        }
    }
}

impl Severity {
    /// Get the label text for this severity.
    pub fn label(&self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::Major => "MAJOR",
            Severity::Minor => "MINOR",
            Severity::Info => "INFO",
        }
    }

    /// Get the icon for this severity.
    pub fn icon(&self) -> &'static str {
        match self {
            Severity::Critical => "🔴",
            Severity::Major => "🟠",
            Severity::Minor => "🟡",
            Severity::Info => "ℹ️",
        }
    }
}

/// Issue type categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueType {
    Security,
    Performance,
    Bug,
    BestPractice,
    Style,
    Suggestion,
}

impl fmt::Display for IssueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueType::Security => write!(f, "security"),
            IssueType::Performance => write!(f, "performance"),
            IssueType::Bug => write!(f, "bug"),
            IssueType::BestPractice => write!(f, "best_practice"),
            IssueType::Style => write!(f, "style"),
            IssueType::Suggestion => write!(f, "suggestion"),
        }
    }
}

impl IssueType {
    /// Parse from string with lenient matching (accepts plural forms, etc.)
    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "security" | "sec" => IssueType::Security,
            "performance" | "perf" => IssueType::Performance,
            "bug" | "bugs" => IssueType::Bug,
            "best_practice" | "best-practice" | "bestpractice" | "best practice" => {
                IssueType::BestPractice
            }
            "style" | "formatting" => IssueType::Style,
            "suggestion" | "info" => IssueType::Suggestion,
            _ => IssueType::Suggestion, // fallback
        }
    }
}

/// A single review issue found in code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewIssue {
    pub file: String,
    #[serde(default)]
    pub line: Option<u32>,
    pub severity: Severity,
    /// Issue type/category — stored as string since LLM output varies.
    /// Common values: security, performance, bug, best_practice, style, suggestion
    #[serde(rename = "type", alias = "issue_type")]
    pub issue_type: Option<String>,
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub suggested_fix: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── Severity::from_str_lossy ───

    #[test]
    fn severity_critical() {
        assert_eq!(Severity::from_str_lossy("critical"), Severity::Critical);
    }

    #[test]
    fn severity_major() {
        assert_eq!(Severity::from_str_lossy("major"), Severity::Major);
    }

    #[test]
    fn severity_minor() {
        assert_eq!(Severity::from_str_lossy("minor"), Severity::Minor);
    }

    #[test]
    fn severity_info() {
        assert_eq!(Severity::from_str_lossy("info"), Severity::Info);
    }

    #[test]
    fn severity_case_insensitive() {
        assert_eq!(Severity::from_str_lossy("CRITICAL"), Severity::Critical);
        assert_eq!(Severity::from_str_lossy("Major"), Severity::Major);
        assert_eq!(Severity::from_str_lossy("MINOR"), Severity::Minor);
        assert_eq!(Severity::from_str_lossy("Info"), Severity::Info);
    }

    #[test]
    fn severity_unknown_falls_back_to_info() {
        assert_eq!(Severity::from_str_lossy("unknown"), Severity::Info);
        assert_eq!(Severity::from_str_lossy(""), Severity::Info);
        assert_eq!(Severity::from_str_lossy("foobar"), Severity::Info);
    }

    // ─── Severity ordering ───

    #[test]
    fn severity_ordering() {
        // Ord is by discriminant: Critical(0) < Major(1) < Minor(2) < Info(3)
        assert!(Severity::Critical < Severity::Major);
        assert!(Severity::Major < Severity::Minor);
        assert!(Severity::Minor < Severity::Info);
    }

    // ─── Severity display ───

    #[test]
    fn severity_display() {
        assert_eq!(format!("{}", Severity::Critical), "critical");
        assert_eq!(format!("{}", Severity::Major), "major");
        assert_eq!(format!("{}", Severity::Minor), "minor");
        assert_eq!(format!("{}", Severity::Info), "info");
    }

    #[test]
    fn severity_label() {
        assert_eq!(Severity::Critical.label(), "CRITICAL");
        assert_eq!(Severity::Major.label(), "MAJOR");
        assert_eq!(Severity::Minor.label(), "MINOR");
        assert_eq!(Severity::Info.label(), "INFO");
    }

    #[test]
    fn severity_icon() {
        assert!(!Severity::Critical.icon().is_empty());
        assert!(!Severity::Major.icon().is_empty());
        assert!(!Severity::Minor.icon().is_empty());
        assert!(!Severity::Info.icon().is_empty());
    }

    // ─── IssueType::from_str_lossy ───

    #[test]
    fn issue_type_security() {
        assert_eq!(IssueType::from_str_lossy("security"), IssueType::Security);
    }

    #[test]
    fn issue_type_sec_alias() {
        assert_eq!(IssueType::from_str_lossy("sec"), IssueType::Security);
    }

    #[test]
    fn issue_type_performance() {
        assert_eq!(IssueType::from_str_lossy("performance"), IssueType::Performance);
    }

    #[test]
    fn issue_type_perf_alias() {
        assert_eq!(IssueType::from_str_lossy("perf"), IssueType::Performance);
    }

    #[test]
    fn issue_type_bug() {
        assert_eq!(IssueType::from_str_lossy("bug"), IssueType::Bug);
    }

    #[test]
    fn issue_type_bugs_alias() {
        assert_eq!(IssueType::from_str_lossy("bugs"), IssueType::Bug);
    }

    #[test]
    fn issue_type_best_practice_variants() {
        assert_eq!(IssueType::from_str_lossy("best_practice"), IssueType::BestPractice);
        assert_eq!(IssueType::from_str_lossy("best-practice"), IssueType::BestPractice);
        assert_eq!(IssueType::from_str_lossy("bestpractice"), IssueType::BestPractice);
        assert_eq!(IssueType::from_str_lossy("best practice"), IssueType::BestPractice);
    }

    #[test]
    fn issue_type_style() {
        assert_eq!(IssueType::from_str_lossy("style"), IssueType::Style);
    }

    #[test]
    fn issue_type_formatting_alias() {
        assert_eq!(IssueType::from_str_lossy("formatting"), IssueType::Style);
    }

    #[test]
    fn issue_type_suggestion() {
        assert_eq!(IssueType::from_str_lossy("suggestion"), IssueType::Suggestion);
    }

    #[test]
    fn issue_type_info_alias() {
        assert_eq!(IssueType::from_str_lossy("info"), IssueType::Suggestion);
    }

    #[test]
    fn issue_type_unknown_falls_back() {
        assert_eq!(IssueType::from_str_lossy("xyz"), IssueType::Suggestion);
        assert_eq!(IssueType::from_str_lossy(""), IssueType::Suggestion);
    }

    // ─── IssueType display ───

    #[test]
    fn issue_type_display() {
        assert_eq!(format!("{}", IssueType::Security), "security");
        assert_eq!(format!("{}", IssueType::Performance), "performance");
        assert_eq!(format!("{}", IssueType::Bug), "bug");
        assert_eq!(format!("{}", IssueType::BestPractice), "best_practice");
        assert_eq!(format!("{}", IssueType::Style), "style");
        assert_eq!(format!("{}", IssueType::Suggestion), "suggestion");
    }

    // ─── LLMConfig::default ───

    #[test]
    fn llm_config_default() {
        let cfg = LLMConfig::default();
        assert!(cfg.api_key.is_empty());
        assert_eq!(cfg.base_url, "https://api.openai.com/v1");
        assert_eq!(cfg.model, "gpt-4o-mini");
        assert_eq!(cfg.provider, "openai");
    }

    // ─── TokenUsage::default ───

    #[test]
    fn token_usage_default() {
        let usage = TokenUsage::default();
        assert_eq!(usage.input_tokens, 0);
        assert_eq!(usage.output_tokens, 0);
        assert!((usage.estimated_cost_usd - 0.0).abs() < f64::EPSILON);
    }

    // ─── Exit codes ───

    #[test]
    fn exit_codes_are_correct() {
        assert_eq!(EXIT_OK, 0);
        assert_eq!(EXIT_ERROR, 1);
        assert_eq!(EXIT_BLOCKED, 2);
        assert_eq!(EXIT_AUTH_ERROR, 3);
        assert!(EXIT_OK < EXIT_ERROR);
        assert!(EXIT_ERROR < EXIT_BLOCKED);
        assert!(EXIT_BLOCKED < EXIT_AUTH_ERROR);
    }

    // ─── Constants ───

    #[test]
    fn max_diff_size() {
        assert_eq!(MAX_DIFF_SIZE, 50 * 1024);
    }

    #[test]
    fn max_scan_batch_files() {
        assert_eq!(MAX_SCAN_BATCH_FILES, 20);
    }

    #[test]
    fn max_scan_batch_chars() {
        assert_eq!(MAX_SCAN_BATCH_CHARS, 80_000);
    }

    // ─── ReviewIssue serde round-trip ───

    #[test]
    fn review_issue_roundtrip() {
        let issue = ReviewIssue {
            file: "src/main.rs".to_string(),
            line: Some(42),
            severity: Severity::Critical,
            issue_type: Some("security".to_string()),
            title: "SQL Injection".to_string(),
            body: "Details here".to_string(),
            suggested_fix: Some("Use params".to_string()),
        };
        let json = serde_json::to_string(&issue).unwrap();
        let back: ReviewIssue = serde_json::from_str(&json).unwrap();
        assert_eq!(back.file, issue.file);
        assert_eq!(back.line, issue.line);
        assert_eq!(back.severity, issue.severity);
        assert_eq!(back.issue_type, issue.issue_type);
        assert_eq!(back.title, issue.title);
        assert_eq!(back.body, issue.body);
        assert_eq!(back.suggested_fix, issue.suggested_fix);
    }

    #[test]
    fn review_issue_with_type_alias_deserializes() {
        let json = r#"{"file":"a.rs","severity":"info","type":"security","title":"T","body":"B"}"#;
        let issue: ReviewIssue = serde_json::from_str(json).unwrap();
        assert_eq!(issue.issue_type.as_deref(), Some("security"));
    }

    #[test]
    fn review_issue_with_issue_type_deserializes() {
        let json = r#"{"file":"a.rs","severity":"info","issue_type":"performance","title":"T","body":"B"}"#;
        let issue: ReviewIssue = serde_json::from_str(json).unwrap();
        assert_eq!(issue.issue_type.as_deref(), Some("performance"));
    }
}

/// Token usage tracking
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub estimated_cost_usd: f64,
}

/// Response from a code review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResponse {
    pub issues: Vec<ReviewIssue>,
    pub summary: String,
    pub tokens_used: Option<TokenUsage>,
    pub should_block: bool,
}

/// Response from a full project scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResponse {
    pub issues: Vec<ReviewIssue>,
    pub summary: String,
    pub files_scanned: usize,
    #[serde(default)]
    pub lines_scanned: usize,
    pub tokens_used: Option<TokenUsage>,
    pub should_block: bool,
}

/// LLM provider configuration
#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub provider: String,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4o-mini".to_string(),
            provider: "openai".to_string(),
        }
    }
}

/// CLI exit codes
pub const EXIT_OK: i32 = 0;
pub const EXIT_ERROR: i32 = 1;
pub const EXIT_BLOCKED: i32 = 2;
pub const EXIT_AUTH_ERROR: i32 = 3;

/// Maximum diff size in bytes (50KB by default)
pub const MAX_DIFF_SIZE: usize = 50 * 1024;

/// Maximum files per scan batch
pub const MAX_SCAN_BATCH_FILES: usize = 20;

/// Maximum characters per scan batch
pub const MAX_SCAN_BATCH_CHARS: usize = 80_000;
