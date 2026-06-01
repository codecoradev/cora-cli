use std::path::PathBuf;

use anyhow::{Context, Result};
use colored::Colorize;

use crate::config::loader;
use crate::config::schema::{CoraFile, HookSection, OutputSection, ProviderSection};

/// Execute `cora config show` — print the current resolved configuration.
pub fn execute_config_show() -> Result<()> {
    let config = loader::load_config(None, None, None, None, None, None, false)?;

    println!(
        "{}",
        "╔══════════════════════════════════════════╗".cyan().bold()
    );
    println!(
        "{}",
        "║          Current Configuration            ║"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════╝".cyan().bold()
    );
    println!();

    println!(
        "{} {}",
        "provider:".bold(),
        config.provider.provider.green()
    );
    println!("{} {}", "  model:".dimmed(), config.provider.model.green());
    println!(
        "{} {}",
        "  base_url:".dimmed(),
        config.provider.base_url.green()
    );

    println!(
        "{} {}",
        "focus:".bold(),
        config
            .focus
            .iter()
            .map(|f| format!("{}", f.green()))
            .collect::<Vec<_>>()
            .join(", ")
    );

    if !config.rules.is_empty() {
        println!(
            "{} {}",
            "rules:".bold(),
            config
                .rules
                .iter()
                .map(|r| format!("{}", r.green()))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    println!(
        "{} mode={} min_severity={} max_diff_size={}",
        "hook:".bold(),
        config.hook.mode.yellow(),
        config.hook.min_severity.yellow(),
        config.hook.max_diff_size
    );

    println!(
        "{} format={} color={}",
        "output:".bold(),
        config.output.format.green(),
        config.output.color
    );

    println!(
        "{} {}",
        "ignore files:".bold(),
        if config.ignore.files.is_empty() {
            "(none)".to_string().dimmed().to_string()
        } else {
            config
                .ignore
                .files
                .iter()
                .map(|f| format!("{}", f.dimmed()))
                .collect::<Vec<_>>()
                .join(", ")
        }
    );

    Ok(())
}

/// Execute `cora config set [--global] <key> <value>` — write a key-value pair
/// to a YAML config file.
///
/// Supported keys: model, provider, base_url, format, severity
pub fn execute_config_set(key: &str, value: &str, global: bool) -> Result<()> {
    // Validate the key
    match key {
        "model" | "provider" | "base_url" | "format" | "severity" => {}
        _ => {
            anyhow::bail!(
                "unsupported key: {key}\nSupported keys: model, provider, base_url, format, severity"
            );
        }
    }

    // Determine target path
    let path = if global {
        let dir = loader::cora_dir()?;
        std::fs::create_dir_all(&dir)
            .with_context(|| format!("failed to create {}", dir.display()))?;
        dir.join("config.yaml")
    } else {
        // Warn if not inside a git project (could create orphan file)
        if std::process::Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok_and(|s| s.success())
        {
            PathBuf::from(".cora.yaml")
        } else {
            anyhow::bail!(
                "Not in a git project. Use --global to set config in ~/.cora/config.yaml, or cd into a git repo first."
            );
        }
    };

    // Load existing file or start fresh
    let mut cora = if path.is_file() {
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        CoraFile::from_str(&content).unwrap_or_default()
    } else {
        CoraFile::default()
    };

    // Apply the key update
    match key {
        "model" => {
            let provider = cora.provider.get_or_insert_with(ProviderSection::default);
            provider.model = Some(value.to_string());
        }
        "provider" => {
            let provider = cora.provider.get_or_insert_with(ProviderSection::default);
            provider.provider = Some(value.to_string());
        }
        "base_url" => {
            let provider = cora.provider.get_or_insert_with(ProviderSection::default);
            provider.base_url = Some(value.to_string());
        }
        "format" => {
            let output = cora.output.get_or_insert_with(OutputSection::default);
            output.format = Some(value.to_string());
        }
        "severity" => {
            let hook = cora.hook.get_or_insert_with(HookSection::default);
            hook.min_severity = Some(value.to_string());
        }
        _ => unreachable!(),
    }

    // Write back as YAML
    let yaml = serde_yaml_ng::to_string(&cora).context("failed to serialize config to YAML")?;
    std::fs::write(&path, &yaml).with_context(|| format!("failed to write {}", path.display()))?;

    // Restrict permissions for global config (defense-in-depth)
    #[cfg(unix)]
    if global {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
    }

    let scope = if global { "global" } else { "project" };
    println!(
        "{} Set {} = {} in {} ({})",
        "✓".green().bold(),
        key.bold(),
        value.green(),
        path.display(),
        scope.dimmed()
    );

    Ok(())
}
