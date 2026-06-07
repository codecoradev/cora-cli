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

## 06 — GitHub Actions CI

Add cora to your CI pipeline.

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
      - name: Install cora
        run: cargo install cora-cli
      - name: Run AI code review
        env:
          CORA_API_KEY: ${{ secrets.CORA_API_KEY }}
          CORA_PROVIDER: openai
        run: cora review --base main --format sarif
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
