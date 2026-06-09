//! `cora profile` subcommand — list, show, and validate quality profiles.

use anyhow::Result;
use colored::Colorize;

use crate::engine::profiles;

/// Execute `cora profile list` — show all available built-in profiles.
pub fn execute_profile_list() -> Result<()> {
    println!("{}", "Available Quality Profiles:".bold());
    println!();

    for name in profiles::BUILTIN_PROFILES {
        if let Some(p) = profiles::load_builtin(name) {
            println!(
                "  {} {}",
                p.name.bold().cyan(),
                format!("(v{})", p.version).dimmed()
            );
            println!("    {}", p.description.dimmed());
            println!(
                "    {} focus areas, {} ignored, tone: {}",
                p.focus_areas.len().to_string().green(),
                p.ignore_areas.len(),
                p.review_style.tone
            );
            println!();
        }
    }

    println!("{}", "Usage:".bold());
    println!("  {}  # in .cora.yaml", "profile: security-first".cyan());
    println!(
        "  {}  # extend with overrides",
        "profile: { extends: rust-strict, ... }".cyan()
    );
    println!(
        "  {}  # from file",
        "profile: ./my-team-profile.yaml".cyan()
    );

    Ok(())
}

/// Execute `cora profile show <name>` — display a profile's full definition.
pub fn execute_profile_show(name: &str) -> Result<()> {
    let profile = profiles::load_builtin(name).ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown profile '{}'. Available: {}",
            name,
            profiles::BUILTIN_PROFILES.join(", ")
        )
    })?;

    println!(
        "{} {}",
        profile.name.bold().cyan(),
        format!("(v{})", profile.version).dimmed()
    );
    println!("{}", profile.description);
    println!();

    // Focus areas
    if !profile.focus_areas.is_empty() {
        println!("{}", "Focus Areas:".bold());
        let mut areas = profile.focus_areas.clone();
        areas.sort_by_key(|b| std::cmp::Reverse(b.weight));

        for area in &areas {
            let action_label = if area.action == "block" {
                "BLOCK".red().bold().to_string()
            } else {
                "WARN".yellow().to_string()
            };
            println!(
                "  {} [weight: {}/10, {}]",
                area.id.bold(),
                area.weight,
                action_label
            );
            for rule in &area.rules {
                println!("    - {rule}");
            }
        }
        println!();
    }

    // Ignore areas
    if !profile.ignore_areas.is_empty() {
        println!(
            "{} {}",
            "Ignored:".bold(),
            profile.ignore_areas.join(", ").dimmed()
        );
        println!();
    }

    // Review style
    println!("{}", "Review Style:".bold());
    println!("  tone: {}", profile.review_style.tone);
    println!("  detail_level: {}", profile.review_style.detail_level);
    println!("  suggest_fixes: {}", profile.review_style.suggest_fixes);
    println!(
        "  max_findings: {}",
        profile
            .review_style
            .max_findings
            .map_or_else(|| "unlimited".to_string(), |m| m.to_string())
    );

    Ok(())
}

/// Execute `cora profile validate <path>` — validate a custom profile YAML file.
pub fn execute_profile_validate(path: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("cannot read '{}': {e}", path))?;

    match profiles::parse_profile_yaml(&content) {
        Ok(profile) => {
            println!("{} Valid profile: {}", "✅".green(), profile.name.bold());
            if !profile.description.is_empty() {
                println!("   {}", profile.description.dimmed());
            }
            println!(
                "   {} focus areas, {} ignore areas",
                profile.focus_areas.len(),
                profile.ignore_areas.len()
            );
            println!(
                "   Review style: tone={}, detail={}",
                profile.review_style.tone, profile.review_style.detail_level
            );
            Ok(())
        }
        Err(e) => {
            anyhow::bail!("Invalid profile: {e}");
        }
    }
}
