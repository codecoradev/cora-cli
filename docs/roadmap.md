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

## v0.5 — Install & Distribution

- [#151](https://github.com/codecoradev/cora-cli/issues/151) Easy install — Homebrew tap & install script — ✓ Done
- [#149](https://github.com/codecoradev/cora-cli/issues/149) CI gate mode — block PRs on review findings — ✓ Done
- [#163](https://github.com/codecoradev/cora-cli/issues/163) Website redesign — landing page + docs — ✓ Done
- [#172](https://github.com/codecoradev/cora-cli/issues/172) Interactive auth login with tiered provider selection — ✓ Done
- [#162](https://github.com/codecoradev/cora-cli/issues/162) README overhaul — market-facing copy — ◎ Planned

## Future — What's Next

- [#117](https://github.com/codecoradev/cora-cli/issues/117) Lightweight agent follow-up — 1 capped tool-call — → Planned
- [#161](https://github.com/codecoradev/cora-cli/issues/161) `cora gain` — local productivity stats + viral sharing — → Planned
- [#132](https://github.com/codecoradev/cora-cli/issues/132) GitHub App backend MVP in Rust (Axum) — → Planned
- [#47](https://github.com/codecoradev/cora-cli/issues/47) Publish cora-review as GitHub Marketplace action — → Planned
