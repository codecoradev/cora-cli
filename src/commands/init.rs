use anyhow::{Context, Result};
use colored::Colorize;
use tracing::debug;

use crate::hook;

/// Example .cora.yaml template content.
const CONFIG_TEMPLATE: &str = r#"# Cora Code Review Configuration
# See: https://github.com/codecoradev/cora-cli

# Provider settings
provider:
  provider: openai
  model: gpt-4o-mini
  base_url: https://api.openai.com/v1

# Review focus areas: security, performance, bugs, best-practice, style
focus:
  - security
  - performance
  - bugs

# Custom rules / additional instructions for the reviewer
rules: []
# rules:
#   - "Always check for SQL injection vulnerabilities"
#   - "Ensure all public functions have error handling"

# Ignore patterns
ignore:
  files:
    - "*.lock"
    - "package-lock.json"
    - "yarn.lock"
    - "vendor/"
    - "node_modules/"
  rules: []
#   rules:
#     - "style"  # ignore style issues

# Hook settings
hook:
  mode: warn          # "warn" (print but allow) or "block" (fail commit)
  min_severity: major # minimum severity to trigger hook action
  max_diff_size: 5242880 # max 5MB diff chars before refusing

# Output settings
output:
  format: pretty      # pretty, json, compact, sarif
  color: true
"#;

/// Execute the init command: create a `.cora.yaml` file and install pre-commit hook.
pub fn execute_init(skip_hook: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let config_path = cwd.join(".cora.yaml");

    if config_path.is_file() {
        anyhow::bail!(
            ".cora.yaml already exists at {}. Use --force to overwrite.",
            config_path.display()
        );
    }

    std::fs::write(&config_path, CONFIG_TEMPLATE)
        .with_context(|| format!("failed to write {}", config_path.display()))?;

    debug!(path = %config_path.display(), "created .cora.yaml");
    println!(
        "{} Created {} with example configuration.",
        "✅".green().bold(),
        config_path.display()
    );
    println!(
        "{}",
        "   Edit the file to customize your review settings.".dimmed()
    );
    println!(
        "{}",
        "   API keys should be set via CORA_API_KEY env var or `cora auth login`.".dimmed()
    );

    // Install pre-commit hook unless --no-hook is specified
    if !skip_hook {
        install_hook_silent();
    } else {
        println!(
            "{}",
            "   Skipped hook installation (--no-hook). Run `cora hook install` later.".dimmed()
        );
    }

    Ok(())
}

/// Execute the init command with --force flag.
pub fn execute_init_force(skip_hook: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let config_path = cwd.join(".cora.yaml");

    std::fs::write(&config_path, CONFIG_TEMPLATE)
        .with_context(|| format!("failed to write {}", config_path.display()))?;

    debug!(path = %config_path.display(), "overwrote .cora.yaml");
    println!(
        "{} Overwrote {} with example configuration.",
        "✅".green().bold(),
        config_path.display()
    );

    // Install pre-commit hook unless --no-hook is specified
    if !skip_hook {
        install_hook_silent();
    } else {
        println!(
            "{}",
            "   Skipped hook installation (--no-hook). Run `cora hook install` later.".dimmed()
        );
    }

    Ok(())
}

/// Global config template - placed in ~/.cora/config.yaml
const GLOBAL_CONFIG_TEMPLATE: &str = r#"# Cora Global Configuration
# This file lives in ~/.cora/config.yaml and contains user-wide defaults.
# Per-project .cora.yaml files override these settings.

# Default provider - change this to your preferred LLM provider
provider:
  provider: openai
  model: gpt-4o-mini
  base_url: https://api.openai.com/v1
  # Uncomment to use a different provider:
  # provider: anthropic
  # model: claude-3-5-sonnet-latest
  # base_url: https://api.anthropic.com/v1

# Default review focus areas
focus:
  - security
  - performance
  - bugs
"#';

/// Execute init with --global flag: create `~/.cora/config.yaml` with user-wide defaults.
pub fn execute_init_global() -> Result<()> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .context("Could not determine home directory. Set $HOME or $USERPROFILE.")?;
    let cora_dir = std::path::PathBuf::from(&home).join(".cora");
    let config_path = cora_dir.join("config.yaml");

    if config_path.is_file() {
        anyhow::bail!(
            "Global config already exists at {}. Use --force to overwrite.",
            config_path.display()
        );
    }

    std::fs::create_dir_all(&cora_dir)
        .with_context(|| format!("failed to create {}", cora_dir.display()))?;

    // Restrict directory permissions to owner only (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o700);
        let _ = std::fs::set_permissions(&cora_dir, perms);
    }

    std::fs::write(&config_path, GLOBAL_CONFIG_TEMPLATE)
        .with_context(|| format!("failed to write {}", config_path.display()))?;

    println!(
        "{} Created global config at {}",
        "✅".green().bold(),
        config_path.display()
    );
    println!(
        "{}",
        "   Edit the file to set your preferred LLM provider and model.".dimmed()
    );
    println!(
        "{}",
        "   API keys should be set via CORA_API_KEY env var or `cora auth login`.".dimmed()
    );

    Ok(())
}

/// Execute init with --global --force flag: overwrite existing global config.
pub fn execute_init_global_force() -> Result<()> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .context("Could not determine home directory. Set $HOME or $USERPROFILE.")?;
    let cora_dir = std::path::PathBuf::from(&home).join(".cora");
    let config_path = cora_dir.join("config.yaml");

    std::fs::create_dir_all(&cora_dir)
        .with_context(|| format!("failed to create {}", cora_dir.display()))?;

    std::fs::write(&config_path, GLOBAL_CONFIG_TEMPLATE)
        .with_context(|| format!("failed to write {}", config_path.display()))?;

    println!(
        "{} Overwrote global config at {}",
        "✅".green().bold(),
        config_path.display()
    );

    Ok(())
}

/// Install hook silently — warns on failure but doesn't fail init.
fn install_hook_silent() {
    match hook::install_hook() {
        Ok(path) => {
            println!(
                "{} Installed pre-commit hook to {}",
                "✅".green().bold(),
                path
            );
            println!(
                "{}",
                "   The hook will run `cora review --staged --format compact` before each commit."
                    .dimmed()
            );
        }
        Err(e) => {
            // Don't fail init if hook install fails (e.g. not a git repo)
            println!(
                "{} Could not install pre-commit hook: {}",
                "⚠️".yellow().bold(),
                e
            );
            println!(
                "{}",
                "   Run `cora hook install` later from inside a git repository.".dimmed()
            );
        }
    }
}
