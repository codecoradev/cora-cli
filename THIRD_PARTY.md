# Third-Party Licenses

## nomic-embed-code — Static Token Vectors

**Used in:** `vendored/nomic/code_vectors.bin`, `vendored/nomic/code_tokens.txt`

Static token embeddings extracted from [nomic-embed-code](https://huggingface.co/nomic-ai/nomic-embed-code)
(7B parameter code embedding model, Apache-2.0 license).

### Extraction Process

Token vectors are extracted via full inference from the nomic-embed-code model:
1. Load nomic-embed-code (7B, Qwen2.5-Coder-7B base)
2. Filter vocabulary to ~40K code-relevant alphanumeric tokens
3. Per-token inference → 768-dim float vectors
4. Simulated attention (K=32 neighbors, 3 iterations, α=0.3)
5. Mean centering (anisotropy fix)
6. Int8 quantization (×127 scaling) → ~12MB binary blob

The 7B model is NOT bundled or used at runtime — only the pre-computed static vectors are included.

### Source

- Model: https://huggingface.co/nomic-ai/nomic-embed-code
- License: Apache-2.0
- Vendored from: [DeusData/codebase-memory-mcp](https://github.com/DeusData/codebase-memory-mcp) (MIT license)
  - Original extraction script: `scripts/extract_nomic_vectors.py`
  - Pre-built vectors: `vendored/nomic/code_vectors.bin`, `vendored/nomic/code_tokens.txt`

See `vendored/nomic/LICENSE` and `vendored/nomic/NOTICE` for full license text.