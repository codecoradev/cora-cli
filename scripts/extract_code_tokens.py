#!/usr/bin/env python3
"""
Extract token embeddings from nomic-embed-code (7B) for static lookup table.

Adapted from CBM's extract_nomic_vectors.py for cora-code.
Outputs Rust-compatible format (code_tokens.rs + code_vectors.bin).

Usage:
    pip install torch transformers numpy
    python3 scripts/extract_code_tokens.py [--output-dir vendored/nomic]

Output:
    code_vectors.bin — [int32 count][int32 dim] + count×dim int8
    code_tokens.txt  — one token per line

One-time extraction. ~2-3h on GPU, ~6-10h on CPU (float16, ~14GB RAM).
"""

import argparse
import os
import re
import struct
import sys
import time
from pathlib import Path

import numpy as np
import torch

# Parallelize CPU inference across all cores BEFORE any torch ops
NUM_THREADS = min(os.cpu_count() * 2, 12)
torch.set_num_threads(NUM_THREADS)
torch.set_num_interop_threads(max(NUM_THREADS // 2, 1))
os.environ.setdefault("OMP_NUM_THREADS", str(NUM_THREADS))
os.environ.setdefault("MKL_NUM_THREADS", str(NUM_THREADS))

from transformers import AutoModel, AutoTokenizer

# ── Configuration ──────────────────────────────────────────────────────

MODEL_NAME = "nomic-ai/nomic-embed-code"
OUTPUT_DIM = 768
SIM_ATTENTION_K = 32
SIM_ATTENTION_ITERS = 3
SIM_ATTENTION_ALPHA = 0.3
BATCH_SIZE = 32
CHECKPOINT_EVERY = 500


# ── Token filtering ───────────────────────────────────────────────────

def is_code_relevant(token_str: str) -> bool:
    """Filter vocabulary to code-relevant tokens."""
    s = token_str.strip()
    if not s:
        return False
    clean = s.lstrip("\u0120\u2581")  # Ġ, ▁
    if not clean:
        return False
    if clean.startswith("<") and clean.endswith(">"):
        return False
    if clean.startswith("[") and clean.endswith("]"):
        return False
    inner = clean.strip("_")
    if not inner:
        return False
    if not re.match(r'^[a-zA-Z][a-zA-Z0-9_]*$', inner):
        return False
    if len(inner) < 2:
        return False
    return True


def clean_token(token_str: str) -> str:
    """Normalize a BPE token to the form our runtime tokenizer produces."""
    s = token_str.strip()
    s = s.lstrip("\u0120\u2581")
    s = s.strip("_")
    s = s.lower()
    return s


# ── Simulated attention ──────────────────────────────────────────────

def simulated_attention(vectors: np.ndarray, k: int, iterations: int, alpha: float) -> np.ndarray:
    """Apply simulated self-attention."""
    n, d = vectors.shape
    result = vectors.copy()
    for iteration in range(iterations):
        t0 = time.time()
        chunk_size = 2048
        new_result = np.zeros_like(result)
        for i in range(0, n, chunk_size):
            end = min(i + chunk_size, n)
            chunk = result[i:end]
            sims = chunk @ result.T
            for j in range(end - i):
                global_idx = i + j
                sim_row = sims[j].copy()
                sim_row[global_idx] = -1.0
                if k < n - 1:
                    top_k_idx = np.argpartition(sim_row, -k)[-k:]
                else:
                    top_k_idx = np.arange(n)
                top_k_idx = top_k_idx[top_k_idx != global_idx]
                neighbor_mean = result[top_k_idx].mean(axis=0)
                blended = (1 - alpha) * result[global_idx] + alpha * neighbor_mean
                norm = np.linalg.norm(blended)
                if norm > 1e-8:
                    blended /= norm
                new_result[global_idx] = blended
        result = new_result
        elapsed = time.time() - t0
        print(f"  sim-attention iter {iteration + 1}/{iterations}: {elapsed:.1f}s")
    return result


# ── Extraction ───────────────────────────────────────────────────────

def extract_embeddings(model, tokenizer, tokens: list, device: str,
                       batch_size: int = 64, checkpoint_path: str = None) -> np.ndarray:
    """Run full model inference on each token string. Returns (N, D) float32."""
    start_idx = 0
    all_vecs = []
    if checkpoint_path and os.path.exists(checkpoint_path):
        data = np.load(checkpoint_path)
        all_vecs = list(data["vectors"])
        start_idx = len(all_vecs)
        print(f"  resuming from checkpoint: {start_idx}/{len(tokens)} tokens")

    model.eval()
    total = len(tokens)
    t0 = time.time()
    with torch.no_grad():
        for batch_start in range(start_idx, total, batch_size):
            batch_end = min(batch_start + batch_size, total)
            batch_tokens = tokens[batch_start:batch_end]
            texts = [f"search_query: {t}" for t in batch_tokens]
            encoded = tokenizer(
                texts, padding=True, truncation=True, max_length=64, return_tensors="pt"
            ).to(device)
            outputs = model(**encoded)
            attention_mask = encoded["attention_mask"]
            token_embeddings = outputs.last_hidden_state
            input_mask_expanded = (
                attention_mask.unsqueeze(-1).expand(token_embeddings.size()).float()
            )
            sum_embeddings = torch.sum(token_embeddings * input_mask_expanded, dim=1)
            sum_mask = torch.clamp(input_mask_expanded.sum(dim=1), min=1e-9)
            mean_pooled = sum_embeddings / sum_mask
            if mean_pooled.shape[1] > OUTPUT_DIM:
                mean_pooled = mean_pooled[:, :OUTPUT_DIM]
            mean_pooled = torch.nn.functional.normalize(mean_pooled, p=2, dim=1)
            vecs = mean_pooled.cpu().numpy()
            all_vecs.extend(vecs)
            done = batch_end
            elapsed = time.time() - t0
            rate = (done - start_idx) / elapsed if elapsed > 0 else 0
            eta = (total - done) / rate if rate > 0 else 0
            print(f"  [{done:>6}/{total}] {rate:.1f} tok/s ETA {eta / 60:.0f}m", flush=True)
            if checkpoint_path and (done % CHECKPOINT_EVERY < batch_size):
                np.savez_compressed(checkpoint_path, vectors=np.array(all_vecs, dtype=np.float32))
    print()
    return np.array(all_vecs, dtype=np.float32)


# ── Output generation ────────────────────────────────────────────────

def write_bin(path: str, vectors: np.ndarray, dim: int):
    """Write binary blob: [int32 count][int32 dim] + count×dim int8."""
    n = vectors.shape[0]
    quantized = np.clip(np.round(vectors * 127.0), -127, 127).astype(np.int8)
    with open(path, "wb") as f:
        f.write(struct.pack("<ii", n, dim))
        f.write(quantized.tobytes())
    print(f"  {path}: written ({os.path.getsize(path) / (1024*1024):.1f} MB)")


def write_tokens_txt(path: str, tokens: list):
    """Write one token per line."""
    with open(path, "w") as f:
        for t in tokens:
            f.write(t + "\n")
    print(f"  {path}: written ({len(tokens)} tokens)")


# ── Main ─────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description="Extract nomic-embed-code token embeddings for cora-code")
    parser.add_argument("--output-dir", default="vendored/nomic")
    parser.add_argument("--device", default=None)
    parser.add_argument("--skip-attention", action="store_true")
    parser.add_argument("--batch-size", type=int, default=BATCH_SIZE)
    args = parser.parse_args()

    if args.device:
        device = args.device
    elif torch.cuda.is_available():
        device = "cuda"
    else:
        device = "cpu"

    sys.stdout.reconfigure(line_buffering=True)
    print(f"device={device}")
    print(f"threads={torch.get_num_threads()}")
    print(f"model={MODEL_NAME}")
    print(f"output_dim={OUTPUT_DIM}")
    print()

    out_dir = Path(args.output_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    checkpoint_path = str(out_dir / "checkpoint.npz")

    # Step 1: Load model + tokenizer
    print("step 1: loading model + tokenizer...")
    t0 = time.time()
    tokenizer = AutoTokenizer.from_pretrained(MODEL_NAME, trust_remote_code=True)
    model = AutoModel.from_pretrained(
        MODEL_NAME, trust_remote_code=True, dtype=torch.float16,
        low_cpu_mem_usage=True,
    )
    model = model.to(device)
    print(f"  loaded in {time.time() - t0:.1f}s")
    print(f"  hidden_size={model.config.hidden_size}")
    print(f"  vocab_size={tokenizer.vocab_size}")
    print()

    # Step 2: Filter vocabulary
    print("step 2: filtering vocabulary to code-relevant tokens...")
    vocab = tokenizer.get_vocab()
    print(f"  raw vocabulary: {len(vocab)} tokens")
    seen = set()
    filtered_tokens = []
    for tok_str, tok_id in sorted(vocab.items(), key=lambda x: x[1]):
        if not is_code_relevant(tok_str):
            continue
        clean = clean_token(tok_str)
        if not clean or clean in seen:
            continue
        if len(clean) < 2:
            continue
        seen.add(clean)
        filtered_tokens.append(clean)
    filtered_tokens.sort()
    print(f"  code-relevant (deduplicated): {len(filtered_tokens)} tokens")
    print()

    # Step 3: Extract embeddings
    print(f"step 3: extracting embeddings ({len(filtered_tokens)} tokens)...")
    t0 = time.time()
    vectors = extract_embeddings(model, tokenizer, filtered_tokens, device,
                                 batch_size=args.batch_size, checkpoint_path=checkpoint_path)
    elapsed = time.time() - t0
    print(f"  extracted {vectors.shape[0]} vectors × {vectors.shape[1]}d in {elapsed:.0f}s")

    if vectors.shape[1] > OUTPUT_DIM:
        vectors = vectors[:, :OUTPUT_DIM]
    norms = np.linalg.norm(vectors, axis=1, keepdims=True)
    norms = np.maximum(norms, 1e-8)
    vectors = vectors / norms

    # Mean-center
    mean_vec = vectors.mean(axis=0)
    print(f"  mean vector norm before centering: {np.linalg.norm(mean_vec):.4f}")
    vectors = vectors - mean_vec
    norms = np.linalg.norm(vectors, axis=1, keepdims=True)
    norms = np.maximum(norms, 1e-8)
    vectors = vectors / norms
    print(f"  mean vector norm after centering: {np.linalg.norm(vectors.mean(axis=0)):.6f}")
    print()

    # Step 4: Simulated attention
    if not args.skip_attention:
        print(f"step 4: simulated attention (K={SIM_ATTENTION_K}, iters={SIM_ATTENTION_ITERS})...")
        t0 = time.time()
        vectors = simulated_attention(vectors, SIM_ATTENTION_K, SIM_ATTENTION_ITERS, SIM_ATTENTION_ALPHA)
        print(f"  completed in {time.time() - t0:.1f}s")
    else:
        print("step 4: simulated attention SKIPPED")
    print()

    # Step 5: Write output
    print("step 5: writing output files...")
    dim = vectors.shape[1]
    write_bin(str(out_dir / "code_vectors.bin"), vectors, dim)
    write_tokens_txt(str(out_dir / "code_tokens.txt"), filtered_tokens)

    if os.path.exists(checkpoint_path):
        os.remove(checkpoint_path)

    bin_size = os.path.getsize(str(out_dir / "code_vectors.bin"))
    print()
    print("=" * 60)
    print(f"  model: {MODEL_NAME}")
    print(f"  tokens: {len(filtered_tokens)}")
    print(f"  dimensions: {dim}")
    print(f"  blob size: {bin_size / (1024*1024):.1f} MB")
    print("=" * 60)


if __name__ == "__main__":
    main()