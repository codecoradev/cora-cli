use std::io::{self, Write};

use anyhow::Result;
use colored::Colorize;

use crate::config::loader;

/// Execute `auth login` — prompt for API key and save it.
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

    print!("🔑 Enter your API key: ");
    io::stdout().flush()?;

    let mut key = String::new();
    io::stdin().read_line(&mut key)?;
    let key = key.trim().to_string();

    if key.is_empty() {
        anyhow::bail!("API key cannot be empty");
    }

    loader::save_api_key(&key)?;
    println!(
        "{} API key saved to ~/.cora/config.toml",
        "✅".green().bold()
    );
    println!(
        "{}",
        "   This file is local to your machine and not committed to git.".dimmed()
    );

    Ok(())
}

/// Execute `auth status` — show whether an API key is configured.
pub fn execute_auth_status() -> Result<()> {
    let status = loader::auth_status()?;

    if status.has_key {
        println!("{} API key is configured.", "✅".green().bold());
        println!("   Source: {}", status.source);
    } else {
        println!("{} No API key configured.", "❌".red().bold());
        println!("   Set it via:");
        println!("     • CORA_API_KEY environment variable");
        println!("     • `cora auth login` command");
        println!("     • `cora review --api-key <key>` flag");
    }

    Ok(())
}

/// Execute `auth remove` — delete the stored API key.
pub fn execute_auth_remove() -> Result<()> {
    let status = loader::auth_status()?;
    if !status.has_key && std::env::var("CORA_API_KEY").is_err() {
        println!("{}", "No API key found to remove.".yellow());
        return Ok(());
    }

    loader::remove_api_key()?;
    println!("{} API key removed from local config.", "✅".green().bold());
    println!(
        "{}",
        "   If you set CORA_API_KEY in your shell, remove it there too.".dimmed()
    );

    Ok(())
}
