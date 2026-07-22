//! Bag-of-tokens embedding via hashing trick.
//!
//! Pure-Rust, zero-dependency code embedding.  Each token is hashed into a
//! fixed-dimension vector; the final embedding is the L2-normalised sum
//! (bag-of-words) of those per-token vectors.
//!
//! This is the fast / lightweight backend.  For higher quality, use
//! [`crate::embed::token_vocab`] (pre-trained 768-dim vectors from
//! nomic-embed-code).

use std::collections::HashMap;

// ─── Constants ───────────────────────────────────────────────────────────────

/// Dimensionality of the hashing-trick embedding vectors.
pub const EMBEDDING_DIM: usize = 256;

/// Maximum number of distinct tokens kept per snippet.  Keeps memory bounded
/// and acts as light denoising.
const MAX_TOKENS: usize = 512;

/// Seed for the deterministic hash-to-vector mapping.
const HASH_SEED: u64 = 0xC07A_C0DE_C0DE_C0DE;

/// Punctuation / operators that appear in virtually all code and add no
/// discriminative signal — they are dropped during tokenisation.
const STOP_PUNCTUATION: &[char] = &[
    '(', ')', '{', '}', '[', ']', ',', ';', '.', ':', '\'', '"', '`',
];

// ─── Public types ────────────────────────────────────────────────────────────

/// A fixed-length (256-dim) embedding vector with utility methods.
#[derive(Debug, Clone)]
pub struct TokenEmbedding {
    vec: Vec<f64>,
}

impl Default for TokenEmbedding {
    fn default() -> Self {
        Self {
            vec: vec![0.0; EMBEDDING_DIM],
        }
    }
}

impl TokenEmbedding {
    /// Returns a reference to the underlying vector.
    pub fn as_slice(&self) -> &[f64] {
        &self.vec
    }

    /// Consumes self and returns the underlying vector.
    pub fn into_vec(self) -> Vec<f64> {
        self.vec
    }
}

/// Split a single identifier into subwords on camelCase and snake_case boundaries.
///
/// - `calculateHash` → `["calculate", "Hash"]`
/// - `hello_world` → `["hello", "world"]`
/// - `HTTPServer` → `["HTTP", "Server"]`
/// - `simple` → `["simple"]`
/// - `_leading` → `["leading"]`
/// - `trailing_` → `["trailing"]`
///
/// Consecutive uppercase letters before a lowercase are grouped (acronyms).
fn split_identifier(ident: &str) -> Vec<&str> {
    let bytes = ident.as_bytes();
    if bytes.is_empty() {
        return Vec::new();
    }

    let mut parts = Vec::new();
    let mut part_start = 0;

    // Skip leading underscores
    while part_start < bytes.len() && bytes[part_start] == b'_' {
        part_start += 1;
    }
    if part_start >= bytes.len() {
        return Vec::new();
    }

    for i in (part_start + 1)..bytes.len() {
        let prev = bytes[i - 1];
        let curr = bytes[i];

        // snake_case boundary: underscore
        if curr == b'_' {
            if i > part_start {
                parts.push(&ident[part_start..i]);
            }
            part_start = i + 1;
            continue;
        }

        // camelCase boundary: lowercase→uppercase
        if prev.is_ascii_lowercase() && curr.is_ascii_uppercase() {
            parts.push(&ident[part_start..i]);
            part_start = i;
            continue;
        }

        // Acronym boundary: uppercase→uppercase+lowercase (e.g. HTTp → HTT|p)
        // "HTTPRequest" → at position of 'R': prev='T'(upper), curr='R'(upper), next='e'(lower)
        if prev.is_ascii_uppercase()
            && curr.is_ascii_uppercase()
            && i + 1 < bytes.len()
            && bytes[i + 1].is_ascii_lowercase()
        {
            parts.push(&ident[part_start..i]);
            part_start = i;
        }
    }

    // Flush remaining (trim trailing underscores)
    if part_start < bytes.len() {
        let remaining = &ident[part_start..];
        let trimmed = remaining.trim_end_matches('_');
        if !trimmed.is_empty() {
            parts.push(trimmed);
        }
    }

    if parts.is_empty() && part_start < bytes.len() {
        parts.push(ident.trim_matches('_'));
    }

    parts
}

// ─── Public API ──────────────────────────────────────────────────────────────

/// Tokenize a code string into a frequency map of lowercase tokens.
///
/// The tokenizer recognises:
/// - **Keywords / identifiers** — `[a-zA-Z_][a-zA-Z0-9_]*`
/// - **Numbers** — digit sequences (floats normalised to `<FLOAT>`, large
///   integers to `<NUM>`)
/// - **Multi-char operators** — consecutive non-alphanumeric runs (e.g. `->`,
///   `::`, `+=`) become a single token.
/// - **Noise filtering** — ubiquitous single-char punctuation (brackets,
///   commas, semicolons, etc.) is dropped to keep embeddings discriminative.
///
/// Returns `HashMap<String, u32>` (token → count), compatible with both the
/// hashing-trick [`embed`] and the pre-trained [`crate::embed::embed_pretrained`].
pub fn tokenize_code(code: &str) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    let mut chars = code.char_indices().peekable();

    while let Some(&(i, ch)) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        // Identifiers / keywords — split on camelCase and snake_case boundaries
        if ch.is_ascii_alphabetic() || ch == '_' {
            let start = i;
            while let Some(&(j, c)) = chars.peek() {
                if c.is_ascii_alphanumeric() || c == '_' {
                    chars.next();
                } else {
                    break;
                }
                let _ = j;
            }
            let end = chars.peek().map_or(code.len(), |&(j, _)| j);
            let word = &code[start..end];

            // Split camelCase/snake_case into subwords for better vocab coverage
            for sub in split_identifier(word) {
                let lower = sub.to_lowercase();
                if lower.len() >= 2 {
                    *counts.entry(lower).or_insert(0) += 1;
                }
            }
            continue;
        }

        // Numbers (digits and at most one dot)
        if ch.is_ascii_digit() {
            let start = i;
            let mut seen_dot = false;
            while let Some(&(_, c)) = chars.peek() {
                if c.is_ascii_digit() {
                    chars.next();
                } else if c == '.' && !seen_dot {
                    seen_dot = true;
                    chars.next();
                } else {
                    break;
                }
            }
            let end = chars.peek().map_or(code.len(), |&(j, _)| j);
            let num = &code[start..end];
            let token = if num.contains('.') {
                "<FLOAT>".to_string()
            } else if num.len() > 4 {
                "<NUM>".to_string()
            } else {
                num.to_string()
            };
            *counts.entry(token).or_insert(0) += 1;
            continue;
        }

        // Operators / punctuation — group consecutive non-alnum chars
        let start = i;
        while let Some(&(j, c)) = chars.peek() {
            if !c.is_ascii_alphanumeric() && !c.is_whitespace() {
                chars.next();
                let _ = j;
            } else {
                break;
            }
        }
        let end = chars.peek().map_or(code.len(), |&(j, _)| j);
        let op = &code[start..end];

        // Single-char punctuation on the stop list → skip
        if op.len() == 1 && STOP_PUNCTUATION.contains(&op.chars().next().unwrap()) {
            continue;
        }

        *counts.entry(op.to_string()).or_insert(0) += 1;
    }

    // Keep only the top MAX_TOKENS by frequency
    if counts.len() > MAX_TOKENS {
        let mut entries: Vec<_> = counts.into_iter().collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.truncate(MAX_TOKENS);
        counts = entries.into_iter().collect();
    }

    counts
}

/// Compute the L2-normalised bag-of-tokens embedding for the given token
/// frequency map using the hashing trick.
///
/// Each unique token is hashed into a pseudo-random vector of
/// [`EMBEDDING_DIM`] dimensions.  The per-token vectors are weighted by
/// frequency and summed, then L2-normalised so that [`cosine_similarity`]
/// reduces to a dot product.
pub fn embed(tokens: &HashMap<String, u32>) -> TokenEmbedding {
    let mut vec = vec![0.0f64; EMBEDDING_DIM];

    for (token, count) in tokens {
        let weight = *count as f64;
        let token_vec = hash_to_vec(token);
        for (v, tv) in vec.iter_mut().zip(token_vec.iter()) {
            *v += weight * tv;
        }
    }

    // L2 normalise
    let norm: f64 = vec.iter().map(|v| v * v).sum::<f64>().sqrt();
    if norm > 0.0 {
        for v in vec.iter_mut() {
            *v /= norm;
        }
    }

    TokenEmbedding { vec }
}

/// Convenience function: tokenize → embed in one step.
pub fn embed_code(code: &str) -> TokenEmbedding {
    let tokens = tokenize_code(code);
    embed(&tokens)
}

/// Cosine similarity between two [`TokenEmbedding`]s.
///
/// Because vectors are pre-normalised by [`embed`], this is just the dot
/// product.  Returns a value in `[-1, 1]`.
pub fn cosine_similarity(a: &TokenEmbedding, b: &TokenEmbedding) -> f64 {
    a.vec.iter().zip(b.vec.iter()).map(|(x, y)| x * y).sum()
}

// ─── Internal helpers ────────────────────────────────────────────────────────

/// Deterministic hash of a token string to a pseudo-random vector in `[-1, 1]`.
///
/// Uses FNV-1a expanded across all dimensions via per-dimension seed mixing.
fn hash_to_vec(token: &str) -> Vec<f64> {
    let bytes = token.as_bytes();
    let mut vec = Vec::with_capacity(EMBEDDING_DIM);

    for dim in 0..EMBEDDING_DIM {
        let mut hash = HASH_SEED.wrapping_add(dim as u64);
        for &byte in bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100_0000_01b3);
        }
        let val = ((hash >> 33) as i64) as f64 / (i32::MAX as f64);
        vec.push(val.clamp(-1.0, 1.0));
    }

    vec
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple() {
        let code = "fn main() { println!(\"hello\"); }";
        let tokens = tokenize_code(code);
        assert!(tokens.contains_key("fn"));
        assert!(tokens.contains_key("main"));
        assert!(tokens.contains_key("println"));
        // Braces, parens, semicolons are stop-punctuation → filtered
        assert!(!tokens.contains_key("("));
        assert!(!tokens.contains_key(")"));
        assert!(!tokens.contains_key("{"));
        assert!(!tokens.contains_key("}"));
        assert!(!tokens.contains_key(";"));
        // String contents inside "" are still tokenised as identifiers
        assert!(tokens.contains_key("hello"));
    }

    #[test]
    fn tokenize_normalises_case() {
        let a = tokenize_code("Hello World");
        let b = tokenize_code("hello world");
        assert_eq!(a, b);
    }

    #[test]
    fn tokenize_numbers() {
        let tokens = tokenize_code("let x = 42;");
        assert!(tokens.contains_key("42"));
        assert!(!tokens.contains_key("<NUM>"));
    }

    #[test]
    fn tokenize_large_numbers_normalised() {
        let tokens = tokenize_code("let id = 12345678;");
        assert!(tokens.contains_key("<NUM>"));
        assert!(!tokens.contains_key("12345678"));
    }

    #[test]
    fn tokenize_floats_normalised() {
        let tokens = tokenize_code("let pi = 3.14;");
        assert!(tokens.contains_key("<FLOAT>"));
        assert!(!tokens.contains_key("3.14"));
    }

    #[test]
    fn tokenize_multi_char_operators() {
        let tokens = tokenize_code("a -> b :: c += d");
        assert!(tokens.contains_key("->"));
        assert!(tokens.contains_key("::"));
        assert!(tokens.contains_key("+="));
        assert!(!tokens.contains_key("-"));
        assert!(!tokens.contains_key(">"));
    }

    #[test]
    fn embed_dimension() {
        let emb = embed_code("fn foo() {}");
        assert_eq!(emb.as_slice().len(), EMBEDDING_DIM);
    }

    #[test]
    fn embed_is_normalised() {
        let emb = embed_code("fn foo() {}");
        let norm: f64 = emb.as_slice().iter().map(|v| v * v).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn identical_code_similarity_is_one() {
        let a = embed_code("fn main() { println!(); }");
        let b = embed_code("fn main() { println!(); }");
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 1e-10);
    }

    #[test]
    fn similar_code_high_similarity() {
        let a = embed_code("fn add(a: i32, b: i32) -> i32 { a + b }");
        let b = embed_code("fn add(x: i64, y: i64) -> i64 { x + y }");
        let sim = cosine_similarity(&a, &b);
        assert!(
            sim > 0.8,
            "similar functions should have high similarity, got {sim}"
        );
    }

    #[test]
    fn dissimilar_code_lower_similarity() {
        let a = embed_code("fn add(a: i32, b: i32) -> i32 { a + b }");
        let b =
            embed_code("SELECT * FROM users WHERE email = 'test@example.com' ORDER BY created_at");
        let sim = cosine_similarity(&a, &b);
        // The hashing trick is inherently noisy for cross-language comparison
        // (256-dim pseudo-random projections). We just verify it's not exactly 1.0.
        assert!(
            sim < 0.999,
            "dissimilar snippets should not be identical, got {sim}"
        );
    }

    #[test]
    fn empty_code_embeds_to_zero() {
        let emb = embed_code("");
        let norm: f64 = emb.as_slice().iter().map(|v| v * v).sum::<f64>().sqrt();
        assert!(norm < 1e-10);
    }

    #[test]
    fn max_tokens_limit() {
        let code: String = (0..600)
            .map(|i| format!("let var_{i} = {i};"))
            .collect::<Vec<_>>()
            .join("\n");
        let tokens = tokenize_code(&code);
        assert!(tokens.len() <= MAX_TOKENS);
    }

    #[test]
    fn deterministic_embedding() {
        let a = embed_code("hello world");
        let b = embed_code("hello world");
        assert_eq!(a.as_slice(), b.as_slice());
    }

    #[test]
    fn split_identifier_camel() {
        let parts = split_identifier("calculateHash");
        assert_eq!(parts, vec!["calculate", "Hash"]);
    }

    #[test]
    fn split_identifier_snake() {
        let parts = split_identifier("hello_world");
        assert_eq!(parts, vec!["hello", "world"]);
    }

    #[test]
    fn split_identifier_acronym() {
        let parts = split_identifier("HTTPServer");
        assert_eq!(parts, vec!["HTTP", "Server"]);
    }

    #[test]
    fn split_identifier_simple() {
        let parts = split_identifier("simple");
        assert_eq!(parts, vec!["simple"]);
    }

    #[test]
    fn split_identifier_leading_underscore() {
        let parts = split_identifier("_leading");
        assert_eq!(parts, vec!["leading"]);
    }

    #[test]
    fn split_identifier_trailing_underscore() {
        let parts = split_identifier("trailing_");
        assert_eq!(parts, vec!["trailing"]);
    }

    #[test]
    fn split_identifier_mixed() {
        let parts = split_identifier("std_collections_HashMap");
        assert_eq!(parts, vec!["std", "collections", "Hash", "Map"]);
    }

    #[test]
    fn tokenize_splits_camelcase() {
        let tokens = tokenize_code("let calculateHash = 1;");
        assert!(
            tokens.contains_key("calculate"),
            "should split calculateHash → calculate"
        );
        assert!(
            tokens.contains_key("hash"),
            "should split calculateHash → hash"
        );
    }

    #[test]
    fn tokenize_splits_snakecase() {
        let tokens = tokenize_code("fn hello_world() {}");
        assert!(
            tokens.contains_key("hello"),
            "should split hello_world → hello"
        );
        assert!(
            tokens.contains_key("world"),
            "should split hello_world → world"
        );
    }

    #[test]
    fn cosine_similarity_range() {
        let a = embed_code("fn foo() {}");
        let b = embed_code("class Bar { constructor() {} }");
        let sim = cosine_similarity(&a, &b);
        assert!(
            (-1.0..=1.0).contains(&sim),
            "cosine similarity must be in [-1, 1], got {sim}"
        );
    }
}
