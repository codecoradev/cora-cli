# AGENT.md — AI-Assisted Development Onboarding

## Project Overview

**cora-cli** is a Rust CLI tool for AI-powered code review. Bring Your Own Keys (BYOK) —
no managed API, no cloud service. Runs locally against diffs, scans, or branches.

- **License:** MIT
- **Edition:** Rust 2024 (MSRV 1.85)
- **Repo:** `codecoradev/cora-cli`
- **Default branch:** `develop`

## Build & Development Commands

```bash
cargo build              # Build (debug)
cargo build --release    # Build (release)
cargo test               # Run all 157 tests
cargo clippy             # Lint
cargo fmt                # Format check (use -- --check for CI)
```

## Code Structure (`src/`)

```
src/
├── main.rs              # Entry point, CLI argument parsing
├── commands/            # Subcommand handlers
│   ├── mod.rs
│   ├── review.rs        # cora review (diff-based review)
│   ├── scan.rs          # cora scan (full-file scan)
│   ├── config_cmd.rs    # cora config (show/set)
│   ├── auth.rs          # cora auth (API key management)
│   ├── hook_cmd.rs      # cora hook (pre-commit hook install/uninstall)
│   ├── init.rs          # cora init (project scaffolding)
│   ├── upload.rs        # cora upload (review upload)
│   ├── completion.rs    # Shell completion generation
│   └── providers.rs     # cora providers (list providers)
├── config/
│   ├── mod.rs
│   ├── schema.rs        # Config structs & defaults
│   ├── loader.rs        # Config loading & priority chain
│   └── providers.rs     # LLM provider definitions
├── engine/
│   ├── mod.rs
│   ├── review.rs        # Review orchestration logic
│   ├── scanner.rs       # File scanning engine
│   ├── llm.rs           # LLM API interaction
│   └── types.rs         # Severity, finding, and result types
├── formatters/          # Output format implementations
│   ├── mod.rs
│   ├── pretty.rs        # Human-readable terminal output
│   ├── compact.rs       # Compact single-line output
│   ├── json_fmt.rs      # JSON output
│   └── sarif.rs         # SARIF format (CI integration)
├── git/
│   ├── mod.rs
│   ├── diff.rs          # Git diff parsing
│   └── files.rs         # File listing & filtering
└── hook/
    ├── mod.rs
    ├── install.rs       # Hook install/uninstall logic
    └── template.rs      # Hook script templates
```

## Key Files

| File | Purpose |
|---|---|
| `commands/config_cmd.rs` | Config subcommand — display, set, path resolution |
| `config/loader.rs` | Config loading with full priority chain resolution |
| `config/schema.rs` | All config structs, defaults, serde annotations |
| `commands/init.rs` | Project scaffolding, `.cora.yaml` generation |

## Testing

```bash
cargo test               # 205 tests total
                         #   183 unit tests
                         #   16 CLI integration tests
                         #    6 config tests
cargo test --no-verify   # Skip pre-commit hooks (avoids timeout in hooks)
```

Use `--no-verify` when running tests through pre-commit hooks to prevent nested
hook execution and timeouts.

## CI/CD

Three GitHub Actions workflows in `.github/workflows/`:

1. **ci.yml** — PR checks: build, test, clippy, fmt on push to `develop` and PRs
2. **release.yml** — Triggered by `v*` tags; builds for 4 platforms (Linux x86_64,
   macOS x86_64, macOS ARM64, Windows x86_64), publishes to crates.io
3. **deploy-website.yml** — Deploys project website/docs

## Config System

### Priority Chain (highest to lowest)

1. **CLI flags** (`--provider`, `--model`, etc.)
2. **Environment variables** (`CORA_PROVIDER`, `CORA_MODEL`, etc.)
3. **Project config** (`.cora.yaml` in repo root)
4. **Global config** (`~/.cora/config.yaml`)
5. **Built-in defaults** (defined in `config/schema.rs`)

### Auth Separation

API keys live in a separate `auth.toml` file (`~/.cora/auth.toml`), not in
`.cora.yaml`. This prevents accidental key leakage via committed config files.

## Design Decisions

- **Severity comparison uses `<=` not `>=`**: Severity filtering uses `<=` because
  the ordinal maps info=1, warning=2, error=3 — so `--severity warning` means
  "include warning and below (info)", not "warning and above (error)". This is
  intentional; the mapping is defined in `engine/types.rs`.
- **TOML → YAML migration**: Config format migrated from TOML to YAML for better
  readability and broader tooling support. The loader only reads YAML now.
- **Auth/config separation**: API keys are deliberately stored in `auth.toml`
  rather than the main config to allow `.cora.yaml` to be safely committed to
  repositories without exposing secrets.
- **LLM JSON repair**: `engine/llm.rs` includes `repair_invalid_escapes()` — a state
  machine that fixes invalid JSON escape sequences produced by some LLMs (e.g.
  `\s`, `\d` → `\\s`, `\\d`). Applied before `serde_json::from_str` in all parse paths.
  `review_diff()` also retries once on parse failure.
- **Temperature 0 (deterministic)**: Default temperature is 0 (v0.1.7+). Same diff
  produces identical review output every run. Cache key includes model + temperature.
- **Per-request HTTP timeout**: `reqwest::Client` shared via `LazyLock` (connection
  pooling). Timeout set per-request via `.timeout()` on RequestBuilder, not at
  client construction (client-level is misleading).
- **Diff-hash caching**: Reviews cached in `~/.cache/cora/reviews/` by SHA-256 of
  diff + model + temperature. TTL configurable via `llm.cache_ttl`. Bypass with
  `--no-cache`. Cache stores fully-filtered response (after anti-hallucination).
- **Anti-hallucination**: File paths extracted from diff headers, injected into
  prompt. Post-parse filtering removes issues referencing files not in diff.
  `system_prompt_file` path traversal guard (canonicalized root check).
- **Composite action resilience**: Version resolution retries 3x with fallback.
  `curl -sfL` (fail on HTTP errors). `d.get('tag_name', '')` (no KeyError).
- **Release workflow v-prefix**: `release.yml` strips `v` from git tag to match
  CHANGELOG `[X.Y.Z]` format. `TAG` (with v) for display/URLs, `VERSION` (without)
  for changelog sed matching and asset naming.
- **Infisical secrets**: All workflows use `secrets.INFISICAL_IDENTITY_ID` — never
  hardcode identity IDs.
- **Documentation update before release**: README config/features/flags, website
  configuration docs page, and homepage feature bullets MUST be updated to reflect
  new features BEFORE version bump and tagging.
