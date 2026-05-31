# Changelog

All notable changes to cora-cli are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/ajianaz/cora-cli/compare/v0.1.2...develop
[0.1.2]: https://github.com/ajianaz/cora-cli/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/ajianaz/cora-cli/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/ajianaz/cora-cli/releases/tag/v0.1.0
