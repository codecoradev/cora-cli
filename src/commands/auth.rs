use std::io::{self, Write};

use anyhow::Result;
use colored::Colorize;

use crate::config::loader;
use crate::config::providers::PRESETS;

/// Execute `auth login` — interactive tiered provider selection and API key setup.
pub fn execute_auth_login() -> Result<()> {
    // Check if already logged in
    let status = loader::auth_status()?;
    if status.has_key {
        println!(
            "{}",
            "⚠️  An API key is already configured.".yellow().bold()
        );
        println!("   Source: {}", status.source);
        print!("   Overwrite? [y/N] ");
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        if !response.trim().eq_ignore_ascii_case("y") {
            println!("   Aborted.");
            return Ok(());
        }
    }

    println!();
    println!("{}", "🔑 Cora Auth Setup".bold());
    println!("{}", "   Choose your LLM provider:".dimmed());
    println!();

    // List known providers
    for (i, preset) in PRESETS.iter().enumerate() {
        println!(
            "  {} {} — {} (model: {})",
            format!("[{}]", i + 1).cyan().bold(),
            preset.name.bold(),
            preset.default_base_url.dimmed(),
            preset.default_model.dimmed(),
        );
    }
    // Custom option
    println!(
        "  {} {} — use any OpenAI-compatible endpoint",
        format!("[{}]", PRESETS.len() + 1).cyan().bold(),
        "custom".bold(),
    );
    println!();

    print!(
        "  {} ",
        format!("Select provider [1-{}]:", PRESETS.len() + 1).bold()
    );
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice_num: usize = choice
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid choice — enter a number"))?;

    if choice_num == 0 || choice_num > PRESETS.len() + 1 {
        anyhow::bail!(
            "Invalid choice — pick a number between 1 and {}",
            PRESETS.len() + 1
        );
    }

    let (provider, base_url, model) = if choice_num <= PRESETS.len() {
        // Known provider — just need API key
        let preset = &PRESETS[choice_num - 1];
        println!();
        println!(
            "  {} {} ({})",
            "→".green(),
            "Provider:".bold(),
            preset.name.green()
        );
        println!(
            "  {} {} ({})",
            "→".green(),
            "Model:".bold(),
            preset.default_model.green()
        );
        println!(
            "  {} {} ({})",
            "→".green(),
            "Base URL:".bold(),
            preset.default_base_url.dimmed()
        );
        println!();
        (
            preset.name.to_string(),
            preset.default_base_url.to_string(),
            preset.default_model.to_string(),
        )
    } else {
        // Custom provider — collect all details
        println!();
        println!("  {} {}", "→".green(), "Custom provider setup".bold());
        println!();

        let provider = prompt_input("  Provider name (e.g. my-llm):")?;
        let base_url = prompt_input("  Base URL (e.g. https://api.example.com/v1):")?;
        let model = prompt_input("  Default model (e.g. gpt-4o):")?;

        if provider.is_empty() || base_url.is_empty() || model.is_empty() {
            anyhow::bail!("Provider name, base URL, and model are required for custom providers");
        }

        (provider, base_url, model)
    };

    // Collect API key
    println!();
    print!("  🔑 Enter your API key: ");
    io::stdout().flush()?;

    let mut key = String::new();
    io::stdin().read_line(&mut key)?;
    let key = key.trim().to_string();

    if key.is_empty() {
        anyhow::bail!("API key cannot be empty");
    }

    // Save API key + provider info
    loader::save_api_key(&key)?;
    loader::save_provider_info(&provider, &base_url, &model)?;

    println!();
    println!(
        "{} API key saved to {}",
        "✅".green().bold(),
        "~/.cora/auth.toml".green()
    );
    println!(
        "{} Provider: {} | Model: {} | Base: {}",
        "   ".dimmed(),
        provider.bold(),
        model.bold(),
        base_url.dimmed()
    );
    println!(
        "{}",
        "   This file is local to your machine and not committed to git.".dimmed()
    );
    println!();
    println!(
        "{} Run {} to verify, or {} to start reviewing.",
        "Next:".bold(),
        "cora auth status".cyan(),
        "cora review".cyan()
    );

    Ok(())
}

/// Prompt for a non-empty string with label.
fn prompt_input(label: &str) -> Result<String> {
    print!("{} ", label);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Execute `auth status` — show whether an API key is configured and which provider.
pub fn execute_auth_status() -> Result<()> {
    let status = loader::auth_status()?;

    if status.has_key {
        println!("{} API key is configured.", "✅".green().bold());
        println!("   Source: {}", status.source);

        // Show stored provider info if available
        if let Some(info) = loader::load_provider_info()? {
            println!();
            println!("   {} {}", "Provider:".bold(), info.provider.green());
            println!("   {} {}", "Model:".bold(), info.model.green());
            println!("   {} {}", "Base URL:".bold(), info.base_url.dimmed());
        } else {
            println!(
                "   {} No provider info stored — run {} to set up",
                "ℹ️".yellow(),
                "cora auth login".cyan()
            );
        }
    } else {
        println!("{} No API key configured.", "❌".red().bold());
        println!("   Set it via:");
        println!("     • {} (interactive setup)", "cora auth login".cyan());
        println!("     • CORA_API_KEY environment variable");
        println!("     • Provider-specific env vars: OPENAI_API_KEY, ANTHROPIC_API_KEY, etc.");
    }

    Ok(())
}

/// Execute `auth remove` — delete the stored API key and provider info.
pub fn execute_auth_remove() -> Result<()> {
    let status = loader::auth_status()?;
    if !status.has_key && std::env::var("CORA_API_KEY").is_err() {
        println!("{}", "No API key found to remove.".yellow());
        return Ok(());
    }

    loader::remove_api_key()?;
    loader::remove_provider_info()?;
    println!(
        "{} API key and provider info removed from local config.",
        "✅".green().bold()
    );
    println!(
        "{}",
        "   If you set CORA_API_KEY in your shell, remove it there too.".dimmed()
    );

    Ok(())
}
