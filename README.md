<div align="center">

<img src="assets/icon.png" alt="cora" width="120" />

**AI-Powered Code Review CLI**

[![CI](https://github.com/ajianaz/cora-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/ajianaz/cora-cli/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/cora-cli.svg)](https://crates.io/crates/cora-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange.svg)](https://www.rust-lang.org/)

**Cora** is a fast, opinionated CLI tool that uses LLMs to review your code changes — directly in your terminal, CI/CD pipeline, or git hooks.

🌐 [Website](https://cora.ajianaz.dev)

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

Cora ships with **two reusable composite actions** for GitHub Actions. Both handle downloading the binary from GitHub Releases, running the review, posting PR comments, and optionally uploading SARIF. The difference is how LLM secrets are provided.

### Which Action to Use?

| | **cora-review** | **cora-review-simple** |
|--|:--|:--|
| Secret source | Infisical (OIDC) | GitHub Secrets |
| Keys in GitHub? | ❌ Zero | ✅ 3 secrets |
| Setup complexity | Infisical account needed | Copy-paste |
| Best for | Teams with Infisical | Quick setup, personal repos |

---

### Option A: `cora-review` — Infisical OIDC (recommended)

Zero API keys stored in GitHub. Secrets are pulled at runtime from [Infisical](https://infisical.com/) via OIDC.

#### Step 1: Copy the action

```bash
mkdir -p .github/actions/cora-review
curl -sL https://raw.githubusercontent.com/ajianaz/cora-cli/develop/.github/actions/cora-review/action.yml \
  -o .github/actions/cora-review/action.yml
```

#### Step 2: Add required secrets

| Secret | Description |
|--------|-------------|
| `INFISICAL_IDENTITY_ID` | Infisical OIDC identity ID |
| `CORA_API_KEY` | LLM API key (set in Infisical, not GitHub) |
| `CORA_BASE_URL` | API base URL (set in Infisical, not GitHub) |
| `CORA_MODEL` | Model ID (set in Infisical, not GitHub) |

#### Step 3: Add the CI job

```yaml
# .github/workflows/ci.yml
name: CI

on:
  pull_request:
    branches: [develop]

permissions:
  contents: read
  security-events: write
  pull-requests: write
  id-token: write   # Required for Infisical OIDC

jobs:
  cora-review:
    name: Cora Review
    runs-on: ubuntu-latest
    if: github.event_name == 'pull_request'
    permissions:
      contents: read
      security-events: write
      pull-requests: write
      id-token: write
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - uses: ./.github/actions/cora-review
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          infisical-identity-id: ${{ secrets.INFISICAL_IDENTITY_ID }}
          # Optional:
          # upload-sarif: 'true'        # Enable GitHub Code Scanning
          # cora-version: 'v0.1.2'      # Pin version (default: latest)
          # severity: 'warning'          # Minimum severity (default: major)
          # base-branch: 'origin/main'   # Diff target (default: origin/develop)
```

---

### Option B: `cora-review-simple` — GitHub Secrets

Set API keys directly as GitHub repository secrets. No external service needed.

#### Step 1: Copy the action

```bash
mkdir -p .github/actions/cora-review-simple
curl -sL https://raw.githubusercontent.com/ajianaz/cora-cli/develop/.github/actions/cora-review-simple/action.yml \
  -o .github/actions/cora-review-simple/action.yml
```

#### Step 2: Add required secrets

Go to your repo → **Settings → Secrets and variables → Actions → New repository secret**:

| Secret | Example Value |
|--------|---------------|
| `CORA_API_KEY` | `sk-...` (OpenAI, Anthropic, etc.) |
| `CORA_BASE_URL` | `https://api.openai.com/v1` |
| `CORA_MODEL` | `gpt-4o` |

#### Step 3: Add the CI job

```yaml
# .github/workflows/ci.yml
name: CI

on:
  pull_request:
    branches: [develop]

permissions:
  contents: read
  security-events: write
  pull-requests: write

jobs:
  cora-review:
    name: Cora Review
    runs-on: ubuntu-latest
    if: github.event_name == 'pull_request'
    permissions:
      contents: read
      security-events: write
      pull-requests: write
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - uses: ./.github/actions/cora-review-simple
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          cora-api-key: ${{ secrets.CORA_API_KEY }}
          cora-base-url: ${{ secrets.CORA_BASE_URL }}
          cora-model: ${{ secrets.CORA_MODEL }}
          # Optional:
          # upload-sarif: 'true'        # Enable GitHub Code Scanning
          # cora-version: 'v0.1.2'      # Pin version (default: latest)
          # severity: 'warning'          # Minimum severity (default: major)
          # base-branch: 'origin/main'   # Diff target (default: origin/develop)
```

---

### Configuration Reference (both actions)

| Input | Default | Description |
|-------|---------|-------------|
| `github-token` | *(required)* | `secrets.GITHUB_TOKEN` for PR comments |
| `cora-version` | `latest` | Version tag or `latest` for auto-resolve via API |
| `upload-sarif` | `false` | Upload SARIF to GitHub Code Scanning |
| `severity` | `major` | Minimum severity: `info`, `minor`, `major`, `critical` |
| `base-branch` | `origin/develop` | Branch to diff against |

### How It Works

```
PR opened
  → Resolve cora version (GitHub API if "latest", or use pinned tag)
  → Download cora binary from GitHub Releases
  → Fetch LLM secrets (Infisical OIDC or GitHub Secrets)
  → Run cora review --base <branch> --format sarif
  → Parse SARIF → post structured PR comment (updates on re-run)
  → Optionally upload SARIF to GitHub Code Scanning
  → Exit 1 if error-level findings found (blocks merge)
```

### Block Merge on Critical Findings

Add `Cora Review` as a required status check in your **Branch Protection** or **Repository Ruleset**. When cora finds error-level issues, the job fails and merge is blocked.

### Pre-commit Hook

Add cora as a git pre-commit hook for instant feedback:

```bash
cora hook install
```

Or add it manually to `.git/hooks/pre-commit`:

```bash
#!/bin/sh
cora review --quiet --severity critical
if [ $? -ne 0 ]; then
  echo "❌ Code review found critical issues. Commit blocked."
  exit 1
fi
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

### ⚠️ Troubleshooting

<details>
<summary>Common issues and solutions</summary>

**Q: `tar: Child returned status 1` — binary download fails**

Release archives contain a binary named `cora` (since v0.1.2). Older releases used the target triple name. Always use `latest` or pin to `>=v0.1.2`.

**Q: `latest` version doesn't resolve**

The action resolves `latest` via GitHub API: `GET /repos/ajianaz/cora-cli/releases/latest` → extracts `tag_name`. This requires internet access on the runner. If rate-limited, pin a specific version instead.

**Q: Branch protection blocks merge but no failing checks visible in UI**

GitHub has two layers: **Branch Protection** (API) and **Repository Rulesets** (UI). Ruleset checks may not appear in the PR checks list. Verify via API:

```bash
gh api repos/{owner}/{repo}/rulesets/{id}
gh api repos/{owner}/{repo}/branches/{branch}/protection
```

**Q: Chicken-and-egg — action needs new binary but can't merge to release it**

Temporarily remove `Cora Review` from your ruleset's required checks, merge the fix, release, then re-add.

**Q: Private repos get 403 on Branch Protection API**

GitHub Free doesn't support Branch Protection API for private repos. Use **Repository Rulesets** instead.

**Q: Should cora-cli review itself in CI?**

No — the action downloads the released binary, which doesn't include in-progress changes. Self-review always reviews the *previous* release against itself.

</details>

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
