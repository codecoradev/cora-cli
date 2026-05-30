use anyhow::Result;
use colored::Colorize;

use crate::config::providers::{PRESETS, detected_presets};

/// Execute the `cora providers` subcommand.
pub fn execute_providers() -> Result<()> {
    let detected = detected_presets();

    println!("{}", "Available LLM Providers".bold().underline());
    println!();

    for preset in PRESETS {
        let is_detected = detected.iter().any(|d| d.name == preset.name);
        let status = if is_detected {
            "✓".green().to_string()
        } else {
            "✗".dimmed().to_string()
        };

        println!("  {} {}", status, preset.name.bold());

        if is_detected {
            println!("    {}  {} (detected)", "key:".dimmed(), preset.env_key);
        } else {
            println!("    {}  set {} to enable", "key:".dimmed(), preset.env_key);
        }

        println!("    {}  {}", "model:".dimmed(), preset.default_model);
        println!("    {}  {}", "base:".dimmed(), preset.default_base_url);
        println!(
            "    {}  {} = <custom url>",
            "url override:".dimmed(),
            preset.env_url
        );
        println!();
    }

    // Summary
    if detected.is_empty() {
        println!(
            "{} No providers detected. Set an API key env var from the list above.",
            "⚠️".yellow()
        );
        println!(
            "{}",
            "   Or set CORA_API_KEY with --provider to use any OpenAI-compatible endpoint."
                .dimmed()
        );
    } else if detected.len() == 1 {
        println!(
            "{} Auto-detected provider: {}",
            "✓".green(),
            detected[0].name.bold()
        );
        println!(
            "{}",
            "   Set CORA_PROVIDER or use --provider to override.".dimmed()
        );
    } else {
        let names: Vec<&str> = detected.iter().map(|d| d.name).collect();
        println!(
            "{} Multiple providers detected: {}",
            "ℹ️".cyan(),
            names.join(", ").bold()
        );
        println!(
            "{}",
            "   Using first detected. Set CORA_PROVIDER or use --provider to choose.".dimmed()
        );
    }

    Ok(())
}
