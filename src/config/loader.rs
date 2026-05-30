use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tracing::debug;

use crate::config::schema::{Config, CoraFile};
use crate::engine::LLMConfig;

/// The name of the config file we search for.
const CONFIG_FILENAME: &str = ".cora.yaml";

/// Name of the secret config file for API keys (never committed).
const AUTH_FILENAME: &str = "config.toml";

/// Locate the `.cora.yaml` config by walking parent directories from `start`.
/// Returns the path and parsed content, or `None` if not found.
pub fn find_cora_file(start: &Path) -> Result<Option<(PathBuf, CoraFile)>> {
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
                .with_context(|| format!("failed to read {}", candidate.display()))?;
            let cora = CoraFile::from_str(&content)?;
            return Ok(Some((candidate, cora)));
        }

        match dir.parent() {
            Some(parent) if parent != dir => dir = parent.to_path_buf(),
            _ => return Ok(None),
        }
    }
}

/// Load the full resolved config: defaults ← .cora.yaml ← CLI overrides.
///
/// `cli_provider`, `cli_model`, `cli_api_key`, and `cli_format` are `None`
/// when the user did not pass the corresponding flag.
pub fn load_config(
    cli_config_path: Option<&str>,
    cli_provider: Option<&str>,
    cli_model: Option<&str>,
    cli_base_url: Option<&str>,
    _cli_api_key: Option<&str>,
    cli_format: Option<&str>,
    no_color: bool,
) -> Result<Config> {
    let mut config = Config::default();

    // 1. Load .cora.yaml
    if let Some(path) = cli_config_path {
        let path = Path::new(path);
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read config at {}", path.display()))?;
        let cora = CoraFile::from_str(&content)?;
        cora.merge_into(&mut config);
        debug!(path = %path.display(), "loaded explicit config");
    } else if let Some((path, cora)) = find_cora_file(&std::env::current_dir()?)? {
        cora.merge_into(&mut config);
        debug!(path = %path.display(), "loaded discovered config");
    } else {
        debug!("no .cora.yaml found, using defaults");
    }

    // 2. CLI overrides
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
/// from: CLI flag → env CORA_API_KEY → ~/.config/cora/config.toml.
pub fn build_llm_config(
    config: &Config,
    cli_api_key: Option<&str>,
) -> Result<LLMConfig> {
    let api_key = if let Some(key) = cli_api_key {
        key.to_string()
    } else if let Some(key) = std::env::var("CORA_API_KEY").ok() {
        key
    } else if let Some(key) = load_api_key_from_auth_file()? {
        key
    } else {
        anyhow::bail!(
            "no API key found. Set CORA_API_KEY env var, pass --api-key, or run `cora auth login`"
        );
    };

    Ok(LLMConfig {
        api_key,
        base_url: config.provider.base_url.clone(),
        model: config.provider.model.clone(),
        provider: config.provider.provider.clone(),
    })
}

/// Get the cora config directory: ~/.cora/
fn cora_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("cannot determine home directory")?;
    Ok(home.join(".cora"))
}

/// Read the stored API key from ~/.cora/config.toml.
pub fn load_api_key_from_auth_file() -> Result<Option<String>> {
    let dir = cora_dir()?;
    let path = dir.join(AUTH_FILENAME);
    if !path.is_file() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    // Simple TOML: expect `[auth]\napi_key = "..."`  or just `api_key = "..."`
    let value: toml::Table = content
        .parse::<toml::Table>()
        .context("auth config file is not valid TOML")?;

    let key = value
        .get("auth")
        .and_then(|a| a.get("api_key"))
        .and_then(|k| k.as_str())
        .map(|s| s.to_string())
        .or_else(|| value.get("api_key").and_then(|k| k.as_str()).map(|s| s.to_string()));

    Ok(key)
}

/// Save an API key to ~/.cora/config.toml.
pub fn save_api_key(key: &str) -> Result<()> {
    let dir = cora_dir()?;
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("failed to create {}", dir.display()))?;

    let path = dir.join(AUTH_FILENAME);
    let content = format!("api_key = \"{key}\"\n");

    std::fs::write(&path, content)
        .with_context(|| format!("failed to write {}", path.display()))?;

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

/// Remove the stored API key from ~/.cora/config.toml.
pub fn remove_api_key() -> Result<()> {
    let dir = cora_dir()?;
    let path = dir.join(AUTH_FILENAME);
    if path.is_file() {
        std::fs::remove_file(&path)
            .with_context(|| format!("failed to remove {}", path.display()))?;
        debug!("removed API key file");
    }
    Ok(())
}

/// Check the auth status: whether an API key is available.
pub fn auth_status() -> Result<AuthStatus> {
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
