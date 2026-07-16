---
title: Providers
---

# Providers

cora supports multiple AI providers. Use your own API key — no subscriptions to us.

## Supported Providers

| Provider | Default Model | Env Var | Custom Base URL |
|----------|---------------|---------|-----------------|
| OpenAI | `gpt-4o-mini` | `OPENAI_API_KEY` | `OPENAI_BASE_URL` |
| Anthropic | `claude-sonnet-4-20250514` | `ANTHROPIC_API_KEY` | `ANTHROPIC_BASE_URL` |
| Groq | `llama-3.1-8b-instant` | `GROQ_API_KEY` | `GROQ_BASE_URL` |
| Ollama | `llama3.1` | — (local) | `OLLAMA_HOST` (default: `http://localhost:11434`) |
| Z.AI | `glm-5.1` | `ZAI_API_KEY` | `ZAI_BASE_URL` |

## Auto-Detection

cora automatically detects which provider to use by checking environment variables in this order:

1. `OPENAI_API_KEY` → uses OpenAI
2. `ANTHROPIC_API_KEY` → uses Anthropic
3. `GROQ_API_KEY` → uses Groq
4. `ZAI_API_KEY` → uses Z.AI
5. `OLLAMA_HOST` → uses Ollama (localhost)

Override auto-detection with `CORA_PROVIDER` env var or `--provider` flag.

## Usage Examples

```bash
# Use OpenAI (auto-detected from OPENAI_API_KEY)
$ OPENAI_API_KEY=sk-... cora review --staged
```

```bash
# Use Anthropic with explicit provider
$ CORA_PROVIDER=anthropic CORA_API_KEY=sk-ant-... cora review --staged
```

```bash
# Use Ollama locally (no API key needed)
$ CORA_PROVIDER=ollama cora review --staged
```

```bash
# Use a custom model
$ CORA_MODEL=gpt-4o-mini cora review --staged
```
