<div align="center">

<img src="assets/logo.png" alt="CodeCora" width="120" />

**AI-Powered Code Review CLI — BYOK**

[![GitHub stars](https://img.shields.io/github/stars/codecoradev/cora-cli?style=social)](https://github.com/codecoradev/cora-cli/stargazers)
[![CI](https://github.com/codecoradev/cora-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/codecoradev/cora-cli/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/cora-cli.svg)](https://crates.io/crates/cora-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange.svg)](https://www.rust-lang.org/)

</div>

---

**Cora** is a fast, native CLI that uses any LLM to review your code — in your terminal, CI/CD, git hooks, or directly inside AI coding agents. Bring your own key, pick any model, review in seconds.

## Why Cora?

- 🤖 **Multi-LLM** — OpenAI, Anthropic, Groq, Ollama, Z.AI, or any OpenAI-compatible API
- ⚡ **Native Rust** — fast binary, no runtime dependencies, cross-platform
- 🪝 **Pre-commit hooks** — catch issues before they reach CI
- 📋 **SARIF output** — upload to GitHub Code Scanning
- 🛡️ **Deterministic scanners** — 11 security patterns + 12 secret detection patterns that run without LLM
- 🧠 **Language-specific analysis** — tailored review guidance for Dart/Flutter, Svelte, TypeScript, Go, Rust, Python
- 🚧 **Quality gate** — configurable pass/fail thresholds for CI enforcement
- 📐 **Quality profiles** — strict, balanced, or lax presets for different project needs
- 📏 **Custom rule engine** — write your own regex rules in `.cora.yaml`
- ✂️ **Auto-chunking** — splits large PRs into reviewable chunks automatically
- 🔌 **MCP server** — expose rules and config to AI agents (Claude Code, Cursor, Copilot, Windsurf)
- 💾 **Diff-hash caching** — skip repeat reviews automatically
- 🔧 **Configurable** — per-project `.cora.yaml`, global `~/.cora/config.yaml`, or env vars

## Quick Start

### Install

```bash
curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh
```

> Pin a version: `CORA_VERSION=v0.4.6 curl -fsSL ... | sh`  
> Or: `cargo install --git https://github.com/codecoradev/cora-cli`  
> Pre-built binaries: [GitHub Releases](https://github.com/codecoradev/cora-cli/releases)

### Authenticate

```bash
cora auth login
```

Pick a provider, enter your API key. Done. Provider env vars (`ZAI_API_KEY`, `OPENAI_API_KEY`, etc.) are auto-detected.

### Review

```bash
cora review              # staged changes
cora review --base main  # vs a branch
cora review --unpushed   # unpushed commits
```

### Project Config

```bash
cora init  # creates .cora.yaml + installs pre-commit hook
```

## Configuration

**Priority:** CLI flags → env vars → `.cora.yaml` (project) → `~/.cora/config.yaml` (global) → defaults

```yaml
# .cora.yaml
provider: zai
model: glm-5.1
focus: [security, bugs]

# Quality gate — enforce code quality in CI
quality_gate:
  enabled: true
  thresholds:
    max_critical: 0     # 0 critical = gate FAIL
    max_security: 0     # 0 security findings = gate FAIL
  categories:
    performance:
      action: warn      # warn only, don't fail CI
      max_findings: 5
```

```bash
cora config show           # effective merged config
cora config show --global  # ~/.cora/config.yaml
cora config show --project # .cora.yaml
```

| File | Purpose |
|------|---------|
| `~/.cora/auth.toml` | API key (secret, chmod 600) |
| `~/.cora/config.yaml` | Global defaults (provider, model, etc.) |
| `.cora.yaml` | Per-project overrides |

See **[Configuration →](https://codecora.dev/configuration.html)** for full reference.

## CI/CD

[![GitHub Marketplace](https://img.shields.io/badge/Marketplace-Cora%20AI%20Code%20Review-blue?logo=github)](https://github.com/marketplace/actions/cora-ai-code-review)

```yaml
# .github/workflows/cora-review.yml
on: pull_request
jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with: { fetch-depth: 0 }
      - uses: codecoradev/cora-review-action@v1
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          cora-api-key: ${{ secrets.CORA_API_KEY }}
```

Required secrets: `CORA_API_KEY`, `CORA_BASE_URL` (optional), `CORA_MODEL` (optional)

See [GitHub Marketplace](https://github.com/marketplace/actions/cora-ai-code-review) for full documentation.

Works on **all CI platforms** — [Gitea, GitLab, Bitbucket →](https://codecora.dev/examples.html#_07-gitea-forgejo-ci)

## Commands

| Command | Description |
|---------|-------------|
| `cora review` | Review code changes |
| `cora scan` | Scan files for issues |
| `cora init` | Create project config + hook |
| `cora auth login` | Save API key |
| `cora config show` | Show resolved config |
| `cora providers` | List available LLM providers |
| `cora debt` | Show tech debt report from review history |
| `cora mcp` | Start MCP server for AI coding agents |
| `cora hook install` | Install pre-commit hook |

See **[CLI Reference →](https://codecora.dev/cli-reference.html)** for all flags and examples.

## Environment Variables

| Variable | Description |
|----------|-------------|
| `CORA_API_KEY` | API key (CI use) |
| `CORA_PROVIDER` | Override provider |
| `CORA_MODEL` | Override model |
| `CORA_BASE_URL` | Override API base URL |

Provider-specific keys are auto-detected: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `GROQ_API_KEY`, `ZAI_API_KEY`

## Documentation

| Page | Description |
|------|-------------|
| [Getting Started](https://codecora.dev/getting-started.html) | Install, auth, first review |
| [Configuration](https://codecora.dev/configuration.html) | Config files, env vars, priority |
| [CLI Reference](https://codecora.dev/cli-reference.html) | All commands and flags |
| [Providers](https://codecora.dev/providers.html) | Supported LLM providers |
| [Examples](https://codecora.dev/examples.html) | Common workflows & CI setup |
| [Changelog](https://codecora.dev/changelog.html) | Release history |
| [Roadmap](https://codecora.dev/roadmap.html) | Planned features |

## Star History

<a href="https://www.star-history.com/?repos=codecoradev%2Fcora-cli%2Ccodecoradev%2Futeke&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=codecoradev/cora-cli%2Ccodecoradev/uteke&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=codecoradev/cora-cli%2Ccodecoradev/uteke&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=codecoradev/cora-cli%2Ccodecoradev/uteke&type=date&legend=top-left" />
 </picture>
</a>

## Contributing

See **[CONTRIBUTING.md](CONTRIBUTING.md)** for guidelines. PRs welcome!

## License

[MIT](LICENSE)
