---
title: Roadmap
---

# Roadmap

Demand-gated — we build what people actually need. Track progress on [GitHub Issues](https://github.com/codecoradev/cora-cli/issues).

## v0.1.5 — Initial Release

- [#90](https://github.com/codecoradev/cora-cli/issues/90) Basic diff review with OpenAI — ✓ Done
- [#89](https://github.com/codecoradev/cora-cli/issues/89) JSON response repair & unicode handling — ✓ Done
- [#90](https://github.com/codecoradev/cora-cli/issues/90) CLI interface with review command — ✓ Done

## v0.1.6 — Custom Prompts & Path Injection

- [#95](https://github.com/codecoradev/cora-cli/issues/95) Enhanced default system prompts — ✓ Done
- [#94](https://github.com/codecoradev/cora-cli/issues/94) Custom system prompt via .cora.yaml config — ✓ Done
- [#93](https://github.com/codecoradev/cora-cli/issues/93) Inject valid file paths into system prompt — ✓ Done
- [#92](https://github.com/codecoradev/cora-cli/issues/92) JSON object response format (opt-in) — ✓ Done

## v0.1.7 — Deterministic & Reliable

- [#98](https://github.com/codecoradev/cora-cli/issues/98) Deterministic reviews — temperature=0 — ✓ Done
- [#97](https://github.com/codecoradev/cora-cli/issues/97) Non-deterministic output bug fix — ✓ Done
- [#99](https://github.com/codecoradev/cora-cli/issues/99) HTTP timeout + connection pooling — ✓ Done
- [#100](https://github.com/codecoradev/cora-cli/issues/100) Diff-hash caching for repeat reviews — ✓ Done
- [#101](https://github.com/codecoradev/cora-cli/issues/101) Configurable max_tokens — ✓ Done

## v0.2.0 — Multi-Provider & SARIF

- [#106](https://github.com/codecoradev/cora-cli/issues/106) BYOK — Anthropic, Groq, Ollama support — ✓ Done
- [#106](https://github.com/codecoradev/cora-cli/issues/106) SARIF output format — ✓ Done
- [#106](https://github.com/codecoradev/cora-cli/issues/106) Branch review mode — ✓ Done
- [#106](https://github.com/codecoradev/cora-cli/issues/106) Output footer watermark — ✓ Done

## v0.3 — Progress & CI Hardening

- [#140](https://github.com/codecoradev/cora-cli/issues/140) Static analysis context injection (reduce false positives) — ✓ Done
- [#108](https://github.com/codecoradev/cora-cli/issues/108) --progress flag for machine-readable output — ✓ Done
- [#102](https://github.com/codecoradev/cora-cli/issues/102) Composite action crash fix (KeyError) — ✓ Done
- [#88](https://github.com/codecoradev/cora-cli/issues/88) Config validate command — ✓ Done

## v0.4 — Deterministic Engine Pipeline

- [#116](https://github.com/codecoradev/cora-cli/issues/116) Deterministic rule engine — 12 built-in rules — ✓ Done
- [#115](https://github.com/codecoradev/cora-cli/issues/115) File bundling — parallel per-bundle review — ✓ Done
- [#114](https://github.com/codecoradev/cora-cli/issues/114) AST-based cross-file dependency extraction — ✓ Done
- [#159](https://github.com/codecoradev/cora-cli/issues/159) Hunk header regex panic fix + 5MB diff support — ✓ Done

## v0.4.5 — Config Architecture

- [#209](https://github.com/codecoradev/cora-cli/issues/209) Config redesign — auth.toml for secrets, config.yaml for settings — ✓ Done
- [#203](https://github.com/codecoradev/cora-cli/issues/203) Auth login auto-detect provider env vars — ✓ Done
- [#189](https://github.com/codecoradev/cora-cli/issues/189) `cora config show` effective resolved config — ✓ Done
- [#182](https://github.com/codecoradev/cora-cli/issues/182) Env var override visibility — ✓ Done
- [#185](https://github.com/codecoradev/cora-cli/issues/185) Deterministic rules exclude `rules/` — ✓ Done
- [#186](https://github.com/codecoradev/cora-cli/issues/186) Truncated JSON repair tests — ✓ Done

## v0.4.6 — Polish & Docs

- [#162](https://github.com/codecoradev/cora-cli/issues/162) README overhaul — market-facing copy — ✓ Done
- [#204](https://github.com/codecoradev/cora-cli/issues/204) Deterministic secrets pre-scan — ✓ Done
- [#195](https://github.com/codecoradev/cora-cli/issues/195) Diff parser hardening Phase 1 — ✓ Done

## v0.5 — Agent & Quality

- [#205](https://github.com/codecoradev/cora-cli/issues/205) Quality gate — CI pass/fail thresholds — ✓ Done
- [#234](https://github.com/codecoradev/cora-cli/issues/234) Static security scanner — 11 patterns — ✓ Done
- [#233](https://github.com/codecoradev/cora-cli/issues/233) Language-specific analyzers (Dart, Svelte, TS, Go, Rust, Python) — ✓ Done
- [#207](https://github.com/codecoradev/cora-cli/issues/207) MCP server — expose rules to AI agents — ✓ Done
- [#238](https://github.com/codecoradev/cora-cli/issues/238) Quality profiles bug fix — path resolution, fail-fast, dedup — ✓ Done
- [#188](https://github.com/codecoradev/cora-cli/issues/188) Auto-chunking for large diffs — ✓ Done
- [#206](https://github.com/codecoradev/cora-cli/issues/206) Tech debt metrics — review history — ✓ Done

## v0.6 — Growth & Marketplace

- [#47](https://github.com/codecoradev/cora-cli/issues/47) GitHub Marketplace action — ✓ Done
- [#196](https://github.com/codecoradev/cora-cli/issues/196) VitePress docs site — ✓ Done
- [#161](https://github.com/codecoradev/cora-cli/issues/161) `cora gain` — local stats + viral sharing — ◎ Planned
- [#160](https://github.com/codecoradev/cora-cli/issues/160) Landing page redesign — ◎ Planned

## Future — What's Next

- [#117](https://github.com/codecoradev/cora-cli/issues/117) Lightweight agent follow-up — 1 capped tool-call — → Planned
- [#132](https://github.com/codecoradev/cora-cli/issues/132) GitHub App backend MVP in Rust (Axum) — → Planned
