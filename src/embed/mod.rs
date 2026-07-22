//! Bag-of-tokens embedding engine.
//!
//! Provides two backends:
//!
//! 1. **Hashing trick** (`tokens` module) — zero-dependency, 256-dim, fast but
//!    low-quality.  Suitable for dedup / near-duplicate detection.
//!
//! 2. **Pre-trained nomic-embed-code** (`token_vocab` module) — real 768-dim
//!    embeddings distilled from [nomic-ai/nomic-embed-code](https://huggingface.co/nomic-ai/nomic-embed-code),
//!    compiled into the binary via `include_bytes!` / `include_str!`.
//!    Higher quality at the cost of ~30 MB binary size.
//!
//! Re-exports will be added to this module in Phase 3 when `cora brain`
//! commands are wired up.

// Phase 1: public API not yet consumed by any command.
// Clippy dead_code warnings are expected and will resolve in Phase 3.
#![allow(dead_code)]

pub mod token_vocab;
pub mod tokens;
