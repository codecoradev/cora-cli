# Changelog

All notable changes to cora-cli are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.8] - 2026-06-02

### Fixed

- **`unwrap()` → `expect()`** in ProgressStyle templates (llm.rs, scanner.rs) — clearer panic messages on template parse failure (#87)
- **Consolidated duplicate `impl Severity` blocks** into single implementation (#83)
- **`file_content_hash` returns `Option<String>`** instead of empty string on read failure — prevents infinite rescan loop on unreadable files (#77)
- **Permission errors logged in scanner** — file walk now logs permission errors at debug level instead of silently skipping (#76)
- **Auth file permission warning** — warns if `~/.cora/auth.toml` has overly permissive file permissions (Unix only) (#72)
- **SARIF upload size validation** — validates SARIF file size against GitHub's 10MB limit before upload (#82)
- **Float division for MB display** — SARIF size error now shows accurate fractional MB (was integer division truncating to 0) (#82)
- **Non-deterministic `DefaultHasher` → `sha2`** — scan cache now uses SHA-256 for deterministic hashing across Rust versions (#81)

### Added

- **`checksums-sha256.txt` in release artifacts** — release workflow generates SHA-256 checksums for all platform binaries (#109)

### Changed

- **Official CodeCora branding assets** — logo, favicon, and OG image updated from ajianaz/cora SaaS repo (#110)
- **Standalone `cora-review.yml` workflow** — CI action extracted from inline `ci.yml` job to dedicated workflow with concurrency control (#107)
- **Action v2 hardened** — all third-party actions pinned to commit SHA, checksum verification for binary downloads, env var indirection for inputs, `grep` pipefail fix, empty file guard, Node 24 strict mode compatibility (#107)

## [0.1.7] - 2026-06-01

### Added

- **Diff-hash caching** — review results cached by SHA-256 of diff + model + temperature in `~/.cache/cora/reviews/`. Cache TTL configurable via `llm.cache_ttl` (#100)
- **`--no-cache` flag** — bypass cache for fresh reviews (#100)
- **Configurable LLM parameters** — `llm.temperature` (default: 0), `llm.max_tokens` (default: 4096), `llm.timeout` (default: 120s), `llm.cache_ttl` (default: 1440 min) in `.cora.yaml` (#98 #101)
- **Git ref validation** — rejects refs containing shell metacharacters or path traversal sequences (#73)

### Fixed

- **Temperature default now 0** — eliminates non-deterministic LLM output. Same diff produces identical issues on every run (#98, #97)
- **HTTP timeout actually works** — per-request timeout via reqwest RequestBuilder (not client-level). Configurable timeout respected (#99)
- **Connection pooling** — shared reqwest::Client via LazyLock, reused across all requests (#99)
- **Cache key includes model + temperature** — config changes invalidate cache automatically (#100)
- **Silent config corruption** — malformed `.cora.yaml` now shows clear error with file path and hint (#78)
- **Composite action KeyError on API failure** — version resolution retries 3x with 5s delay, falls back to v0.1.6 with warning. Fixed in both `cora-review` and `cora-review-simple` actions (#102)

## [0.1.6] - 2026-06-01

### Added

- **Custom system prompts via config** — `review.system_prompt`, `review.system_prompt_file`, `scan.system_prompt`, `scan.system_prompt_file` fields in `.cora.yaml` (#94)
- **`response_format` config** — opt-in `json_object` response format for providers that support it, via `review.response_format: json_object` (#92)
- **File path injection into prompts** — valid diff file paths are injected into the review user prompt to reduce LLM hallucination (#93)
- **Post-parse file path filtering** — issues referencing non-existent files are filtered out after LLM response parsing (#93)
- **Enhanced default system prompts** — both review and scan prompts now include explicit anti-hallucination constraints, severity definitions, and format instructions (#95)

### Fixed

- **Path traversal in `system_prompt_file`** — arbitrary file read vulnerability. Now validates file path is within canonicalized project root (#92)
- **Symlink bypass in path traversal guard** — project root is now canonicalized to match resolved file paths

## [0.1.5] - 2026-06-01

### Fixed

- **Critical: JSON repair corrupts valid unicode escapes** — `is_valid_json_escape()` missing `'u'`, causing `\uXXXX` to be double-escaped. Now properly validates and handles incomplete `\u` sequences (#89)
- **Critical: TOML injection in `save_api_key()`** — API key written via `format!` string interpolation. Now uses `toml::Table` serialization (#69)
- **Retry prompt improvement** — retry on parse failure now includes stricter JSON format instructions (#90)
- **Temp file race condition** — SARIF upload now uses PID-suffixed temp path instead of fixed filename (#70)
- **Confusing unused `_cli_api_key` parameter** — removed from `load_config()` signature (#75)

### Security

- `save_api_key()` now uses `toml::Table::insert()` instead of string interpolation (prevents TOML injection)
- Temp SARIF file path includes process ID (prevents TOCTOU race)

## [0.1.4] - 2026-06-01

### Added

- LLM JSON repair engine (`repair_invalid_escapes`) — auto-fixes invalid escape sequences in LLM output (e.g. `\s`, `\d`) before JSON parse
- Retry mechanism in `review_diff` — if first LLM parse fails, automatically retries once
- Branding footer on "No issues found" PR comment — consistent with issues-found variant

### Fixed

- **Silent false-negative** — cora JSON parse failure previously posted "No issues found" without actual review (LLM invalid escapes)
- Hardcoded Infisical `identity-id` in `release.yml` and `deploy-website.yml` — migrated to `secrets.INFISICAL_IDENTITY_ID`
- Release workflow changelog extraction — `v` prefix mismatch (tag `v0.1.3` vs CHANGELOG `[0.1.3]`) now properly stripped
- `printf` double-escape in release workflow — `\\n` corrected to `\n`
- Stale `v0.1.2` binary download filenames in README
- Clippy `unnecessary_map_or` lint — `.map_or(false, |s| s.success())` replaced with `.is_ok_and(|s| s.success())`

### Changed

- All 3 workflows use `secrets.INFISICAL_IDENTITY_ID` (consistent with `ci.yml` pattern)
- Release workflow validates semver format before sed injection
- Branch cleanup — removed 14 stale branches

## [0.1.3] - 2026-06-01

### Added

- `cora config set --global` — write config to `~/.cora/config.yaml` instead of project `.cora.yaml`
- `cora config set base_url` — set base URL via CLI (previously only in YAML)
- Global config support (`~/.cora/config.yaml`) with priority chain: CLI flags → env vars → project → global → defaults
- Auto-migration from old `~/.cora/config.toml` to new YAML + `auth.toml` split

### Changed

- `cora config set` now writes YAML instead of TOML (compatible with config loader)
- API key storage moved from `~/.cora/config.toml` to `~/.cora/auth.toml` (0600 permissions)
- YAML serialization uses `skip_serializing_if` — no more `null` values in output

### Fixed

- **Severity comparison inverted** — `Critical` issues no longer silently pass `should_block` check (Ord ordering bug)
- Hook `mode: block` no longer exits with code 2 when "No issues found" (severity filter mismatch)
- Consistent severity logic across review, scan, and block mode paths

## [0.1.2] - 2025-05-29

### Added

- `cora init` — create `.cora.yaml` config file with provider/model selection
- `cora hook install|uninstall` — pre-commit hook management
- `cora config show|set` — configuration management
- CI composite action (`cora-review-simple`) for easy GitHub Actions integration
- Shell completions for bash, zsh, fish, and powershell
- `cora scan --incremental` with SHA256 content hash cache for fast incremental scanning
- `cora review --upload` for direct SARIF upload to GitHub Code Scanning
- `cora review --stream` for real-time review output
- `cora review --unpushed` for reviewing unpushed commits
- `cora review --base <branch>` for branch comparison
- `cora review --diff-file <path>` for reviewing external diff files
- `cora providers` command to list available LLM providers
- `cora auth login` for interactive API key storage

### Fixed

- SARIF schema compliance for GitHub Code Scanning upload
- Clippy `format_in_format_args` warnings
- Replaced deprecated `serde_yaml` with `serde_yaml_ng`
- Normalized release binary naming (`cora-{arch}-{target}-v{version}.tar.gz`)

### Changed

- Replaced deprecated dependencies
- Removed unused dependencies
- Bumped minimum Rust version to 1.85

## [0.1.1] - 2025-05-27

### Changed

- Replaced ASCII art banner with eye icon in README
- Updated README branding to cora-cli

### Fixed

- CI `cargo publish` with `--allow-dirty` for Cargo.lock mismatch on tag checkout

## [0.1.0] - 2025-05-25

### Added

- **AI Code Review** — review staged changes, commit ranges, branch diffs, and full project scans
- **BYOK** — bring your own API key (OpenAI, Anthropic, Groq, Ollama, Google)
- **5 LLM Providers** — with auto-detection from installed API keys
- **Pre-commit Hooks** — `cora hook install` for automatic review on every commit
- **SARIF Output** — `--format sarif` for GitHub Code Scanning integration
- **4 Output Formats** — pretty (colored), compact, JSON, SARIF
- **Project Config** — `.cora.yaml` per-project configuration with provider, focus, rules, ignore, and hook settings
- **Environment Variables** — `CORA_API_KEY`, `CORA_MODEL`, `CORA_PROVIDER`, `CORA_BASE_URL`, `CORA_CONFIG`, `CORA_FORMAT`
- **Severity Levels** — `info`, `minor`, `major`, `critical` with configurable thresholds
- **Focus Areas** — `security`, `performance`, `bugs`, `best_practice`, `maintainability`
- **Ignore Rules** — file patterns and rule-level exclusions
- **Cross-platform** — Linux (x86_64, ARM64), macOS (Apple Silicon), Windows (x86_64)
- **MIT License** — fully open source

[Unreleased]: https://github.com/ajianaz/cora-cli/compare/v0.1.8...develop
[0.1.8]: https://github.com/ajianaz/cora-cli/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/ajianaz/cora-cli/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/ajianaz/cora-cli/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/ajianaz/cora-cli/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/ajianaz/cora-cli/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/ajianaz/cora-cli/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/ajianaz/cora-cli/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/ajianaz/cora-cli/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/ajianaz/cora-cli/releases/tag/v0.1.0
