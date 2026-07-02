/// Rule types used by the rule engine.
use serde::{Deserialize, Serialize};

use crate::engine::Severity;

/// Runtime configuration for the rule engine (lives on `Config`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesConfig {
    /// Whether the rule engine is enabled.
    pub enabled: bool,
    /// Maximum number of findings to report per review (prevents noisy output).
    pub max_findings: usize,
    /// User-defined custom rules, merged with built-in rules.
    pub custom_rules: Vec<CustomRule>,
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_findings: 5,
            custom_rules: Vec::new(),
        }
    }
}

/// A user-defined or built-in rule definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomRule {
    /// Unique rule identifier (e.g., `"sec-hardcoded-secret"`).
    pub id: String,
    /// Regex pattern to match against diff lines.
    pub pattern: String,
    /// Severity of findings from this rule.
    pub severity: Severity,
    /// Human-readable description used as the finding body.
    pub message: String,
    /// Languages this rule applies to. `["all"]` means all languages.
    pub languages: Vec<String>,
    /// Glob patterns for file paths to exclude from this rule.
    pub exclude: Vec<String>,
    /// Pre-compiled regex for the pattern (populated after rule assembly).
    #[serde(skip, default)]
    pub compiled_pattern: Option<regex::Regex>,
}

impl CustomRule {
    /// Compile the rule's pattern into a cached regex, if not already compiled.
    /// Returns `true` if compilation succeeded, `false` if the pattern is invalid.
    pub fn ensure_compiled(&mut self) -> bool {
        if self.compiled_pattern.is_some() {
            return true;
        }
        match regex::Regex::new(&self.pattern) {
            Ok(re) => {
                self.compiled_pattern = Some(re);
                true
            }
            Err(_) => false,
        }
    }
}

/// A single finding produced by a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleFinding {
    /// The rule that produced this finding.
    pub rule_id: String,
    /// File path where the finding was located.
    pub file: String,
    /// Line number in the new file.
    pub line: u32,
    /// Severity of this finding.
    pub severity: Severity,
    /// Short title.
    pub title: String,
    /// Detailed description.
    pub body: String,
}
