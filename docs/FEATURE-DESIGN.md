# Feature Design: `cora init --interactive` & `cora doctor`

## Status: Draft — Pending Review

## Overview

Two new features to improve developer experience:

1. **`cora init --interactive`** (or `cora init -i`) — guided, step-by-step config creation with prompts
2. **`cora doctor`** — health check command that validates API connectivity, config, and environment

---

## 1. `cora init --interactive`

### Current Behavior

`cora init` generates a static `.cora.yaml` template with hardcoded defaults (openai, gpt-4o-mini). User must manually edit the file to customize.

CLI flags (`--provider`, `--model`, `--base-url`, `--api-key`) allow one-shot config but are non-interactive.

### Proposed Behavior

With `--interactive` (or `-i`), `cora init` becomes a guided wizard:

```
$ cora init -i

🚀 Welcome to Cora! Let's set up your project.

? Select a provider:
  > OpenAI
    Anthropic
    Groq
    Ollama (local)
    Google
    Custom (OpenAI-compatible)

? Model name (default: gpt-4o-mini): glm-5.1

? Base URL (default: https://api.openai.com/v1): https://litellm:4000

? API key: **** (or leave blank to use CORA_API_KEY env var)

? Review focus areas (space to toggle, enter to confirm):
  [✓] security
  [✓] performance
  [ ] bugs
  [ ] best_practice
  [ ] style
  [ ] maintainability

? Pre-commit hook mode:
  > warn (print findings but allow commit)
    block (fail commit on findings above threshold)

✅ Created .cora.yaml with your settings.
✅ Pre-commit hook installed.

Next steps:
  cora review     # Review staged changes
  cora scan       # Scan the full project
```

### Implementation Approach

- **No new dependencies** — use `dialoguer` (already in Cargo.toml? check) or `inquire`. If neither exists, use minimal stdin/stdout with `rustyline` or basic `io::stdin()`.
- **Fallback** — if `--interactive` is not passed, current behavior is preserved (static template).
- **Flags take precedence** — `cora init -i --provider openai --model gpt-4o` skips the provider/model prompts.

### Config Schema Changes

None — output is the same `.cora.yaml` format, just populated interactively.

### Open Questions

1. **Which prompt library?** — `dialoguer` (lightweight), `inquire` (feature-rich), or raw stdin?
2. **Should it test API connectivity** after setup? (e.g., send a minimal request to validate key)
3. **Should it detect existing API keys** from env vars and pre-select the provider?

---

## 2. `cora doctor`

### Purpose

Diagnostic command that checks the entire cora setup — config, API connectivity, git state, environment.

### Proposed Output

```
$ cora doctor

🔍 Cora Doctor — System Health Check

Config
  ✅ .cora.yaml found at /project/.cora.yaml
  ✅ Config schema valid

API Connectivity
  ✅ Provider: openai (gpt-4o-mini)
  ✅ API key detected (CORA_API_KEY)
  ✅ Connection OK — 142ms latency
  ✅ Model accessible

Git
  ✅ Inside a git repository
  ✅ Upstream detected: origin/main
  ⚠️  3 uncommitted changes detected

Environment
  ✅ Rust version: 1.85.0
  ✅ cora version: 0.1.2
  ✅ Shell completions available: bash, zsh, fish, powershell

Pre-commit Hook
  ✅ Installed at .git/hooks/pre-commit

Result: 7 passed, 1 warning — your setup is ready! 🚀
```

### Checks (ordered)

| # | Check | Severity | What it does |
|---|-------|----------|-------------|
| 1 | Config file found | error | Looks for `.cora.yaml` or `~/.config/cora/config.yaml` |
| 2 | Config schema valid | error | Parses YAML, validates against schema |
| 3 | API key available | error | Checks `CORA_API_KEY` or provider-specific env vars |
| 4 | Provider configured | error | Has a valid provider name |
| 5 | API connectivity | error | Sends a minimal request (e.g., list models) to validate key + endpoint |
| 6 | API latency | info | Measures round-trip time to API endpoint |
| 7 | Model accessible | warn | Validates the configured model exists at the endpoint |
| 8 | Git repository | warn | Checks if inside a git repo (not required for `cora scan --path`) |
| 9 | Pre-commit hook | info | Checks if `.git/hooks/pre-commit` has cora |
| 10 | Cora version | info | Shows current version, checks for updates |

### Implementation Approach

- **New command** — `src/commands/doctor.rs`
- **New subcommand** in `main.rs` CLI definition
- **Check result type** — enum `{ Ok, Warning, Error }` with message
- **Optional `--json` flag** — machine-readable output for CI
- **Exit code** — 0 if all pass, 1 if any error, 2 if connection failed

### Open Questions

1. **Check for updates** — should `cora doctor` hit GitHub API to check latest version? (adds network dependency)
2. **Model validation** — how to check if a model exists without expensive API calls? (Some providers support `GET /models`)
3. **Should it fix things?** — e.g., `cora doctor --fix` to auto-install hook, fix config schema?

---

## Effort Estimate

| Feature | LOC (est.) | New Files | Effort | Risk |
|---------|-----------|-----------|--------|------|
| `cora init -i` | ~150-250 | 1 (modify `init.rs`) | 2-3h | Low |
| `cora doctor` | ~200-300 | 2 (`doctor.rs`, types) | 3-4h | Low |

## Dependencies

- `dialoguer` or `inquire` crate (for interactive prompts) — whichever is lighter
- No new deps for `cora doctor` — uses existing HTTP client

## Timeline

Both features can go into **v0.2.0** (next minor release) or as individual patches.
