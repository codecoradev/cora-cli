//! Context chain types — symbols, resolved references, and configuration.
//!
//! The context chain enriches LLM review prompts with cross-file dependency
//! information extracted deterministically from changed code, zero LLM tokens.

use serde::{Deserialize, Serialize};

/// Configuration for the context chain feature.
///
/// Lives inside `Config` and is also deserializable from `.cora.yaml`
/// under the `review.context_chain` key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    /// Whether the context chain is enabled.
    /// When `false`, no cross-file context is collected.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Maximum number of *tokens* of additional context to inject.
    /// The budget is enforced via a rough `chars / 4` estimation.
    /// Default: 3000 tokens ≈ 12 KB of code.
    #[serde(default = "default_max_context_tokens")]
    pub max_context_tokens: usize,

    /// Maximum levels of dependency following.
    /// Depth 1 = resolve symbols from changed lines.
    /// Depth 2 = also resolve symbols referenced in depth-1 results.
    /// Depth 3+ follows further. Default: 1 (only direct references).
    #[serde(default = "default_follow_depth")]
    pub follow_depth: u32,

    /// Whether to auto-resolve test files via naming convention.
    /// For Rust: `src/foo.rs` → `tests/foo_test.rs` or `tests/foo_test/`.
    /// Default: true.
    #[serde(default = "default_true")]
    pub include_tests: bool,
}

fn default_true() -> bool {
    true
}

fn default_max_context_tokens() -> usize {
    3000
}

fn default_follow_depth() -> u32 {
    1
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_context_tokens: 3000,
            follow_depth: 1,
            include_tests: true,
        }
    }
}

/// A symbol extracted from source code in changed lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    /// Function or method call (e.g., `validate_token()`, `self.handle()`).
    FunctionCall(String),
    /// Type or struct reference (e.g., `CryptoConfig`, `HashMap<K, V>`).
    TypeRef(String),
    /// Import/use statement pointing to a module path
    /// (e.g., `use crate::engine::scanner`).
    Import(String),
}

impl std::fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolKind::FunctionCall(name) => write!(f, "fn {name}"),
            SymbolKind::TypeRef(name) => write!(f, "type {name}"),
            SymbolKind::Import(path) => write!(f, "import {path}"),
        }
    }
}

/// A single symbol extracted from a changed line, with source location.
#[derive(Debug, Clone)]
pub struct ExtractedSymbol {
    /// What kind of symbol this is.
    pub kind: SymbolKind,
    /// The file path where the symbol was found.
    pub file: String,
    /// Line number in the file (1-indexed).
    pub line: u32,
    /// The raw text that matched (for debugging).
    pub raw: String,
}

/// A resolved context entry — a (file, line range, symbol) tuple ready
/// for reading and injection into the prompt.
#[derive(Debug, Clone)]
pub struct ContextEntry {
    /// Resolved file path (relative to project root).
    pub file: String,
    /// Start line (1-indexed, inclusive).
    pub line_start: u32,
    /// End line (1-indexed, inclusive).
    pub line_end: u32,
    /// Human-readable label (e.g., `fn validate_token`, `struct CryptoConfig`).
    pub label: String,
    /// Priority tier for budget allocation.
    pub priority: ContextPriority,
}

/// Priority of a context entry — controls which entries are included
/// first when the token budget is limited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContextPriority {
    /// Function/method definitions — highest priority.
    FunctionDef = 0,
    /// Type/struct definitions.
    TypeDef = 1,
    /// Test functions — lowest priority.
    Test = 2,
}

/// Statistics from a context chain build, useful for logging / progress events.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContextStats {
    /// Number of symbols extracted from changed lines.
    pub symbols_extracted: usize,
    /// Number of symbols successfully resolved to file locations.
    pub symbols_resolved: usize,
    /// Number of context entries read from disk.
    pub entries_read: usize,
    /// Estimated token count of the assembled context.
    pub estimated_tokens: usize,
    /// Whether the budget was hit (some entries were dropped).
    pub budget_hit: bool,
}

/// The fully assembled context chain, ready for prompt injection.
#[derive(Debug, Clone, Default)]
pub struct ContextChain {
    /// Formatted text to inject into the prompt.
    pub text: String,
    /// Build statistics.
    pub stats: ContextStats,
}

/// Rough token estimation: ~4 characters per token.
/// This is a heuristic — consistent across runs, which matters more than accuracy.
pub fn estimate_tokens(text: &str) -> usize {
    text.len() / 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_config_default() {
        let cfg = ContextConfig::default();
        assert!(cfg.enabled);
        assert_eq!(cfg.max_context_tokens, 3000);
        assert_eq!(cfg.follow_depth, 1);
        assert!(cfg.include_tests);
    }

    #[test]
    fn context_config_deserialize_partial() {
        let yaml = "enabled: false\nmax_context_tokens: 5000\n";
        let cfg: ContextConfig = serde_yaml_ng::from_str(yaml).unwrap();
        assert!(!cfg.enabled);
        assert_eq!(cfg.max_context_tokens, 5000);
        // Defaults should fill in for missing fields
        assert_eq!(cfg.follow_depth, 1);
        assert!(cfg.include_tests);
    }

    #[test]
    fn context_config_deserialize_full() {
        let yaml =
            "enabled: true\nmax_context_tokens: 2000\nfollow_depth: 3\ninclude_tests: false\n";
        let cfg: ContextConfig = serde_yaml_ng::from_str(yaml).unwrap();
        assert!(cfg.enabled);
        assert_eq!(cfg.max_context_tokens, 2000);
        assert_eq!(cfg.follow_depth, 3);
        assert!(!cfg.include_tests);
    }

    #[test]
    fn estimate_tokens_basic() {
        // 40 chars → ~10 tokens
        assert_eq!(estimate_tokens("hello world this is a test string xyz123"), 10);
    }

    #[test]
    fn estimate_tokens_empty() {
        assert_eq!(estimate_tokens(""), 0);
    }

    #[test]
    fn estimate_tokens_short() {
        // 3 chars → 0 tokens (integer division)
        assert_eq!(estimate_tokens("abc"), 0);
    }

    #[test]
    fn symbol_kind_display() {
        let fc = format!("{}", SymbolKind::FunctionCall("foo".into()));
        assert_eq!(fc, "fn foo");
        let tr = format!("{}", SymbolKind::TypeRef("Bar".into()));
        assert_eq!(tr, "type Bar");
        let im = format!("{}", SymbolKind::Import("crate::mod".into()));
        assert_eq!(im, "import crate::mod");
    }

    #[test]
    fn context_priority_ordering() {
        // FunctionDef < TypeDef < Test (Ord — smaller = higher priority)
        assert!(ContextPriority::FunctionDef < ContextPriority::TypeDef);
        assert!(ContextPriority::TypeDef < ContextPriority::Test);
    }
}
