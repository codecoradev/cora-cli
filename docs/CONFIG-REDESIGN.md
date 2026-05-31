# Config System Redesign: Global + Project Config

## Status: Draft — Pending Approval

## Problem

Current `cora config set` writes to `~/.cora/config.toml` but the config loader only reads `.cora.yaml`. Values set via `config set` are silently ignored. Additionally, there's no way to set global defaults across all projects.

## Goal

Support both global and per-project configuration with clear priority:

```
CLI flags → CORA_* env vars → .cora.yaml (project) → ~/.cora/config.yaml (global) → Defaults
```

## Design

### Config Sources

| Source | Path | Format | Scope | Priority |
|--------|------|--------|-------|----------|
| **Global** | `~/.cora/config.yaml` | YAML | All projects | 4 (lowest, after env) |
| **Project** | `.cora.yaml` (walk parent dirs) | YAML | Current project | 3 |
| **Env vars** | `CORA_PROVIDER`, `CORA_MODEL`, `CORA_BASE_URL`, `CORA_API_KEY`, etc. | — | Current shell | 2 |
| **CLI flags** | `--provider`, `--model`, `--base-url`, etc. | — | Current command | 1 (highest) |

### Merge Strategy

Same as current `.cora.yaml` merge — global config is loaded first (into defaults), then project config overwrites any fields present. Each field is independently resolved.

Example:

**Global** (`~/.cora/config.yaml`):
```yaml
provider:
  provider: openai
  model: gpt-4o-mini
output:
  format: compact
```

**Project** (`.cora.yaml`):
```yaml
provider:
  model: glm-5.1
  base_url: http://litellm:4000
hook:
  mode: block
```

**Resolved:**
```yaml
provider:
  provider: openai          # from global
  model: glm-5.1           # project overrides
  base_url: http://litellm:4000  # project
output:
  format: compact           # from global
hook:
  mode: block               # from project
  min_severity: major       # default
```

### `cora config set` Redesign

```bash
# Default: write to project .cora.yaml (must be inside a git project)
cora config set model glm-5.1
cora config set base_url http://litellm:4000

# Explicit global
cora config set --global model gpt-4o-mini
cora config set --global provider openai

# All supported keys:
# model, provider, base_url, format, severity (maps to hook.min_severity)
```

Behavior:
- **No `--global`**: writes to `.cora.yaml` in current dir (creates if missing)
- **`--global`**: writes to `~/.cora/config.yaml` (creates if missing)
- **`--global --show`**: prints global config path + contents
- Works with the same TOML→YAML key mapping as before

### Auth File (Separate Concern)

API key storage stays separate at `~/.cora/auth.toml` (rename from `config.toml`):

- `cora auth login` → `~/.cora/auth.toml` (contains only `api_key`)
- `cora auth status` → reads from `~/.cora/auth.toml`
- `cora auth remove` → deletes `~/.cora/auth.toml`

This separation means:
- Config = `~/.cora/config.yaml` (global settings, no secrets)
- Auth = `~/.cora/auth.toml` (API key only, permissions 0o600)

### `cora config show` Enhancement

```bash
$ cora config show

╔══════════════════════════════════════════╗
║          Current Configuration            ║
╚══════════════════════════════════════════╝

  Config sources (priority ↓):
    CLI flags        —
    Env vars         —
    Project config   .cora.yaml ✅
    Global config    ~/.cora/config.yaml ✅

provider:
  provider: openai           (global)
  model: glm-5.1            (project)
  base_url: http://litellm:4000  (project)

focus:
  security, performance, bugs  (default)

hook:
  mode: block             (project)
  min_severity: major     (default)
  max_diff_size: 51200    (default)

output:
  format: compact          (global)
  color: true             (default)
```

Each value shows its source — makes debugging config priority transparent.

## Files to Change

| File | Change |
|------|--------|
| `src/config/loader.rs` | Add `load_global_config()` → reads `~/.cora/config.yaml`. Update `load_config()` to merge: global → project → CLI. Rename `AUTH_FILENAME` to `auth.toml`. |
| `src/commands/config_cmd.rs` | Rewrite `execute_config_set()`: write YAML instead of TOML. Add `--global` flag support. Add `base_url` key. Keep `cora config show` but add source annotations. |
| `src/main.rs` | Add `--global` flag to `config set` subcommand. Add `config set base_url <value>`. |
| `src/config/schema.rs` | No changes needed — `CoraFile` schema already supports all fields. Global uses same `CoraFile` struct. |

## Migration

- **No breaking change** — existing `.cora.yaml` files work as-is
- `~/.cora/config.toml` (if exists) → auto-migrate to `~/.cora/config.yaml` on first run (read TOML, write YAML, delete TOML). For auth keys: rename `config.toml` → `auth.toml`
- `cora auth login` switches from writing `config.toml` to `auth.toml`

## Effort Estimate

| Item | LOC (est.) | Effort |
|------|-----------|--------|
| Global config loader | ~40 | 30 min |
| `config set` rewrite (YAML + `--global` + `base_url`) | ~80 | 1h |
| `config show` source annotations | ~40 | 30 min |
| Auth file rename (`config.toml` → `auth.toml`) | ~20 | 15 min |
| Migration logic (TOML → YAML) | ~30 | 30 min |
| Tests | ~60 | 45 min |
| **Total** | ~270 | **~3.5h** |

## Risks

| Risk | Mitigation |
|------|-----------|
| YAML serialization changes formatting | Use `serde_yaml_ng` with consistent output |
| Migration edge case (partial TOML) | Log warning, skip gracefully, user can manually edit |
| Two config sources confuse users | `config show` annotates source for each value |
