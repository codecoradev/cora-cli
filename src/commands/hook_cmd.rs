use anyhow::Result;
use colored::Colorize;

use crate::hook;

/// Execute `hook install` — install the pre-commit hook.
pub fn execute_hook_install() -> Result<()> {
    let path = hook::install_hook()?;

    println!(
        "{} Installed pre-commit hook to {}",
        "✅".green().bold(),
        path
    );
    println!(
        "{}",
        "   The hook will run `cora review --staged --format compact` before each commit.".dimmed()
    );
    println!(
        "{}",
        "   Use `cora hook uninstall` to remove.".dimmed()
    );

    Ok(())
}

/// Execute `hook uninstall` — remove the pre-commit hook.
pub fn execute_hook_uninstall() -> Result<()> {
    hook::uninstall_hook()?;

    println!(
        "{} Pre-commit hook removed.",
        "✅".green().bold()
    );

    Ok(())
}
