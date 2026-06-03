//! Context chain — deterministic cross-file dependency extraction.
//!
//! Enriches LLM review prompts with function definitions, type definitions,
//! and test coverage information extracted from the changed code, without
//! using any LLM tokens.
//!
//! ## Architecture
//!
//! 1. **Extraction** (`extraction.rs`): Extract function calls, type refs,
//!    and imports from changed diff lines using language-aware regex.
//!
//! 2. **Resolution** (`resolver.rs`): Resolve extracted symbols to concrete
//!    file locations (e.g., `use crate::auth::validate` → `src/auth/validate.rs`).
//!
//! 3. **Assembly**: Read file content under a token budget, assemble into a
//!    formatted string for prompt injection.
//!
//! ## Usage
//!
//! ```rust,ignore
//! let chain = context::build_context_chain(
//!     &extracted_symbols,
//!     &config.context_chain,
//!     &project_root,
//!     &config.ignore.files,
//! );
//! if !chain.text.is_empty() {
//!     // Inject chain.text into the review prompt
//! }
//! ```

pub mod extraction;
pub mod resolver;
pub mod types;

use std::path::Path;

use crate::engine::diff_parser::FileChunk;
use types::{ContextChain, ContextConfig};

pub use types::{ContextStats, ExtractedSymbol, SymbolKind};

/// Build the full context chain from parsed diff chunks.
///
/// Convenience entry point that combines extraction and resolution.
/// Returns the assembled context chain ready for prompt injection.
pub fn build_context_chain(
    chunks: &[FileChunk],
    config: &ContextConfig,
    project_root: &Path,
    ignore_patterns: &[String],
) -> ContextChain {
    // Step 1: Extract symbols from changed lines
    let symbols = extraction::extract_symbols_from_diff(chunks);

    if symbols.is_empty() || !config.enabled {
        return ContextChain::default();
    }

    // Step 2: Resolve and assemble under budget
    resolver::build_context_chain(&symbols, config, project_root, ignore_patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> ContextConfig {
        ContextConfig::default()
    }

    #[test]
    fn empty_chunks_returns_empty() {
        let chain = build_context_chain(&[], &default_config(), Path::new("/tmp"), &[]);
        assert!(chain.text.is_empty());
    }

    #[test]
    fn disabled_returns_empty() {
        let config = ContextConfig {
            enabled: false,
            ..default_config()
        };
        let chunks = crate::engine::diff_parser::parse_diff(
            "diff --git a/src/main.rs b/src/main.rs\n--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1,1 +1,2 @@\n fn main() {\n+    foo();\n }\n",
        );
        let chain = build_context_chain(&chunks, &config, Path::new("/tmp"), &[]);
        assert!(chain.text.is_empty());
    }

    #[test]
    fn extracts_from_real_diff() {
        let diff = r#"diff --git a/src/main.rs b/src/main.rs
--- a/src/main.rs
+++ b/src/main.rs
@@ -5,6 +5,8 @@
 fn main() {
     println!("hello");
+    let config = load_config();
+    authenticate(&config);
 }
"#;
        let chunks = crate::engine::diff_parser::parse_diff(diff);
        let chain = build_context_chain(&chunks, &default_config(), Path::new("/nonexistent"), &[]);
        // Should extract symbols but not resolve (no files exist)
        assert_eq!(chain.stats.symbols_extracted, 2);
    }
}
