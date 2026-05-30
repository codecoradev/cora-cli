use std::path::PathBuf;

use anyhow::{Context, Result};
use colored::Colorize;

use crate::config::loader;

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

/// Execute `cora config set <key> <value>` — write a key-value pair to ~/.cora/config.toml.
///
/// Supported keys: model, provider, format, severity.
pub fn execute_config_set(key: &str, value: &str) -> Result<()> {
    // Validate the key
    match key {
        "model" | "provider" | "format" | "severity" => {}
        _ => {
            anyhow::bail!(
                "unsupported key: {key}\nSupported keys: model, provider, format, severity"
            );
        }
    }

    let dir = cora_config_dir()?;
    std::fs::create_dir_all(&dir).with_context(|| format!("failed to create {}", dir.display()))?;

    let path = dir.join("config.toml");

    // Load existing config.toml content if it exists
    let mut table = if path.is_file() {
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        content.parse::<toml::Table>().unwrap_or_default()
    } else {
        toml::Table::new()
    };

    // Map the key to the appropriate TOML structure
    match key {
        "model" => {
            let provider = table
                .entry("provider")
                .or_insert_with(|| toml::Value::Table(toml::Table::new()));
            if let toml::Value::Table(p) = provider {
                p.insert("model".to_string(), toml::Value::String(value.to_string()));
            }
        }
        "provider" => {
            let provider = table
                .entry("provider")
                .or_insert_with(|| toml::Value::Table(toml::Table::new()));
            if let toml::Value::Table(p) = provider {
                p.insert(
                    "provider".to_string(),
                    toml::Value::String(value.to_string()),
                );
            }
        }
        "format" => {
            let output = table
                .entry("output")
                .or_insert_with(|| toml::Value::Table(toml::Table::new()));
            if let toml::Value::Table(o) = output {
                o.insert("format".to_string(), toml::Value::String(value.to_string()));
            }
        }
        "severity" => {
            let hook = table
                .entry("hook")
                .or_insert_with(|| toml::Value::Table(toml::Table::new()));
            if let toml::Value::Table(h) = hook {
                h.insert(
                    "min_severity".to_string(),
                    toml::Value::String(value.to_string()),
                );
            }
        }
        _ => unreachable!(),
    }

    let content = table.to_string();
    std::fs::write(&path, content)
        .with_context(|| format!("failed to write {}", path.display()))?;

    println!(
        "{} Set {} = {} in {}",
        "✓".green().bold(),
        key.bold(),
        value.green(),
        path.display()
    );

    Ok(())
}

/// Get the cora config directory: ~/.cora/
fn cora_config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("cannot determine home directory")?;
    Ok(home.join(".cora"))
}
