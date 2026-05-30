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

### Homebrew

```bash
brew tap ajianaz/tap
brew install cora-cli
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

### 3. Review a Specific File

```bash
cora review src/main.rs
```

### 4. Review the Last Commit

```bash
cora review --commit HEAD
```

### 5. Scan the Entire Project

```bash
cora scan .
```

## 📖 Commands

### `cora review`

Review code changes using an LLM.

```bash
# Review staged files
cora review

# Review specific files
cora review src/main.rs src/lib.rs

# Review a range of commits
cora review --commit HEAD~3..HEAD

# Review a pull request diff
cora review --diff-file pr.diff

# Use a specific model
cora review --model gpt-4o

# Output as SARIF (for CI)
cora review --output sarif --output-file results.sarif

# Output as JSON
cora review --output json

# Set severity threshold
cora review --severity warning

# Quiet mode (machine-readable)
cora review --quiet
```

### `cora scan`

Scan files for code quality issues without requiring git context.

```bash
# Scan current directory
cora scan .

# Scan specific files
cora scan src/**/*.rs

# Scan with custom rules focus
cora scan . --focus security,performance

# Exclude patterns
cora scan . --exclude "tests/**" --exclude "examples/**"
```

### `cora config`

Manage configuration.

```bash
# Show current configuration
cora config show

# Initialize config file
cora config init

# Set a configuration value
cora config set model claude-sonnet-4-20250514
cora config set severity error
```

### `cora completion`

Generate shell completions.

```bash
cora completion bash > ~/.cora-completion.bash
cora completion zsh > ~/.cora-completion.zsh
cora completion fish > ~/.cora-completion.fish
```

## ⚙️ Configuration

Create a `.cora.yaml` in your project root or `~/.config/cora/config.yaml` globally:

```yaml
# .cora.yaml
model: gpt-4o
temperature: 0.1

# Provider configuration
provider:
  name: openai          # openai | anthropic | google | custom
  base_url: null        # Override API endpoint (for custom/self-hosted)
  api_key_env: OPENAI_API_KEY  # Environment variable for API key

# Review settings
review:
  severity: warning      # error | warning | info | note
  focus:                 # Focus areas (empty = all)
    - security
    - performance
    - maintainability
  ignore:
    patterns:
      - "tests/**"
      - "vendor/**"
      - "*.generated.*"
    rules:
      - "line-too-long"
  max_tokens: 4096
  language: en            # Response language

# Output settings
output:
  format: colored         # colored | plain | json | sarif
  file: null              # Output to file instead of stdout

# Git settings
git:
  auto_stage: false       # Auto-stage files after review
  base_branch: main       # Base branch for PR reviews
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `OPENAI_API_KEY` | OpenAI API key | — |
| `ANTHROPIC_API_KEY` | Anthropic API key | — |
| `GOOGLE_API_KEY` | Google AI API key | — |
| `CORa_MODEL` | Override model | — |
| `CORa_PROVIDER` | Override provider | — |
| `CORa_CONFIG` | Path to config file | `.cora.yaml` |
| `CORa_LOG_LEVEL` | Log verbosity | `info` |

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
          cora review --commit origin/main..HEAD \
            --output sarif \
            --output-file results.sarif

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
    - cora review --commit origin/main..HEAD --severity error
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
cora review --quiet --severity error
if [ $? -ne 0 ]; then
  echo "❌ Code review found critical issues. Commit blocked."
  echo "   Run 'cora review' to see details, or use 'git commit --no-verify' to skip."
  exit 1
fi
```

### With [pre-commit](https://pre-commit.com) framework

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/ajianaz/cora-cli
    rev: v0.1.0
    hooks:
      - id: cora-review
        args: ['--severity', 'warning']
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
