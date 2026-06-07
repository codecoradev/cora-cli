---
title: Configuration
---

# Configuration

cora uses a layered config system. Later sources override earlier ones.

## Config Resolution Order

Settings are resolved in this order (highest priority first):

1. **CLI flags** — `--provider`, `--model`, `--base-url`, etc.
2. **Environment variables** — `CORA_API_KEY`, `CORA_PROVIDER`, `CORA_MODEL`, etc.
3. **.cora.yaml** — Project root config file
4. **~/.cora/config.yaml** — Global config (optional)
5. **Built-in defaults** — Sensible defaults for all settings

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
| `CORA_API_KEY` | API key for the active provider |
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
