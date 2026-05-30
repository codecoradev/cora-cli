use anyhow::{Context, Result};
use colored::Colorize;
use tracing::debug;

/// Example .cora.yaml template content.
const CONFIG_TEMPLATE: &str = r#"# Cora Code Review Configuration
# See: https://github.com/ajianaz/cora-cli

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
  max_diff_size: 50000 # max diff chars before refusing

# Output settings
output:
  format: pretty      # pretty, json, compact, sarif
  color: true
"#;

/// Execute the init command: create a `.cora.yaml` file in the current directory.
pub fn execute_init() -> Result<()> {
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

    Ok(())
}

/// Execute the init command with --force flag.
pub fn execute_init_force() -> Result<()> {
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

    Ok(())
}
