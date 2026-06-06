<div align="center">

<img src="assets/logo.png" alt="CodeCora" width="120" />

**AI-Powered Code Review CLI**

[![CI](https://github.com/codecoradev/cora-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/codecoradev/cora-cli/actions/workflows/ci.yml)
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
- 🧠 **Deterministic Reviews** — Temperature 0 by default: same diff always produces the same issues
- 💾 **Diff-Hash Caching** — Reviews cached by diff hash in `~/.cache/cora/reviews/` — skip repeat reviews with `--no-cache`
- 🎯 **Custom System Prompts** — Override review/scan prompts via config or file path
- 🛡️ **Anti-Hallucination** — File path injection and post-parse filtering keep LLM output grounded
- 🌡️ **Configurable LLM Params** — Tune temperature, max tokens, timeout, and cache TTL per project

## 📦 Installation

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh
```

Installs to `~/.local/bin`. Add to PATH if needed:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
```

Pin a specific version with `CORA_VERSION`:

```bash
CORA_VERSION=v0.4.2 curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh
```

### Pre-built Binaries

Download from [GitHub Releases](https://github.com/codecoradev/cora-cli/releases):

| Platform | Asset |
|----------|-------|
| Linux x86_64 | `cora-x86_64-unknown-linux-gnu-v*.tar.gz` |
| Linux ARM64 | `cora-aarch64-unknown-linux-gnu-v*.tar.gz` |
| macOS Apple Silicon | `cora-aarch64-apple-darwin-v*.tar.gz` |
| Windows x86_64 | `cora-x86_64-pc-windows-msvc-v*.zip` |

> **Windows users:** Extract the zip and place `cora.exe` somewhere in your PATH (e.g. `C:\Users\<you>\.local\bin`). Run from Command Prompt, PowerShell, or Windows Terminal — do not double-click the `.exe` (it will flash and close).

### Cargo

```bash
cargo install --git https://github.com/codecoradev/cora-cli
```

### Homebrew

> 🚧 Homebrew tap is planned — see [#151](https://github.com/codecoradev/cora-cli/issues/151).

### Build from Source

Requires **Rust 1.85+**.

```bash
git clone https://github.com/codecoradev/cora-cli.git
cd cora-cli
cargo install --path .
```

## 🚀 Quick Start

### 1. Install

```bash
curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh
```

### 2. Set Up Authentication

```bash
cora auth login
```

Pick your provider, enter your API key — done. Cora stores it securely in `~/.cora/auth.toml` (never committed to git).

> **No API key yet?** Set any provider env var instead: `export OPENAI_API_KEY="..."`

### 3. Review Your Code

```bash
# Review staged changes
cora review

# Review vs a branch
cora review --base origin/main

# Review the last commit
cora review --commit HEAD
```

### 4. Optional — Project Config

```bash
cora init
```

Creates `.cora.yaml` in your project root and **automatically installs a pre-commit hook**. The hook runs `cora review --staged --format compact` before each commit. Use `--no-hook` to skip.

```bash
# Skip hook installation
cora init --no-hook

# Manage hooks separately
cora hook install
cora hook uninstall
```

### 5. Provider Shortcuts (v0.4.2+)

You can set `model`, `base_url`, and `provider` directly in `.cora.yaml`:

```yaml
provider: openai
model: gpt-4o-mini
base_url: https://api.openai.com/v1
```

No need for nested `provider:` section. Run `cora config show` to verify resolved config.

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

# CI mode: skip diff size limit, hard gate on any findings
cora review --ci --base ${{ github.base_ref }}

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

# Skip cached reviews
cora review --no-cache
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

# Provider configuration (shortcut format — v0.4.2+)
provider: openai
model: gpt-4o-mini
base_url: https://api.openai.com/v1

# Or use nested format for multiple keys:
# provider:
#   provider: openai
#   model: gpt-4o-mini
#   base_url: https://api.openai.com/v1

# LLM parameters
llm:
  temperature: 0              # Default: 0 (deterministic — same diff = same issues)
  max_tokens: 4096            # Default: 4096
  timeout: 120                # Default: 120 (seconds)
  cache_ttl: 1440             # Default: 1440 (minutes) — diff-hash cache TTL

# Focus areas for review (empty = all)
focus:
  - security
  - performance
  - bugs
  - best_practice

# Review options
review:
  system_prompt: "You are a senior Rust code reviewer."
  # system_prompt_file: ./review-prompt.md   # Load prompt from file
  response_format: json_object              # Opt-in structured JSON output

# Scan options
# scan:
#   system_prompt: "Focus on security vulnerabilities."
#   system_prompt_file: ./scan-prompt.md

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
  on_violation: warn        # warn | disallow — block commit on oversized diff

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
| `CORA_NO_CACHE` | Skip diff-hash cache (same as `--no-cache`) | — |

### Authentication

API keys can be provided via environment variables, stored in `~/.cora/auth.toml` (auto-created by `cora auth login`, permission `0600`), or passed via CLI flags.

#### Interactive Login (Recommended)

```bash
cora auth login
```

This starts an interactive setup that:

1. **Lists known providers** — OpenAI, Anthropic, Groq, Ollama, Z.AI
2. **Lets you pick one** — known providers only need an API key
3. **Or choose "custom"** — enter your own base URL, model, and API key for any OpenAI-compatible endpoint

Example flow:

```
$ cora auth login

🔑 Cora Auth Setup
   Choose your LLM provider:

  [1] openai — https://api.openai.com/v1 (model: gpt-4o-mini)
  [2] anthropic — https://api.anthropic.com/v1 (model: claude-3-haiku-20240307)
  [3] groq — https://api.groq.com/openai/v1 (model: llama-3.1-8b-instant)
  [4] ollama — http://localhost:11434/v1 (model: llama3.1)
  [5] zai — https://api.z.ai/api/coding/paas/v4 (model: glm-5.1)
  [6] custom — use any OpenAI-compatible endpoint

  Select provider [1-6]: 1

  → Provider: openai
  → Model: gpt-4o-mini
  → Base URL: https://api.openai.com/v1

  🔑 Enter your API key: sk-...

✅ API key saved to ~/.cora/auth.toml
   Provider: openai | Model: gpt-4o-mini | Base: https://api.openai.com/v1
```

#### Check Auth Status

```bash
cora auth status
```

Shows your configured provider, model, and key source.

#### Environment Variables

| Variable | Description |
|----------|-------------|
| `CORA_API_KEY` | API key (overrides all other sources) |
| `OPENAI_API_KEY` | OpenAI API key (auto-detected) |
| `ANTHROPIC_API_KEY` | Anthropic API key (auto-detected) |
| `GROQ_API_KEY` | Groq API key (auto-detected) |
| `ZAI_API_KEY` | Z.AI API key (auto-detected) |
| `CORA_PROVIDER` | Override provider name |
| `CORA_MODEL` | Override model |
| `CORA_BASE_URL` | Override API base URL |

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
# Manual install in CI (using install script)
- name: Install cora-cli
  run: curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh
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
