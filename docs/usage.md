---
title: Usage
---

# Usage

## Review Modes

cora supports multiple review modes, each suited to a different workflow:

| Mode | Flag | Scope | Best For |
|------|------|-------|----------|
| Default | *(no flag)* | Tries staged first, then unpushed | Quick feedback |
| Staged | `--staged` | Files in git staging area | Pre-commit review |
| Unstaged | `--unstaged` | Unstaged working changes | Review work in progress |
| Unpushed | `--unpushed` | Commits not yet pushed | Review before push |
| Base Branch | `--base <branch>` | Diff against base branch | PR review workflow |
| Commit | `--commit <ref>` | Specific commit or range | Review specific changes |
| Diff File | `--diff-file <path>` | External diff file | Review patch files |

```bash
# Review staged changes (default)
$ cora review

# Review against a branch
$ cora review --base main

# Review a specific commit
$ cora review --commit HEAD

# Review from a diff file
$ cora review --diff-file pr.diff

# Full project scan (use cora scan)
$ cora scan .
```

## Output Formats

cora can output results in three formats:

- `--format pretty` — Human-readable terminal output (default)
- `--format json` — Machine-readable JSON for CI/CD pipelines
- `--format sarif` — SARIF format for GitHub Code Scanning

```bash
# JSON output example
$ cora review --staged --format json

{
  "files": [
    {
      "path": "src/auth/login.ts",
      "issues": [
        {
          "line": 42,
          "severity": "warning",
          "message": "Potential SQL injection"
        }
      ]
    }
  ],
  "summary": {
    "total_files": 3,
    "total_issues": 3
  }
}
```

## Configuration File

The `.cora.yaml` file provides persistent configuration. Place it in your project root. API keys are stored at `~/.cora/config.toml`.

```yaml
# .cora.yaml — example

review:
  severity: warning
  focus: security,performance

ignore:
  - "vendor/**"
  - "*.min.js"

providers:
  openai:
    model: gpt-4o
```

## Environment Variables

Environment variables override configuration file settings:

| Variable | Description | Required |
|----------|-------------|----------|
| `CORA_API_KEY` | API key for the configured provider | Yes (unless using cora auth) |
| `CORA_PROVIDER` | Override the LLM provider | No |
| `CORA_MODEL` | Override the model name | No |
| `CORA_BASE_URL` | Override the API base URL | No |
| `CORA_CONFIG` | Path to alternative config file | No |

## Working with Monorepos

cora works well in monorepo setups. Use include patterns to limit review scope to specific packages:

```bash
# Review only the backend package
$ cora review --staged --include "packages/backend/**"

# Exclude test and generated files
$ cora review --staged --exclude "**/*.test.ts" --exclude "**/generated/**"
```

Alternatively, set include/exclude patterns in `.cora.yaml` for persistent configuration.

## Exit Codes

cora uses standard exit codes for scripting and CI integration:

| Code | Meaning | CI Behavior |
|------|---------|-------------|
| `0` | No issues found | Pass |
| `1` | Issues found | Fail (warning/error) |
| `2` | Review blocked | Fail (auth/config error) |
| `3` | Authentication error | Fail (missing API key) |

## Tips

- Use `cora review` with no flags for the fastest pre-commit feedback
- Combine `--format json` with `--base main` in CI pipelines
- Use `cora scan . --incremental` for large codebases — only changed files are analyzed
- Use `--quiet` for minimal output and `--severity` to filter by severity level
- Use `cora auth login` to store API keys securely instead of environment variables
