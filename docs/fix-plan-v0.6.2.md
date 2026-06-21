# Fix Plan — v0.6.2 Patch Release

Branch: `fix/token-usage-tracking-and-hardcode-cleanup`
Base: `develop` @ f1027ba

## Executive Summary

Investigasi menemukan **3 bug fungsional** (token usage selalu hilang) dan
**2 masalah hardcode/optimasi**. Semua lolos dari compiler & 586 tests karena
tidak ada test yang meng-assert `tokens_used`.

---

## Bug Fixes

### BUG-1: `tokens_used` selalu `None` di `parse_review_response` / `parse_scan_response`

**Root cause:** Kedua fungsi parse hardcoded `Ok((..., None))` sebagai return
value untuk `tokens_used`, meskipun `chat_completion()` sudah menerima dan
membuang objek `Usage` dari API response.

**Impact:**
- `--progress` events selalu report `tokens: {input: 0, output: 0}`
- `ScanResponse.tokens_used` selalu `None` di output JSON/SARIF
- Cost estimation (`estimated_cost_usd`) tidak pernah terisi
- Debt tracker tidak bisa track token cost dari review ke review

**Fix:**
1. `chat_completion()` return tuple `(String, Option<Usage>)` bukan `String`
2. `parse_review_response` / `parse_scan_response` terima parameter `Option<&Usage>`
3. Konversi `Usage` → `TokenUsage` via helper `usage_to_token_usage()`
4. Update semua call sites

### BUG-2: Streaming response tidak collect `usage`

**Root cause:** `chat_completion_stream()` & `review_diff_stream()` hanya
mengumpulkan `delta.content`, mengabaikan field `usage` yang dikirim provider
(via `stream_options: {include_usage: true}` atau di chunk terakhir).

**Fix:**
1. Tambah `stream_options: {include_usage: true}` ke request body
2. Parse `usage` dari SSE chunk (banyak provider kirim di chunk terakhir)
3. Return `(String, Option<TokenUsage>)` dari streaming path

### BUG-3: Scan multi-batch `total_tokens` di-overwrite, bukan diakumulasi

**Root cause:** `scan.rs:148` — `total_tokens = tokens` menimpa nilai batch
sebelumnya. Hanya batch terakhir yang sukses yang dilaporkan.

**Fix:** Akumulasi via helper `accumulate_token_usage()`.

---

## Hardcode & Optimasi

### HARD-1: Magic number `20` dan `60_000` di `scan.rs`

`max_files_per_batch` fallback `20` dan token-budget `60_000` di-hardcode.
Ekstrak ke konstanta bermakna.

### HARD-2: `Usage.prompt_tokens` / `completion_tokens` diduplikasi field

`Usage` struct punya `#[allow(dead_code)]` untuk `prompt_tokens` dan
`completion_tokens` — sekarang akan dipakai untuk konversi ke `TokenUsage`,
jadi `allow` bisa dihapus.

---

## Testing

- Tambah test untuk BUG-1: assert `tokens_used.is_some()` ketika API kasih usage
- Tambah test untuk BUG-3: assert akumulasi 2 batch → total = jumlah
- Tambah test untuk `usage_to_token_usage()` konversi
- Pastikan semua 586 test existing masih pass

---

## Version Bump & Changelog

- `Cargo.toml`: `0.6.1` → `0.6.2`
- `CHANGELOG.md`: tambah entry `[0.6.2]` dengan Fixed section
