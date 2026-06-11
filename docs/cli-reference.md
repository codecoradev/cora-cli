---
title: CLI Reference
---

# CLI Reference

Complete command reference for the cora CLI.

## Global Flags

| Flag | Description |
|------|-------------|
| `--config` `<path>` | Override config file path (default: `.cora.yaml`) |
| `--format` `<fmt>` | Output format: pretty, json, compact, sarif |
| `--no-color` | Disable colored output |
| `--provider` `<name>` | Override provider |
| `--model` `<name>` | Override model |
| `--base-url` `<url>` | Override API base URL |
| `--api-key` `<key>` | Override API key |
| `--verbose` | Enable debug logging |

## Commands

| Command | Description |
|---------|-------------|
| `cora init` | Create `.cora.yaml` config file |
| `cora review` | Review code changes (default: staged files) |
| `cora review --staged` | Review staged git changes explicitly |
| `cora review --unstaged` | Review unstaged working changes |
| `cora review --unpushed` | Review unpushed commits |
| `cora review --base` `<branch>` | Compare current branch against target |
| `cora review --commit` `<ref>` | Review specific commit or range |
| `cora review --diff-file` `<path>` | Review from a diff file |
| `cora review --upload` | Review and upload SARIF to GitHub Code Scanning |
| `cora scan` `<path>` | Scan files for issues |
| `cora scan .` `[--incremental]` | Scan only changed files |
| `cora config show` | Show resolved configuration |
| `cora config show --global` | Show global config (`~/.cora/config.yaml`) |
| `cora config show --project` | Show project config (`.cora.yaml`) |
| `cora config set` `<key>` `<value>` | Set a config value |
| `cora hook install` | Install pre-commit hook |
| `cora hook uninstall` | Remove pre-commit hook |
| `cora auth login` | Save API key to `~/.cora/auth.toml` |
| `cora auth status` | Check current auth status |
| `cora auth remove` | Remove stored API key |
| `cora providers` | List detected AI providers |
| `cora upload-sarif` `<file>` | Upload SARIF to GitHub Code Scanning |
| `cora debt` | Show tech debt report from review history |
| `cora debt --json` | Debt report as JSON (for CI/dashboards) |
| `cora debt --trend` | Quality score trend graph |
| `cora debt --badge` | Shields.io badge JSON endpoint |
| `cora debt --estimate` | Show estimated fix time |
| `cora debt --since v0.4.5` | Filter by git tag or date |
| `cora debt --branch main` | Filter by branch |
| `cora completion` `<shell>` | Generate shell completions (bash/zsh/fish) |
| `cora mcp` | Start MCP server for AI coding agents (Claude Code, Cursor, Windsurf) |
| `cora debt` | Show tech debt report from review history |
| `cora debt --json` | Debt report in JSON format |
| `cora debt --trend` | Quality score trend graph |
| `cora debt --since <date\|tag>` | Filter by date or git tag |
| `cora debt --branch <name>` | Filter by branch name |

## Quick Examples

```bash
# Review staged changes (what's about to be committed)
$ cora review --staged
```

```bash
# Compare your feature branch against main
$ cora review --base main
```

```bash
# Full project scan with incremental caching
$ cora scan --incremental
```

```bash
# Install pre-commit hook
$ cora hook install
```
