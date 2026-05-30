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
- 🤖 **Multi-LLM Support** — Works with OpenAI, Anthropic, Google, and any OpenAI-compatible API
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
# Linux x86_64
curl -L https://github.com/ajianaz/cora-cli/releases/latest/download/cora-linux-x86_64.tar.gz | tar xz

# macOS ARM (Apple Silicon)
curl -L https://github.com/ajianaz/cora-cli/releases/latest/download/cora-macos-aarch64.tar.gz | tar xz

# macOS x86_64 (Intel)
curl -L https://github.com/ajianaz/cora-cli/releases/latest/download/cora-macos-x86_64.tar.gz | tar xz

# Windows x86_64
curl -L https://github.com/ajianaz/cora-cli/releases/latest/download/cora-windows-x86_64.tar.gz | tar xz
```

### Build from Source

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

### 2. Review Staged Changes

```bash
cora review
```

### 3. Review a Specific Commit

```bash
cora review --commit HEAD
```

### 4. Scan the Entire Project

```bash
cora scan .
```

## 📖 Commands

### `cora review`

Review code changes using an LLM.

```bash
# Review staged files (default)
cora review

# Review staged explicitly
cora review --staged

# Review unstaged changes
cora review --unstaged

# Review unpushed commits
cora review --unpushed

# Review against a branch
cora review --base develop

# Review a commit or range
cora review --commit HEAD
cora review --commit HEAD~3..HEAD

# Review from a diff file
cora review --diff-file pr.diff

# Use a specific model
cora review --model gpt-4o

# Filter by severity
cora review --severity major

# Quiet mode (minimal output)
cora review --quiet

# Stream response
cora review --stream

# Output as SARIF
cora review --format sarif

# Review and upload SARIF to GitHub Code Scanning
cora review --base develop --upload

# Output as JSON
cora review --format json
```

### `cora scan`

Scan files for code quality issues without requiring git context.

```bash
# Scan current directory
cora scan .

# Scan with focus areas
cora scan . --focus security,performance

# Exclude patterns
cora scan . --exclude "tests/**" --exclude "vendor/**"

# Incremental (only changed files)
cora scan . --incremental
```

### `cora config`

Manage configuration.

```bash
# Show current configuration
cora config show

# Set model
cora config set model gpt-4o

# Set provider
cora config set provider anthropic
```

### `cora init`

Create a `.cora.yaml` config file in your project.

```bash
cora init
cora init --force
```

### `cora auth`

Manage API key authentication.

```bash
cora auth login
cora auth status
cora auth remove
```

### `cora hook`

Manage git hooks.

```bash
cora hook install
cora hook uninstall
```

### `cora providers`

List detected AI providers.

```bash
cora providers
```

### `cora upload-sarif`

Upload a SARIF file to GitHub Code Scanning.

```bash
cora upload-sarif results.sarif
```

### `cora completion`

Generate shell completions.

```bash
cora completion bash
cora completion zsh
cora completion fish
```

## ⚙️ Configuration

Create a `.cora.yaml` in your project root. API keys are stored at `~/.cora/config.toml`.

```yaml
# .cora.yaml

# Review settings
review:
  severity: warning        # minimum severity level
  max_issues: 20           # max issues to report
  focus: security,performance  # focus areas

# Patterns to ignore
ignore:
  - "vendor/**"
  - "*.min.js"
  - "migrations/**"

# Provider-specific model overrides
providers:
  openai:
    model: gpt-4o
  anthropic:
    model: claude-sonnet-4-20250514
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `CORA_API_KEY` | Generic API key |
| `OPENAI_API_KEY` | OpenAI API key |
| `ANTHROPIC_API_KEY` | Anthropic API key |
| `GROQ_API_KEY` | Groq API key |
| `ZAI_API_KEY` | Z.AI API key |
| `CORA_PROVIDER` | Override provider |
| `CORA_MODEL` | Override model |
| `CORA_BASE_URL` | Override API base URL |
| `CORA_CONFIG` | Config file path |
| `CORA_FORMAT` | Output format |
| `CORA_NO_COLOR` | Disable colors |
| `GITHUB_TOKEN` | GitHub token for SARIF upload |
| `GITHUB_REPOSITORY` | GitHub repo for SARIF upload |
| `GITHUB_REF` | GitHub ref for SARIF upload |

## 🔗 CI/CD Integration

### GitHub Actions

```yaml
# .github/workflows/review.yml
name: Code Review

on:
  pull_request:
    types: [opened, synchronize, reopened]

jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install cora-cli
        run: cargo install cora-cli

      - name: Run code review
        env:
          OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
        run: |
          cora review --base origin/develop --format sarif --output-file results.sarif

      - name: Upload SARIF to GitHub Code Scanning
        if: always()
        uses: github/codeql-action/upload-sarif@v3
        with:
          sarif_file: results.sarif
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
    - cora review --base origin/develop --severity critical
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
cora review --quiet --severity critical
if [ $? -ne 0 ]; then
  echo "❌ Code review found critical issues. Commit blocked."
  echo "   Run 'cora review' to see details, or use 'git commit --no-verify' to skip."
  exit 1
fi
```

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
