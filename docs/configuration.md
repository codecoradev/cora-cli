---
title: Configuration
---

# Configuration

cora uses a layered config system with clear separation of concerns. Later sources override earlier ones.

## File Roles

| File | Contents | Used by |
|------|----------|--------|
| `~/.cora/auth.toml` | API key only (secret, chmod 600) | Local dev |
| `~/.cora/config.yaml` | Provider, model, base_url, focus, hook, output, etc. | Global default |
| `.cora.yaml` | Per-project config overrides | Project + CI |
| `CORA_API_KEY` env var | API key for CI/one-shot | CI only |

## Config Resolution Order

Settings are resolved in this order (highest priority first):

1. **CLI flags** — `--provider`, `--model`, `--base-url`, etc.
2. **Environment variables** — `CORA_PROVIDER`, `CORA_MODEL`, `CORA_BASE_URL`
3. **.cora.yaml** — Project root config file
4. **~/.cora/config.yaml** — Global config
5. **Auto-detect** — Provider-specific env vars (`OPENAI_API_KEY`, `ZAI_API_KEY`, etc.)
6. **Built-in defaults** — Sensible defaults for all settings

### API Key Resolution

1. `--api-key` flag (one-shot)
2. `CORA_API_KEY` env var (CI)
3. `~/.cora/auth.toml` (local dev)
4. Provider-specific env vars (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, etc.)

## .cora.yaml Example

Create this file in your project root. Run `cora init` to generate it.

```yaml
# cora project config
provider:
  provider: openai
  model: gpt-4o
  base_url: https://api.openai.com/v1

llm:
  temperature: 0
  max_tokens: 4096
  timeout: 120
  cache_ttl: 1440

review:
  system_prompt: "You are a senior code reviewer."
  # system_prompt_file: ./review-prompt.md
  response_format: json_object

focus: security, performance, bugs

hook:
  mode: warn
  min_severity: major
  max_diff_size: 51200

ignore:
  files:
    - "vendor/**"
    - "*.min.js"
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `CORA_API_KEY` | API key for CI (overrides auth.toml) |
| `CORA_PROVIDER` | Active provider (openai, anthropic, groq, ollama, zai) |
| `CORA_MODEL` | Model name override |
| `CORA_BASE_URL` | Custom API base URL |
| `CORA_CONFIG` | Path to config file |
| `CORA_FORMAT` | Output format (pretty, json, compact, sarif) |
| `CORA_NO_COLOR` | Disable colored output |
| `CORA_NO_CACHE` | Skip diff-hash review cache (same as `--no-cache`) |
| `GITHUB_TOKEN` | GitHub token for SARIF upload |
| `GITHUB_REPOSITORY` | GitHub repo for SARIF upload |
| `GITHUB_REF` | GitHub ref for SARIF upload |

## Provider-Specific Env Vars

Each provider has its own API key variable. cora checks these for auto-detection.

```bash
# OpenAI
OPENAI_API_KEY=sk-...
OPENAI_BASE_URL=https://api.openai.com/v1

# Anthropic
ANTHROPIC_API_KEY=sk-ant-...

# Groq
GROQ_API_KEY=gsk_...

# Ollama (local, no key needed)
OLLAMA_HOST=http://localhost:11434
# Optional: OLLAMA_API_KEY if your Ollama instance requires auth
OLLAMA_API_KEY=...

# Z.AI
ZAI_API_KEY=...
```

## Diff-Hash Caching

cora caches review results by diff hash in `~/.cache/cora/reviews/`. If you re-review the same diff, the cached result is returned instantly.

| Setting | Description |
|---------|-------------|
| `llm.cache_ttl` | TTL in minutes (default: 1440 / 24h) |
| `--no-cache` or `CORA_NO_CACHE=1` | Bypass cache |

## Custom System Prompts

Override the default system prompt for `review` or `scan` commands to match your project's coding standards and review criteria.

```yaml
review:
  system_prompt: "Focus on Rust idioms and error handling."
  # Or load from a file:
  system_prompt_file: ./prompts/review.md

scan:
  system_prompt: "Check for OWASP Top 10 vulnerabilities."
  system_prompt_file: ./prompts/scan.md
```

If both `system_prompt` and `system_prompt_file` are set, the file takes precedence.

## Response Format (JSON Mode)

Opt into structured JSON output from the LLM by setting `review.response_format` to `json_object`. This instructs the LLM to return valid JSON, enabling machine-readable parsing and pipeline integration.

```yaml
review:
  response_format: json_object
```

Requires provider support for structured output. Works with OpenAI, Anthropic, and compatible APIs.

## Anti-Hallucination

cora uses two mechanisms to prevent the LLM from fabricating findings:

- **File path injection** — Actual file paths are embedded in the prompt, anchoring the LLM to real files in the diff.
- **Post-parse filtering** — After parsing, any reported file paths or line numbers that don't exist in the actual diff are discarded.

## Quality Gate

Quality gate evaluates review findings against configurable thresholds to produce a **PASS/FAIL** result. This is useful for CI enforcement — block merges when code quality drops below your standards.

```yaml
quality_gate:
  enabled: true

  # Global thresholds — any exceeded = FAIL
  thresholds:
    max_critical: 0        # 0 critical issues allowed
    max_major: 3           # max 3 major issues (disabled by default)
    max_minor: 10          # max 10 minor issues (disabled by default)
    max_security: 0        # 0 security findings allowed

  # Per-category overrides
  categories:
    security:
      action: block        # block = any finding → CI fail
      max_findings: 0
    performance:
      action: warn         # warn = comment only, don't fail CI
      max_findings: 5
    bug_risk:
      action: block
      max_findings: 3
    style:
      action: ignore       # skip entirely — don't count toward gate
```

### How It Works

1. After review, findings are counted by severity and category
2. Each threshold is checked against actual counts
3. Category actions determine enforcement:
   - **block** — exceed threshold = gate FAIL (exit code 2)
   - **warn** — report but don't fail gate
   - **ignore** — skip entirely
4. Overall gate status: **PASSED** or **FAILED**

### CLI Output

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  QUALITY GATE RESULT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Status:   ❌ FAILED
  Findings: 2 critical, 1 major, 4 minor, 0 info

  Threshold Checks:
  ❌ max_critical          → 2 found   ❌ EXCEEDED
  ✅ max_major             → 1 found   ✅ OK
  ✅ max_security          → 0 found   ✅ OK
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Exit Codes

| Code | Meaning |
|------|----------|
| 0 | Gate passed, no issues |
| 2 | Gate failed (threshold exceeded) |

### Default Behavior

When `quality_gate.enabled` is `false` (default), quality gate is skipped. The existing `--ci` flag and `hook.on_violation` settings continue to work as before.

## Debt Tracking

cora tracks review findings over time and reports quality trends. After each review, a lightweight JSON snapshot is saved to `.cora/history/`. Use `cora debt` to view the aggregated report.

```yaml
debt:
  enabled: true              # enable auto-save of review snapshots
  history_dir: .cora/history # directory for snapshot files
  retention_days: 90         # auto-cleanup old snapshots
```

### CLI Usage

```bash
cora debt                    # show debt report table
cora debt --json             # machine-readable JSON
cora debt --trend            # quality score trend graph
cora debt --badge            # shields.io badge JSON
cora debt --estimate         # show estimated fix time
cora debt --since v0.4.5     # filter by git tag or date
```

### Quality Score

Ranges from 0–10 (10 = no issues). Penalties per finding:

| Severity | Penalty |
|----------|--------|
| Critical | -2.0 |
| Major | -1.0 |
| Minor | -0.3 |
| Info | -0.1 |

### Badge Integration

Use `cora debt --badge` output as a shields.io endpoint:

```markdown
[![Quality](https://img.shields.io/endpoint?url=https://example.com/cora-badge.json)]()
```

## Secrets Pre-Scan

cora runs a deterministic secrets scan before the AI review. 12 built-in patterns detect leaked credentials:

| Pattern | Severity |
|---------|----------|
| AWS Access Key (`AKIA...`) | Critical |
| GitHub Token (`ghp_`/`gho_`/`ghu_`) | Critical |
| OpenAI API Key (`sk-`/`sk-proj-`) | Critical |
| Anthropic API Key (`sk-ant-`) | Critical |
| Private Key Block | Critical |
| JWT Token | Major |
| And more (Groq, xAI, Slack, Stripe, Google) | Varies |

Secrets are automatically **masked** in output (e.g. `AKIA****CDEF`). Test/spec/fixture files are auto-skipped.

## Static Security Scanner

cora runs a static security scan on added lines before the AI review. 11 built-in patterns detect common vulnerabilities:

| Pattern | Category | Severity |
|---------|----------|----------|
| MD5 used for password hashing | Weak crypto | Major |
| SHA-1 used for password hashing | Weak crypto | Major |
| Weak hash algorithm (MD5/SHA1) | Weak crypto | Minor |
| Hardcoded password or secret | Hardcoded secret | Critical |
| SQL injection via string concatenation | Injection | Critical |
| eval() with dynamic input | Injection | Critical |
| Command injection via exec/system | Injection | Critical |
| Hardcoded role or permission check | Auth | Major |
| Debug mode enabled | Config | Major |
| CORS wildcard allows all origins | Config | Major |
| SSL certificate verification disabled | Crypto | Critical |

Test files are automatically skipped. Findings are injected into the LLM prompt as additional context.

## Language-Specific Analyzers

cora detects the languages in your diff and injects tailored review guidance:

| Language | Guidance |
|----------|----------|
| **Dart / Flutter** | Widget lifecycle, state management, async patterns, null safety |
| **Svelte / TypeScript** | Reactivity, store patterns, SSR considerations, type safety |
| **Go** | Error handling, concurrency, goroutine leaks, interface design |
| **Rust** | Ownership, lifetimes, error handling, unsafe usage, idioms |
| **Python** | Type hints, async patterns, security (pickle/eval), common pitfalls |

No configuration needed — language context is auto-detected from file extensions in the diff.

## Quality Profiles

cora includes built-in quality profiles for different review strictness:

| Profile | Description |
|---------|------------|
| `strict` | All categories enabled, low tolerance for issues |
| `balanced` | *(default)* Security + bugs + performance, moderate thresholds |
| `lax` | Only critical issues, high tolerance |

Set in `.cora.yaml`:

```yaml
profile: strict
```

## Custom Rule Engine

Write your own regex-based rules in `.cora.yaml`:

```yaml
rules:
  - id: no-unwrap
    pattern: "\\.unwrap\\(\\)"
    severity: warning
    message: "Avoid unwrap() in production code — use proper error handling"
    languages: ["rust"]
    exclude: ["tests/**"]

  - id: no-console-log
    pattern: "console\\.log\\("
    severity: minor
    message: "Remove console.log before merging"
    languages: ["typescript", "javascript"]
```

Rules run during the deterministic pre-scan phase (no LLM needed).

## MCP Server

cora includes a built-in MCP (Model Context Protocol) server that exposes rules and config to AI coding agents like Claude Code, Cursor, Copilot, and Windsurf.

### Start the server

```bash
cora mcp
```

### Available tools

| Tool | Description |
|------|-------------|
| `cora.list_rules` | List all rules, security patterns, and secret patterns |
| `cora.check_snippet` | Check a code snippet against deterministic scanners (no LLM) |
| `cora.get_quality_gate` | Get quality gate config and thresholds |
| `cora.get_config` | Get effective project config (no secrets exposed) |
| `cora.list_profiles` | List all quality profiles |

### Configure in Claude Code

Add to your project's `.claude/settings.json`:

```json
{
  "mcpServers": {
    "cora": {
      "command": "cora",
      "args": ["mcp"]
    }
  }
}
```

### Configure in Cursor

Add to `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "cora": {
      "command": "cora",
      "args": ["mcp"]
    }
  }
}
```

The MCP server communicates via JSON-RPC 2.0 over stdio — no HTTP server needed.
