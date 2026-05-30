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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            "best_practice" | "best-practice" | "bestpractice" | "best practice" => IssueType::BestPractice,
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
