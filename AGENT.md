# AGENT.md — AI-Assisted Development Onboarding

## Project Overview

**cora-cli** is a Rust CLI tool for AI-powered code review. Bring Your Own Keys (BYOK) —
no managed API, no cloud service. Runs locally against diffs, scans, or branches.

- **License:** MIT
- **Edition:** Rust 2024 (MSRV 1.85)
- **Repo:** `codecoradev/cora-cli`
- **Default branch:** `develop`
- **Marketplace:** https://github.com/marketplace/actions/cora-ai-code-review
- **Website:** https://codecora.dev

## Build & Development Commands

```bash
cargo build              # Build (debug)
cargo build --release    # Build (release)
cargo test               # Run all 517 tests
cargo clippy --all-targets -- -D warnings  # Lint (strict)
cargo fmt --all -- --check  # Format check
```

Always run `cargo fmt --all` before committing. CI runs all three checks.

## Code Structure (`src/`)

```
src/
├── main.rs              # Entry point, CLI argument parsing
├── commands/            # Subcommand handlers
│   ├── mod.rs
│   ├── review.rs        # cora review (diff-based review)
│   ├── scan.rs          # cora scan (full-file scan)
│   ├── debt.rs          # cora debt (tech debt report)
│   ├── commit_cmd.rs    # cora commit (review + commit message + commit)
│   ├── config_cmd.rs    # cora config (show/set/validate)
│   ├── auth.rs          # cora auth (API key management)
│   ├── hook_cmd.rs      # cora hook (pre-commit hook install/uninstall)
│   ├── init.rs          # cora init (project scaffolding)
│   ├── upload.rs        # cora upload (review upload)
│   ├── completion.rs    # Shell completion generation
│   ├── debt.rs          # cora debt (tech debt report)
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
│   ├── types.rs         # Severity, finding, and result types
│   ├── diff_parser.rs   # Diff → FileChunk parsing
│   ├── chunker.rs       # Auto-chunking large diffs
│   ├── profiles.rs      # Quality profiles (strict/balanced/lax)
│   ├── quality_gate.rs  # Quality gate thresholds + pass/fail
│   ├── debt_tracker.rs  # Tech debt metrics + trend tracking
│   ├── security_scanner.rs  # Static security pattern matching
│   ├── language_analyzer.rs # Language-specific review guidance
│   ├── secrets_scanner.rs   # Secret/credential detection
│   ├── debt_tracker.rs  # Tech debt metrics + history snapshots
│   ├── memory.rs        # Uteke memory integration (recall + learn)
│   └── rules/           # Custom rule engine
│       ├── mod.rs
│       ├── builtin.rs   # Built-in rules
│       ├── matching.rs  # Rule matching logic
│       └── types.rs     # Rule & finding types
├── mcp/                 # MCP server (Model Context Protocol)
│   ├── mod.rs
│   ├── protocol.rs      # JSON-RPC 2.0 types
│   ├── server.rs        # Stdio transport + request dispatch
│   └── tools.rs         # 5 tool handlers
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
| `commands/config_cmd.rs` | Config subcommand — display, set, validate, path resolution |
| `config/loader.rs` | Config loading with full priority chain resolution |
| `config/schema.rs` | All config structs, defaults, serde annotations |
| `commands/init.rs` | Project scaffolding, `.cora.yaml` generation |

## Testing

```bash
cargo test               # 517 tests total
                         #   484 unit tests
                         #    16 CLI integration tests
                         #     6 config tests
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

## Lessons Learned (Agent Operating Principles)

These are hard-won lessons from actual development sessions. Follow them.

### Workflow Discipline

- **One issue = one branch = one PR = wait CI green = merge = next.** Never stack
  multiple features on one branch. Serial workflow prevents merge conflicts and
  makes bisecting bugs trivial.
- **Local green ≠ CI green.** CI may use a different Rust version that is stricter
  (e.g. treats dead_code as error, not warning). Always wait for CI to pass.
- **Delete stale branches immediately after merge.** `git push origin --delete <branch>`.
  A clean `git branch -a` improves developer experience.

### Rust-Specific

- **`pub` visibility = API contract.** Every `pub` added must have justification.
  When exposing internals (e.g. `SecurityPattern` fields for MCP), consider adding
  getter methods instead of making fields public.
- **`Result<(), CoraError>` is infectious.** Changing a return type from `()` to
  `Result` will propagate to every callsite. Plan the blast radius before changing
  signatures.
- **Use `edit` (exact text replacement), not `sed` for refactoring.** `sed` applies
  globally and can mangle struct literal instances when you only meant to change the
  struct definition.
- **`#[allow(dead_code)]` for structs used only in `#[cfg(test)]`.** Clippy in CI
  with `-D warnings` treats dead_code as error. Functions called only from tests
  need this annotation.

### CI & Code Scanning

- **Code Scanning alerts: evaluate before dismissing.** Some are false positives
  (test fixtures like `AKIAIOSFODNN7EXAMPLE`), but others catch real bugs (e.g.
  redundant `parse_diff()` calls, broken line-based stdio parsing).
- **Dismiss with reason.** Always provide `dismissed_reason` ("won't fix" or
  "false positive") so future reviewers understand the decision.

### MCP Server Design

- **MCP is simpler than you think.** JSON-RPC 2.0 over stdio. No HTTP server needed.
  The client (Claude Code, Cursor, etc.) manages the lifecycle.
- **Don't use line-based stdin parsing for JSON-RPC.** Pretty-printed JSON spans
  multiple lines. Use brace-depth tracking instead.
- **Reuse parsed data.** If `parse_diff()` already ran, pass the `Vec<FileChunk>` to
  downstream functions. Never parse the same diff twice.

### Security Scanner

- **Every regex pattern needs exemption logic.** Skip test files, example keys,
  fixtures. Without exemptions, noise drowns out real findings.
- **Balance sensitivity.** Too strict = wall of false positives. Too loose = miss
  real vulnerabilities. Start conservative, tune based on real findings.

## Pre-Release Checklist

Before any release (version bump + tag), verify ALL of these are done.
Missing any = release blocker.

### 1. Code

- [ ] All target issues merged to `develop`
- [ ] `cargo test` — all 517+ tests pass
- [ ] `cargo clippy --all-targets -- -D warnings` — clean
- [ ] `cargo fmt --all -- --check` — clean
- [ ] `cargo build --release` — no errors
- [ ] `./target/release/cora --version` — prints correct version
- [ ] `./target/release/cora mcp --help` — new subcommands work
- [ ] `./target/release/cora review --staged` — no crash on clean tree

### 2. Documentation (Every File)

Documentation drift is the #1 source of user confusion. Each file must reflect
reality BEFORE version bump.

| File | What to check |
|------|---------------|
| `README.md` | "Why Cora" bullets match all features. Commands table includes all subcommands. Config example shows latest features. All links point to `codecora.dev` |
| `CHANGELOG.md` | New version entry with ALL changes (Added, Changed, Fixed, Removed). Link references at bottom include new version |
| `docs/changelog.md` | Mirrors `CHANGELOG.md` exactly — same content, same links |
| `docs/roadmap.md` | Completed items marked ✓ Done (not ◎ Planned). Future items accurate |
| `docs/getting-started.md` | Quick start still works. Next steps links valid. New major features mentioned |
| `docs/configuration.md` | All config keys documented with examples. New sections (quality gate, security scanner, MCP, profiles, custom rules, language analyzers) present |
| `docs/cli-reference.md` | All commands listed. New subcommands (e.g. `cora mcp`) included. Flags accurate |
| `docs/usage.md` | Review modes, output formats, exit codes up to date. New sections present |
| `docs/examples.md` | CI examples work. Marketplace action reference correct. Multi-platform examples present |
| `docs/providers.md` | Provider list, default models, env vars accurate |
| `docs/installation.md` | Version pin example uses latest. Platforms list accurate |
| `AGENT.md` | Code structure tree current. Test count current. Key files table current |
| `.agent.md` | Pre-release checklist current. CI checks count current. Module dependencies current |

### 3. Cross-Check

- [ ] **Feature coverage**: Every new feature appears in README + CHANGELOG + docs/configuration.md + docs/roadmap.md
- [ ] **Consistent terminology**: Same name for features across all files (e.g. "Quality Gate" not "quality gate" or "gate check")
- [ ] **No broken links**: All `codecora.dev` links resolve. All internal doc links work
- [ ] **Version numbers**: README install example, docs/installation.md pin example, AGENT.md test count — all match current version
- [ ] **Star History chart**: Repository list includes all relevant repos (e.g. `cora-cli,uteke`)

### 4. CI & Scanning

- [ ] CI: 10/10 checks green on develop branch
- [ ] Code Scanning: 0 open alerts (fix or dismiss each with reason)
- [ ] No open PRs blocking the release

### 5. Post-Merge Verification

After merging to `main`:

- [ ] `release.yml` triggers on `v*` tag
- [ ] 4 platform binaries published to GitHub Releases
- [ ] `crates.io` publish succeeds
- [ ] Website (`codecora.dev`) reflects new docs
- [ ] Marketplace action still works (test on a PR)

## Release Workflow

Release authority belongs to the project owner only. The agent NEVER triggers a release
without explicit approval. This workflow was established during v0.5.0 development.

### Phase 1: Pre-Release Preparation (agent does this)

Complete ALL items in the Pre-Release Checklist above. Every single checkbox must be green.

### Phase 2: Validation Report (agent generates, owner reviews)

Generate a comprehensive pre-release report covering:

```
╔══════════════════════════════════════════════════════════════╗
║           LAPORAN PRE-RELEASE vX.Y.Z — FINAL               ║
╚══════════════════════════════════════════════════════════════╝

📦 REPOSITORY: codecoradev/cora-cli
🌿 BRANCH:     develop (N commits ahead of main)
🏷️  TAG:       Next → vX.Y.Z
📋 CARGO:      version = "0.X.Y" (needs bump)

✅ TESTS:        N pass (unit + CLI + config)
✅ CLIPPY:       Clean
✅ FORMAT:       Clean
✅ CI:           10/10 green
✅ CODE SCANNING: 0 open
✅ OPEN PRs:      0
✅ OPEN ISSUES:   N (all post-release scope)

📋 vX.Y.Z FEATURES:
   1. ...
   2. ...

📄 DOCS VERIFICATION:
   ✅ README.md         — ...
   ✅ CHANGELOG.md       — ...
   ✅ docs/*             — ...
   ✅ AGENT.md           — ...

⚠️ REMAINING (post-release):
   • ...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
👍 SIAP RILIS — menunggu approval.
```

### Phase 3: Acceptance (owner decides)

The owner reviews the report and either:
- **APPROVES** → "oke kerjakan" / "go ahead" / "rilis"
- **REQUESTS CHANGES** → specifies what to fix first
- **POSTPONES** → "belum, tunggu dulu"

**The agent must NEVER proceed without explicit approval.**

### Phase 4: Release Execution (only after approval)

```bash
# 1. Bump version
sed -i '' 's/version = "0.X.Y"/version = "0.X.Z"/' Cargo.toml

# 2. Update version references in docs
#    - docs/installation.md: CORA_VERSION pin example
#    - AGENT.md: test count if changed
#    - docs/.vitepress/config.ts: nav bar version

# 3. Commit and tag
git add -A && git commit -m "chore: bump version to X.Y.Z"
git tag vX.Y.Z
git push origin develop --tags

# 4. release.yml auto-triggers:
#    → sync develop → main (force push)
#    → build 4 platforms (Linux x86_64, Linux ARM64, macOS ARM64, Windows)
#    → publish to crates.io
#    → create GitHub Release with changelog excerpt

# 5. deploy-website.yml auto-triggers (on push to main):
#    → build VitePress docs
#    → deploy to codecora.dev
```

### Phase 5: Post-Release Verification (agent does this)

After release completes, verify:

- [ ] GitHub Release page shows vX.Y.Z with correct changelog
- [ ] 4 platform binaries attached to release
- [ ] SHA256 checksums file included
- [ ] `crates.io` shows new version: `cargo search cora-cli`
- [ ] `codecora.dev` reflects new docs
- [ ] Marketplace action still works (test on a test PR)
- [ ] Close the released milestone/issues
- [ ] Update roadmap: mark released items

### Rollback Procedure

If release fails or has critical bugs:

1. Delete the tag: `git push origin --delete vX.Y.Z`
2. Delete the GitHub Release
3. Yank from crates.io: `cargo yank cora-cli@X.Y.Z`
4. Fix on develop, re-tag when ready

### Version Numbering Convention

- **Patch** (0.4.6 → 0.4.7): Bug fixes, docs updates, no new features
- **Minor** (0.4.x → 0.5.0): New features, backwards compatible
- **Major** (0.x → 1.0.0): Breaking changes
- **Pre-release**: Tag with suffix (v0.5.0-beta.1) — `release.yml` marks as prerelease

### Key Lessons from v0.5.0 Release

1. **Documentation drift is the biggest risk.** PRs from other agents may merge code
   without updating docs. Always audit docs coverage BEFORE release.
2. **Ghost versions in CHANGELOG are dangerous.** If a version entry exists but has
   no tag, merge it into the next real version before release.
3. **Test count changes with every PR.** Verify actual test count matches AGENT.md.
4. **Deploy-website trigger must be main-only.** develop pushes should never deploy to production.
5. **Code Scanning alerts accumulate.** Dismiss false positives with reason, fix real ones.

## External Submissions

When submitting cora to directories, aggregators, or showcases (Trendshift, etc.):

### Description Template

> Cora CLI — Open-source AI code review tool written in Rust. BYOK (Bring Your Own Key) —
> works with any OpenAI-compatible LLM. Runs locally via CLI, CI/CD, pre-commit hooks, or
> as an MCP server for AI coding agents (Claude Code, Cursor, Copilot).
>
> Features: diff-based review, static security scanning (11 regex patterns), quality gate
> with configurable thresholds, language-specific analyzers (Dart/Flutter, Svelte,
> TypeScript, Go, Rust, Python), secret detection, custom rule engine, quality profiles,
> auto-chunking for large PRs, and SARIF output for CI integration.

### Key Metrics to Mention

- Test count (495+)
- Lines of Rust code (16,800+)
- CI checks (10)
- GitHub Marketplace action published
- MCP server with 5 tools
- MIT license
- Active development cadence

### Pre-Submission Checks

- [ ] README accurately describes ALL current features (not outdated)
- [ ] All docs synced (changelog, roadmap, configuration, CLI reference)
- [ ] No stale "Planned" items on roadmap that are actually done
- [ ] Star History chart includes all relevant repos
