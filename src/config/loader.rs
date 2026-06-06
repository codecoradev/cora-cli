use std::path::{Path, PathBuf};

use crate::error::CoraError;
use tracing::debug;

use crate::config::providers::{PRESETS, detected_presets};
use crate::config::schema::{Config, CoraFile};
use crate::engine::LLMConfig;

/// The name of the config file we search for in projects.
const CONFIG_FILENAME: &str = ".cora.yaml";

/// Name of the global config file.
const GLOBAL_CONFIG_FILENAME: &str = "config.yaml";

/// Name of the secret config file for API keys (never committed).
const AUTH_FILENAME: &str = "auth.toml";

/// Name of the old auth file (for migration).
const OLD_AUTH_FILENAME: &str = "config.toml";

/// Marker file created after successful migration.
const MIGRATION_MARKER: &str = ".migrated";

/// Locate the `.cora.yaml` config by walking parent directories from `start`.
/// Returns the path and parsed content, or `None` if not found.
pub fn find_cora_file(start: &Path) -> std::result::Result<Option<(PathBuf, CoraFile)>, CoraError> {
    let mut dir = if start.is_file() {
        start.parent().unwrap_or(start).to_path_buf()
    } else {
        start.to_path_buf()
    };

    loop {
        let candidate = dir.join(CONFIG_FILENAME);
        if candidate.is_file() {
            debug!(path = %candidate.display(), "found .cora.yaml");
            let content = std::fs::read_to_string(&candidate)
                .map_err(|e| CoraError::ConfigRead(format!("{}: {}", candidate.display(), e)))?;
            let cora = CoraFile::from_str(&content).map_err(|e| {
                CoraError::ConfigParse(format!(
                    "{}\n  → file: {}\n  → hint: check for syntax errors (indentation, colons, trailing spaces)",
                    e,
                    candidate.display()
                ))
            })?;
            return Ok(Some((candidate, cora)));
        }

        match dir.parent() {
            Some(parent) if parent != dir => dir = parent.to_path_buf(),
            _ => return Ok(None),
        }
    }
}

/// Load the global config from `~/.cora/config.yaml`.
/// Returns `None` if the file doesn't exist or can't be parsed.
fn load_global_config() -> std::result::Result<Option<CoraFile>, CoraError> {
    let dir = cora_dir()?;
    let path = dir.join(GLOBAL_CONFIG_FILENAME);
    if !path.is_file() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(&path)
        .map_err(|e| CoraError::ConfigRead(format!("{}: {}", path.display(), e)))?;
    let cora = CoraFile::from_str(&content)?;
    debug!(path = %path.display(), "loaded global config");
    Ok(Some(cora))
}

/// Migrate old `~/.cora/config.toml` to the new format if it exists.
/// - Non-secret keys → `~/.cora/config.yaml`
/// - `api_key` → `~/.cora/auth.toml`
/// - Delete the old file after successful migration.
/// - Creates `.migrated` marker to prevent re-running.
#[allow(
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn migrate_old_config() {
    let Ok(dir) = cora_dir() else {
        return;
    };
    let old_path = dir.join(OLD_AUTH_FILENAME);
    if !old_path.is_file() {
        return;
    }

    // Check if migration already completed
    if dir.join(MIGRATION_MARKER).is_file() {
        return;
    }

    let content = match std::fs::read_to_string(&old_path) {
        Ok(c) => c,
        Err(e) => {
            debug!("failed to read old config for migration: {}", e);
            return;
        }
    };

    let table: toml::Table = match content.parse::<toml::Table>() {
        Ok(t) => t,
        Err(e) => {
            debug!(
                "old config.toml is not valid TOML, skipping migration: {}",
                e
            );
            return;
        }
    };

    // Check if there's an api_key
    let api_key = table
        .get("auth")
        .and_then(|a| a.get("api_key"))
        .and_then(|k| k.as_str())
        .map(std::string::ToString::to_string)
        .or_else(|| {
            table
                .get("api_key")
                .and_then(|k| k.as_str())
                .map(std::string::ToString::to_string)
        });

    // Extract non-secret config fields into a CoraFile
    let mut cora = CoraFile::default();

    if let Some(provider) = table.get("provider").and_then(|v| v.as_table()) {
        let mut ps = crate::config::schema::ProviderSection::default();
        if let Some(v) = provider.get("provider").and_then(|v| v.as_str()) {
            ps.provider = Some(v.to_string());
        }
        if let Some(v) = provider.get("model").and_then(|v| v.as_str()) {
            ps.model = Some(v.to_string());
        }
        if let Some(v) = provider.get("base_url").and_then(|v| v.as_str()) {
            ps.base_url = Some(v.to_string());
        }
        // Only set if we found something
        if ps.provider.is_some() || ps.model.is_some() || ps.base_url.is_some() {
            cora.provider = Some(ps);
        }
    }

    if let Some(output) = table.get("output").and_then(|v| v.as_table()) {
        let mut os = crate::config::schema::OutputSection::default();
        if let Some(v) = output.get("format").and_then(|v| v.as_str()) {
            os.format = Some(v.to_string());
        }
        if let Some(v) = output.get("color").and_then(toml::Value::as_bool) {
            os.color = Some(v);
        }
        if os.format.is_some() || os.color.is_some() {
            cora.output = Some(os);
        }
    }

    if let Some(hook) = table.get("hook").and_then(|v| v.as_table()) {
        let mut hs = crate::config::schema::HookSection::default();
        if let Some(v) = hook.get("mode").and_then(|v| v.as_str()) {
            hs.mode = Some(v.to_string());
        }
        if let Some(v) = hook.get("min_severity").and_then(|v| v.as_str()) {
            hs.min_severity = Some(v.to_string());
        }
        if let Some(v) = hook.get("max_diff_size").and_then(toml::Value::as_integer) {
            hs.max_diff_size = Some(v as usize);
        }
        if hs.mode.is_some() || hs.min_severity.is_some() || hs.max_diff_size.is_some() {
            cora.hook = Some(hs);
        }
    }

    // Write api_key to auth.toml if present (use TOML library to avoid injection)
    if let Some(key) = api_key {
        let auth_path = dir.join(AUTH_FILENAME);
        let mut table = toml::Table::new();
        table.insert("api_key".to_string(), toml::Value::String(key));
        let content = table.to_string();
        if let Err(e) = std::fs::write(&auth_path, content) {
            debug!("failed to write migrated auth.toml: {}", e);
            return;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o600);
            let _ = std::fs::set_permissions(&auth_path, perms);
        }
    }

    // Write config to config.yaml if there are non-secret fields
    let has_config = cora.provider.is_some()
        || cora.output.is_some()
        || cora.hook.is_some()
        || cora.focus.is_some()
        || cora.rules.is_some()
        || cora.ignore.is_some();

    if has_config {
        let config_path = dir.join(GLOBAL_CONFIG_FILENAME);
        if let Ok(yaml) = serde_yaml_ng::to_string(&cora) {
            if let Err(e) = std::fs::write(&config_path, yaml) {
                debug!("failed to write migrated config.yaml: {}", e);
                return;
            }
        }
    }

    // Delete old file and create migration marker
    if let Err(e) = std::fs::remove_file(&old_path) {
        debug!("failed to remove old config.toml after migration: {}", e);
    } else {
        // Create marker to prevent re-running migration
        let _ = std::fs::write(dir.join(MIGRATION_MARKER), "");
        debug!("migrated ~/.cora/config.toml to new format");
    }
}

/// Load the full resolved config: defaults ← global config ← .cora.yaml ← CLI overrides.
///
/// `cli_provider`, `cli_model`, `cli_api_key`, and `cli_format` are `None`
/// when the user did not pass the corresponding flag.
pub fn load_config(
    cli_config_path: Option<&str>,
    cli_provider: Option<&str>,
    cli_model: Option<&str>,
    cli_base_url: Option<&str>,
    cli_format: Option<&str>,
    no_color: bool,
) -> std::result::Result<Config, CoraError> {
    let mut config = Config::default();

    // Run migration silently on first access
    migrate_old_config();

    // 1. Load global config (~/.cora/config.yaml)
    if let Some(cora) = load_global_config()? {
        cora.merge_into(&mut config);
    }

    // 2. Load project config (.cora.yaml)
    if let Some(path) = cli_config_path {
        let path = Path::new(path);
        let content = std::fs::read_to_string(path)
            .map_err(|e| CoraError::ConfigRead(format!("{}: {}", path.display(), e)))?;
        let cora = CoraFile::from_str(&content).map_err(|e| {
            CoraError::ConfigParse(format!(
                "{}\n  → file: {}\n  → hint: check for syntax errors (indentation, colons, trailing spaces)",
                e,
                path.display()
            ))
        })?;
        cora.merge_into(&mut config);
        debug!(path = %path.display(), "loaded explicit config");
    } else if let Some((path, cora)) = find_cora_file(&std::env::current_dir()?)? {
        cora.merge_into(&mut config);
        debug!(path = %path.display(), "loaded discovered config");
    } else {
        debug!("no .cora.yaml found, using defaults");
    }

    // 3. CLI overrides
    if let Some(p) = cli_provider {
        config.provider.provider = p.to_string();
    }
    if let Some(m) = cli_model {
        config.provider.model = m.to_string();
    }
    if let Some(u) = cli_base_url {
        config.provider.base_url = u.to_string();
    }
    if let Some(f) = cli_format {
        config.output.format = f.to_string();
    }
    if no_color {
        config.output.color = false;
    }

    Ok(config)
}

/// Build an `LLMConfig` from the resolved `Config`, fetching the API key
/// from: CLI flag → env `CORA_API_KEY` → ~/.cora/auth.toml.
///
/// If none of those are set, auto-detect from known provider env vars (`OPENAI_API_KEY`, etc.)
/// and configure `provider/model/base_url` from the matching preset.
pub fn build_llm_config(
    config: &Config,
    cli_api_key: Option<&str>,
) -> std::result::Result<LLMConfig, CoraError> {
    // Resolve the API key and optional auto-detected preset in one pass.
    let (api_key, auto_preset) = if let Some(key) = cli_api_key {
        (key.to_string(), None)
    } else if let Ok(key) = std::env::var("CORA_API_KEY") {
        (key, None)
    } else if let Some(key) = load_api_key_from_auth_file()? {
        (key, None)
    } else {
        // No CORA_API_KEY or stored key — auto-detect from provider presets
        let detected = detected_presets();
        if detected.is_empty() {
            let _available: Vec<String> = PRESETS
                .iter()
                .map(|p| format!("  {} (set {})", p.name, p.env_key))
                .collect();
            return Err(CoraError::NoApiKey);
        }

        // Use the first detected provider
        let preset = detected[0];
        let key = std::env::var(preset.env_key).unwrap_or_default();

        if detected.len() > 1 {
            let names: Vec<&str> = detected.iter().map(|p| p.name).collect();
            eprintln!(
                "ℹ️  Multiple providers detected ({}). Using first: {}. Set CORA_PROVIDER or use --provider to override.",
                names.join(", "),
                preset.name
            );
        } else {
            debug!(provider = preset.name, "auto-detected provider from env");
        }

        (key, Some(preset))
    };

    // Resolve provider/model/base_url: CORA_* env > auto-detected preset > config defaults
    let cora_provider = std::env::var("CORA_PROVIDER").ok();
    let cora_model = std::env::var("CORA_MODEL").ok();
    let cora_base_url = std::env::var("CORA_BASE_URL").ok();

    // Warn when env vars override config file settings
    if let Some(ref env_p) = cora_provider {
        if env_p != &config.provider.provider {
            eprintln!(
                "⚠️  CORA_PROVIDER={env_p} overrides config provider={}",
                config.provider.provider
            );
        }
    }
    if let Some(ref env_m) = cora_model {
        if env_m != &config.provider.model {
            eprintln!(
                "⚠️  CORA_MODEL={env_m} overrides config model={}",
                config.provider.model
            );
        }
    }
    if let Some(ref env_u) = cora_base_url {
        if env_u != &config.provider.base_url {
            eprintln!(
                "⚠️  CORA_BASE_URL overrides config base_url={}",
                config.provider.base_url
            );
        }
    }

    let provider = cora_provider
        .or_else(|| auto_preset.map(|p| p.name.to_string()))
        .unwrap_or_else(|| config.provider.provider.clone());

    let model = cora_model
        .or_else(|| auto_preset.map(|p| p.default_model.to_string()))
        .unwrap_or_else(|| config.provider.model.clone());

    let base_url = cora_base_url
        .or_else(|| {
            // Check if the auto-detected preset has a custom URL override
            auto_preset.and_then(|p| std::env::var(p.env_url).ok())
        })
        .or_else(|| auto_preset.map(|p| p.default_base_url.to_string()))
        .unwrap_or_else(|| config.provider.base_url.clone());

    Ok(LLMConfig {
        api_key,
        base_url,
        model,
        provider,
        temperature: config.temperature,
        max_tokens: config.max_tokens,
        timeout: config.timeout,
    })
}

/// Get the cora config directory: ~/.cora/
pub fn cora_dir() -> std::result::Result<PathBuf, CoraError> {
    let home = dirs::home_dir()
        .ok_or_else(|| CoraError::ConfigRead("cannot determine home directory".into()))?;
    Ok(home.join(".cora"))
}

/// Read the stored API key from ~/.cora/auth.toml.
pub fn load_api_key_from_auth_file() -> std::result::Result<Option<String>, CoraError> {
    let dir = cora_dir()?;
    let path = dir.join(AUTH_FILENAME);
    if !path.is_file() {
        return Ok(None);
    }

    // Security: warn if auth file has overly permissive permissions (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = std::fs::metadata(&path) {
            let mode = meta.permissions().mode();
            if mode & 0o077 != 0 {
                // Auto-fix: restrict permissions to owner-only
                let fixed = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
                if fixed.is_ok() {
                    debug!("auto-fixed auth file permissions: {:o} → 600", mode & 0o777);
                } else {
                    tracing::warn!(
                        "auth file has overly permissive permissions ({:o}). Run: chmod 600 {}",
                        mode & 0o777,
                        path.display()
                    );
                }
            }
        }
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| CoraError::ConfigRead(format!("{}: {}", path.display(), e)))?;

    // Simple TOML: expect `[auth]\napi_key = "..."`  or just `api_key = "..."`
    let value: toml::Table = content
        .parse::<toml::Table>()
        .map_err(|e| CoraError::AuthError(format!("invalid TOML: {}", e)))?;

    let key = value
        .get("auth")
        .and_then(|a| a.get("api_key"))
        .and_then(|k| k.as_str())
        .map(std::string::ToString::to_string)
        .or_else(|| {
            value
                .get("api_key")
                .and_then(|k| k.as_str())
                .map(std::string::ToString::to_string)
        });

    Ok(key)
}

/// Save an API key to ~/.cora/auth.toml.
pub fn save_api_key(key: &str) -> std::result::Result<(), CoraError> {
    let dir = cora_dir()?;
    std::fs::create_dir_all(&dir).map_err(|e| CoraError::AuthError(e.to_string()))?;

    let path = dir.join(AUTH_FILENAME);
    let mut table = toml::Table::new();
    table.insert("api_key".to_string(), toml::Value::String(key.to_string()));
    let content = table.to_string();

    std::fs::write(&path, content)
        .map_err(|e| CoraError::AuthError(format!("{}: {}", path.display(), e)))?;

    debug!(path = %path.display(), "saved API key");

    // Restrict permissions to owner only (0o600)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(&path, perms)?;
    }

    Ok(())
}

/// Remove the stored API key from ~/.cora/auth.toml.
pub fn remove_api_key() -> std::result::Result<(), CoraError> {
    let dir = cora_dir()?;
    let path = dir.join(AUTH_FILENAME);
    if path.is_file() {
        std::fs::remove_file(&path)
            .map_err(|e| CoraError::AuthError(format!("{}: {}", path.display(), e)))?;
        debug!("removed API key file");
    }
    Ok(())
}

/// Check the auth status: whether an API key is available.
pub fn auth_status() -> std::result::Result<AuthStatus, CoraError> {
    if std::env::var("CORA_API_KEY").is_ok() {
        Ok(AuthStatus {
            source: "env var CORA_API_KEY".to_string(),
            has_key: true,
        })
    } else if load_api_key_from_auth_file()?.is_some() {
        let dir = cora_dir()?;
        Ok(AuthStatus {
            source: format!("{}", dir.join(AUTH_FILENAME).display()),
            has_key: true,
        })
    } else {
        Ok(AuthStatus {
            source: String::new(),
            has_key: false,
        })
    }
}

/// Auth status information.
pub struct AuthStatus {
    pub source: String,
    pub has_key: bool,
}

/// Stored provider information alongside the API key.
#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub provider: String,
    pub base_url: String,
    pub model: String,
}

/// Save provider info (name, base_url, model) to `~/.cora/auth.toml`
/// alongside the existing `api_key`.
pub fn save_provider_info(
    provider: &str,
    base_url: &str,
    model: &str,
) -> std::result::Result<(), CoraError> {
    let dir = cora_dir()?;
    let path = dir.join(AUTH_FILENAME);

    // Read existing auth.toml or start fresh
    let mut table = if path.is_file() {
        let content = std::fs::read_to_string(&path)
            .map_err(|e| CoraError::AuthError(format!("{}: {}", path.display(), e)))?;
        content
            .parse::<toml::Table>()
            .unwrap_or_else(|_| toml::Table::new())
    } else {
        toml::Table::new()
    };

    // Set provider info fields
    table.insert(
        "provider".to_string(),
        toml::Value::String(provider.to_string()),
    );
    table.insert(
        "base_url".to_string(),
        toml::Value::String(base_url.to_string()),
    );
    table.insert("model".to_string(), toml::Value::String(model.to_string()));

    let content = table.to_string();
    std::fs::write(&path, content)
        .map_err(|e| CoraError::AuthError(format!("{}: {}", path.display(), e)))?;

    // Restrict permissions to owner only (0o600)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(&path, perms)?;
    }

    debug!(provider = provider, "saved provider info");
    Ok(())
}

/// Load stored provider info from `~/.cora/auth.toml`.
/// Returns `None` if no provider info is stored.
pub fn load_provider_info() -> std::result::Result<Option<ProviderInfo>, CoraError> {
    let dir = cora_dir()?;
    let path = dir.join(AUTH_FILENAME);
    if !path.is_file() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| CoraError::AuthError(format!("{}: {}", path.display(), e)))?;

    let table: toml::Table = content
        .parse::<toml::Table>()
        .map_err(|e| CoraError::AuthError(format!("invalid TOML: {}", e)))?;

    let provider = table
        .get("provider")
        .and_then(toml::Value::as_str)
        .unwrap_or("");
    let base_url = table
        .get("base_url")
        .and_then(toml::Value::as_str)
        .unwrap_or("");
    let model = table
        .get("model")
        .and_then(toml::Value::as_str)
        .unwrap_or("");

    if provider.is_empty() {
        return Ok(None);
    }

    Ok(Some(ProviderInfo {
        provider: provider.to_string(),
        base_url: base_url.to_string(),
        model: model.to_string(),
    }))
}

/// Remove stored provider info from `~/.cora/auth.toml` while keeping the api_key if present.
pub fn remove_provider_info() -> std::result::Result<(), CoraError> {
    let dir = cora_dir()?;
    let path = dir.join(AUTH_FILENAME);
    if !path.is_file() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| CoraError::AuthError(format!("{}: {}", path.display(), e)))?;

    let mut table: toml::Table = content
        .parse::<toml::Table>()
        .map_err(|e| CoraError::AuthError(format!("invalid TOML: {}", e)))?;

    let changed = table.remove("provider").is_some()
        | table.remove("base_url").is_some()
        | table.remove("model").is_some();

    if changed {
        // If only provider info was left (no api_key), just delete the file
        if table.is_empty() {
            std::fs::remove_file(&path)
                .map_err(|e| CoraError::AuthError(format!("{}: {}", path.display(), e)))?;
        } else {
            let content = table.to_string();
            std::fs::write(&path, content)
                .map_err(|e| CoraError::AuthError(format!("{}: {}", path.display(), e)))?;
        }
        debug!("removed provider info from auth.toml");
    }

    Ok(())
}
