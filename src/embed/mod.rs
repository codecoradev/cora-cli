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

pub mod token_vocab;
pub mod tokens;

pub use token_vocab::{
    embed_pretrained, pretrained_cosine_similarity, verify_binary_format, PretrainedEmbedding,
    PRETRAINED_DIM, VOCAB_SIZE,
};
pub use tokens::{cosine_similarity, embed, embed_code, tokenize_code, TokenEmbedding, EMBEDDING_DIM};