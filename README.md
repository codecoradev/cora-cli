<div align="center">

<img src="assets/icon.png" alt="cora" width="120" />

**AI-Powered Code Review CLI**

[![CI](https://github.com/ajianaz/cora-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/ajianaz/cora-cli/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/cora-cli.svg)](https://crates.io/crates/cora-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange.svg)](https://www.rust-lang.org/)

**Cora** is a fast, opinionated CLI tool that uses LLMs to review your code changes — directly in your terminal, CI/CD pipeline, or git hooks.

</div>

---

## ✨ Features

- 🔍 **Git-Aware Scanning** — Automatically detects staged, committed, or changed files
- 🤖 **Multi-LLM Support** — Works with OpenAI, Anthropic, Google, Ollama, and any OpenAI-compatible API
- 🎨 **Beautiful Output** — Colorized, structured review output with severity levels
- 🏗️ **CI/CD Ready** — Designed for GitHub Actions, GitLab CI, and any pipeline
- ⚡ **Fast & Lightweight** — Native Rust binary, no runtime dependencies
- 📋 **SARIF Output** — Upload results to GitHub Code Scanning
- 🔧 **Configurable** — YAML config file with project-level defaults
- 🪝 **Git Hooks** — Pre-commit integration for instant feedback
- 📊 **Exit Codes** — Non-zero exit on critical findings for pipeline gating

## 📦 Installation

### Cargo (Recommended)

```bash
cargo install cora-cli
```

### Binary Download

Download the latest release from [GitHub Releases](https://github.com/ajianaz/cora-cli/releases):

```bash
# Determine your platform tag from the releases page, e.g.:
#   cora-aarch64-unknown-linux-gnu-v0.1.6.tar.gz
#   cora-x86_64-unknown-linux-gnu-v0.1.6.tar.gz
#   cora-aarch64-apple-darwin-v0.1.6.tar.gz
#   cora-x86_64-pc-windows-msvc-v0.1.6.zip

# Example: Linux aarch64
VERSION=$(curl -s https://api.github.com/repos/ajianaz/cora-cli/releases/latest | grep tag_name | cut -d'"' -f4)
curl -L "https://github.com/ajianaz/cora-cli/releases/download/${VERSION}/cora-aarch64-unknown-linux-gnu-${VERSION}.tar.gz" | tar xz
sudo mv cora /usr/local/bin/
```

> **Tip:** Visit the [Releases page](https://github.com/ajianaz/cora-cli/releases) to find the correct asset name for your platform.

### Homebrew

> 🚧 Homebrew tap is planned — check back soon!

### Build from Source

Requires **Rust 1.85+**.

```bash
git clone https://github.com/ajianaz/cora-cli.git
cd cora-cli
cargo install --path .
```

## 🚀 Quick Start

### 1. Set Your API Key

```bash
export OPENAI_API_KEY="sk-..."
# or
export ANTHROPIC_API_KEY="sk-ant-..."
```

### 2. Initialize Config (Optional)

```bash
cora init
```

### 3. Review Staged Changes

```bash
cora review --staged
```

### 4. Review the Last Commit

```bash
cora review --commit HEAD
```

### 5. Scan the Entire Project

```bash
cora scan
```

## 📖 Commands

### `cora review`

Review code changes using an LLM.

```bash
# Review staged files (default)
cora review

# Review unpushed changes
cora review --unpushed

# Review a range of commits
cora review --commit HEAD~3..HEAD

# Review changes vs a base branch
cora review --base origin/main

# Review a pull request diff from a file
cora review --diff-file pr.diff

# Use a specific model
cora review --model gpt-4o

# Output as SARIF
cora review --format sarif

# Output as JSON
cora review --format json

# Upload SARIF to GitHub Code Scanning (implies --format sarif)
cora review --upload

# Set severity threshold
cora review --severity major

# Quiet mode (machine-readable)
cora review --quiet
```

### `cora scan`

Scan files for code quality issues without requiring git context.

```bash
# Scan current directory
cora scan

# Scan a specific directory
cora scan --path src/

# Scan with focus areas
cora scan --focus security,performance

# Exclude patterns
cora scan --exclude "tests/**" --exclude "examples/**"

# Only scan changed files (incremental)
cora scan --incremental
```

### `cora config`

Manage configuration. Supports both project-level (`.cora.yaml`) and global (`~/.cora/config.yaml`) config.

```bash
# Show current resolved configuration
cora config show

# Set a project-level value (writes to .cora.yaml)
cora config set model claude-sonnet-4-20250514
cora config set base_url https://api.openai.com/v1
cora config set severity major

# Set a global value (writes to ~/.cora/config.yaml)
cora config set --global model gpt-4o-mini
cora config set --global provider anthropic

# Supported keys: model, provider, base_url, format, severity
```

**Priority**: CLI flags → env vars → `.cora.yaml` (project) → `~/.cora/config.yaml` (global) → defaults

### `cora init`

Create a `.cora.yaml` config file in the current directory.

```bash
cora init
```

### `cora completion`

Generate shell completions.

```bash
cora completion bash > ~/.cora-completion.bash
cora completion zsh > ~/.cora-completion.zsh
cora completion fish > ~/.cora-completion.fish
```

### `cora hook`

Manage pre-commit git hooks.

```bash
cora hook install
cora hook uninstall
```

## ⚙️ Configuration

Cora reads configuration from multiple sources in priority order:

```
CLI flags → CORA_* env vars → .cora.yaml (project) → ~/.cora/config.yaml (global) → defaults
```

Create a `.cora.yaml` in your project root, or use `~/.cora/config.yaml` for global settings. Project config always overrides global.

```yaml
# .cora.yaml

# Provider configuration
provider:
  provider: openai          # openai | anthropic | google | ollama | custom
  model: gpt-4o-mini
  base_url: https://api.openai.com/v1   # Override for custom/self-hosted endpoints

# Focus areas for review (empty = all)
focus:
  - security
  - performance
  - bugs
  - best_practice

# Custom rules
rules:
  - "no unwrap"

# Ignore configuration
ignore:
  files:
    - "tests/**"
    - "vendor/**"
    - "*.generated.*"
  rules:
    - "skip-rule-1"

# Hook configuration
hook:
  mode: warn               # warn | block
  min_severity: major       # info | minor | major | critical
  max_diff_size: 51200      # Max diff size in bytes (50 KB)

# Output settings
output:
  format: pretty            # pretty | json | compact | sarif
  color: true
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `OPENAI_API_KEY` | OpenAI API key | — |
| `ANTHROPIC_API_KEY` | Anthropic API key | — |
| `GOOGLE_API_KEY` | Google AI API key | — |
| `CORA_API_KEY` | API key (overrides provider-specific keys) | — |
| `CORA_MODEL` | Override model | — |
| `CORA_PROVIDER` | Override provider | — |
| `CORA_BASE_URL` | Override API base URL | — |
| `CORA_CONFIG` | Path to config file | `.cora.yaml` |
| `CORA_FORMAT` | Output format (`pretty`, `json`, `compact`, `sarif`) | `pretty` |
| `CORA_NO_COLOR` | Disable colored output | — |

### Authentication

API keys can be provided via environment variable (`CORA_API_KEY`), provider-specific env vars (`OPENAI_API_KEY`, etc.), or stored in `~/.cora/auth.toml` (auto-created by `cora auth login`, permission `0600`).

```bash
# Interactive login (stores key in ~/.cora/auth.toml)
cora auth login

# Or set via environment variable
export CORA_API_KEY=sk-...
```

## 🔗 CI/CD Integration

### GitHub Actions

Using the official [cora-review composite action](.github/actions/cora-review):

```yaml
name: CI
on:
  pull_request:
    branches: [develop]

jobs:
  cora-review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: ./.github/actions/cora-review
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          infisical-identity-id: ${{ secrets.INFISICAL_IDENTITY_ID }}
          severity: major
          upload-sarif: 'true'
```

Or install manually:

```yaml
# Manual install in CI
- name: Install cora-cli
  run: |
    curl -fsSL https://github.com/ajianaz/cora-cli/releases/latest/download/cora-x86_64-unknown-linux-gnu.tar.gz | tar xz
    sudo mv cora /usr/local/bin/
```

### GitLab CI

```yaml
# .gitlab-ci.yml
code-review:
  stage: test
  image: rust:latest
  before_script:
    - cargo install cora-cli
  script:
    - cora review --base origin/main --severity major
  variables:
    OPENAI_API_KEY: $CI_OPENAI_API_KEY
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

### Pre-commit Hook

Add cora as a git pre-commit hook for instant feedback:

```bash
# Install as pre-commit hook
cora hook install

# Review only staged files before each commit
# This runs automatically on `git commit`

# Remove the hook
cora hook uninstall
```

Or add it manually to `.git/hooks/pre-commit`:

```bash
#!/bin/sh
# cora-cli pre-commit hook
cora review --quiet --severity major
if [ $? -ne 0 ]; then
  echo "❌ Code review found critical issues. Commit blocked."
  echo "   Run 'cora review' to see details, or use 'git commit --no-verify' to skip."
  exit 1
fi
```

### With [pre-commit](https://pre-commit.com) framework

> 🚧 Planned — the pre-commit hook repo will be available soon. For now, use `cora hook install` directly.

## 🆚 Positioning: How Cora Compares

| Feature | **cora-cli** | AI Agent IDE Tools | Standard Linters |
|---------|:---:|:---:|:---:|
| Semantic code understanding | ✅ | ✅ | ❌ |
| Security vulnerability detection | ✅ | ✅ | ⚠️ (pattern only) |
| Performance suggestions | ✅ | ✅ | ❌ |
| Runs in CI/CD pipeline | ✅ | ❌ | ✅ |
| SARIF / structured output | ✅ | ❌ | ✅ |
| Zero-config quick start | ✅ | ❌ | ⚠️ |
| No IDE required | ✅ | ❌ | ✅ |
| Understands business context | ⚠️ | ✅ | ❌ |
| Near-instant feedback | ⚠️ | ✅ | ✅ |
| Cost per review | 💰 | 💰💰💰 | Free |
| Works with any codebase | ✅ | ⚠️ | ⚠️ |

**cora-cli sits between traditional linters and AI IDE agents**: it provides semantic understanding that static tools can't match, while being lightweight enough to run in any CI pipeline or terminal — no IDE plugin required.

- **vs. Linters (clippy, eslint, etc.)**: Cora understands *intent* and *context*, catching logical errors, security flaws, and design issues that pattern-based tools miss.
- **vs. AI IDE Agents (Copilot, Cursor)**: Cora is pipeline-first — it runs in CI/CD, pre-commit hooks, and headless environments. It's the tool you use when you want AI review baked into your development workflow, not tied to a specific editor.

## 🛠️ Development

Requires **Rust 1.85+**.

```bash
# Build
cargo build

# Test
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) before submitting PRs.

## 📄 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [Clap](https://clap.rs/)
- Powered by state-of-the-art LLMs from [OpenAI](https://openai.com/), [Anthropic](https://www.anthropic.com/), and [Google](https://ai.google/)

---

<div align="center">

**Made with 🦀 by [Anaz S Aji](https://github.com/ajianaz)**

</div>
