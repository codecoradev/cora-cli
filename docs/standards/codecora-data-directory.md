# CodeCora Data Directory Standard

All CodeCora products store their runtime data under `$HOME/.codecora/{product}/`.

## Layout

```
~/.codecora/
├── cora-code/          # cora-code runtime data
│   ├── graph.db        # knowledge graph (Phase 2)
│   └── index/          # embedding cache, index files
├── nginjen/            # nginjen runtime data
│   ├── config.toml     # runtime config
│   └── engines/        # cached browser engines
├── trapfall/           # trapfall runtime data
│   └── trapfall.db     # error capture database
└── uteke/              # uteke runtime data (future migration from ~/.uteke/)
    └── uteke.db        # semantic memory database
```

## Rules

1. **Path**: `$HOME/.codecora/{product}/` — always resolve via `dirs::home_dir()`
2. **Product name**: lowercase, kebab-case (`cora-code`, `nginjen`, `trapfall`, `uteke`)
3. **Auto-create**: `create_dir_all()` on first access
4. **SQLite files**: `{product}.db` inside the product directory
5. **Cross-product read**: any CodeCora product MAY read another product's data
   (e.g., cora-code reading uteke memories, trapfall reading cora-code graph)
6. **No config here**: config files go elsewhere (env vars, CLI flags, project-local `.cora/`)
7. **Env override**: `{PRODUCT}_DATA_DIR` overrides the default path

## Reference Implementation (Rust)

```rust
use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("CORA_CODE_DATA_DIR") {
        return PathBuf::from(dir);
    }
    let home = dirs::home_dir()
        .expect("home directory not found");
    home.join(".codecora").join("cora-code")
}
```

## Adoption Status

| Product | Status | Notes |
|---------|--------|-------|
| nginjen | ✅ Adopted | `engine.rs` uses `~/.codecora/nginjen/engines`, `config.rs` uses XDG dirs (inconsistency — align to this standard) |
| trapfall | ✅ Adopted | `config.rs` uses `$HOME/.codecora/trapfall/` |
| cora-code | 🔜 Phase 2 | Will store `graph.db` here |
| uteke | ⏸️ Deferred | Currently `~/.uteke/` — established path, low priority migration |