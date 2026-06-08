---
title: Examples
---

# Examples

Practical examples to get you started with cora.

## 01 — Quick Review

Review your staged changes before committing.

```bash
# Review staged changes (default)
$ cora review

# Or review with explicit flags
$ cora review --staged
```

## 02 — Branch Comparison

Compare your current branch against main.

```bash
$ cora review --base main
```

## 03 — Full Project Scan

Scan your entire project for issues.

```bash
$ cora scan
```

## 04 — Incremental Scan

Only scan files that changed since the last scan.

```bash
$ cora scan --incremental
```

## 05 — Streaming Output

Stream results as they come in from the LLM.

```bash
$ cora review --staged --stream
```

## 06 — GitHub Actions CI (Recommended)

The easiest way to add cora to your PR workflow. This reusable action installs cora, runs the review, posts a PR comment with findings, and optionally uploads SARIF to GitHub Code Scanning.

### Setup

1. Add these secrets to your repository (**Settings → Secrets and variables → Actions**):

| Secret | Description | Example |
|--------|-------------|---------|
| `CORA_API_KEY` | Your LLM API key | `sk-...` |
| `CORA_BASE_URL` | LLM API base URL (optional) | `https://api.openai.com/v1` |
| `CORA_MODEL` | LLM model ID (optional) | `gpt-4o-mini` |

2. Create the workflow file:

```yaml
# .github/workflows/cora-review.yml
name: Cora AI Code Review

on:
  pull_request_target:
    branches: [main, develop]
    types: [opened, synchronize, ready_for_review, reopened]

concurrency:
  group: cora-review-${{ github.event.pull_request.number }}
  cancel-in-progress: true

permissions:
  contents: read
  pull-requests: write
  security-events: write

jobs:
  cora-review:
    name: Cora Review
    runs-on: ubuntu-latest
    timeout-minutes: 10
    steps:
      - name: Checkout PR head
        uses: actions/checkout@v4
        with:
          ref: ${{ github.event.pull_request.head.sha }}
          fetch-depth: 0
          persist-credentials: false

      - name: Run Cora AI Code Review
        uses: codecoradev/cora-review-action@v1
        with:
          github-token: ${{ secrets.GITHUB_TOKEN }}
          cora-api-key: ${{ secrets.CORA_API_KEY }}
          cora-base-url: ${{ secrets.CORA_BASE_URL }}
          cora-model: ${{ secrets.CORA_MODEL }}
```

### Options

| Input | Default | Description |
|-------|---------|-------------|
| `base-branch` | `origin/develop` | Base branch to compare against |
| `severity` | `major` | Minimum severity to report (`info`, `minor`, `major`, `critical`) |
| `cora-version` | `latest` | Pin a specific version (e.g., `v0.4.6`) |
| `upload-sarif` | `true` | Upload findings to GitHub Code Scanning |

### Custom Configuration

Add a `.cora.yaml` to your project root to customize review behavior:

```yaml
# .cora.yaml
focus:
  - security
  - bugs
  - performance

rules:
  - No unwrap() in production code
  - All public functions must have error handling

ignore:
  files:
    - "vendor/**"
    - "**/*.generated.*"
  rules:
    - "style"
```

### How It Works

The action automatically:

1. **Resolves** the latest cora-cli version from GitHub releases
2. **Downloads** the binary with retry + checksum verification
3. **Runs** `cora review` on the PR diff
4. **Posts** a formatted comment on the PR with findings
5. **Uploads** SARIF results to GitHub Code Scanning (optional)
6. **Fails** the job if blocking issues (severity ≥ `major`) are found

### Minimal Setup (No Action)

If you prefer to run cora directly without the reusable action:

```yaml
# .github/workflows/cora-review.yml
name: Code Review

on:
  pull_request:
    branches: [main]

jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: Install and run cora
        env:
          CORA_API_KEY: ${{ secrets.CORA_API_KEY }}
        run: |
          curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh
          cora review --base origin/main --format sarif
```

## 07 — Pre-commit Hook

Install once, then every commit gets reviewed automatically.

```bash
# Install the hook
$ cora hook install

# Now just commit normally — cora reviews automatically
$ git commit -m "fix: handle edge case in parser"
cora pre-commit hook running...
No issues found — commit allowed
```

## 08 — SARIF Upload

Generate SARIF output and upload to GitHub Code Scanning.

```bash
# Generate SARIF report and upload
$ cora review --base main --format sarif > results.sarif && \
  cora upload-sarif results.sarif

Uploaded 3 findings to GitHub Code Scanning
```
