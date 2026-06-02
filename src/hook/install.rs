use anyhow::{Context, Result};
use tracing::debug;

use crate::hook::template::HOOK_TEMPLATE;

/// Install the pre-commit hook to `.git/hooks/pre-commit`.
pub fn install_hook() -> Result<String> {
    let hooks_dir = find_git_hooks_dir()?;
    let hook_path = hooks_dir.join("pre-commit");

    // Check if a hook already exists
    if hook_path.is_file() {
        let existing = std::fs::read_to_string(&hook_path)?;
        if existing.contains("cora") {
            // Already has cora hook — back up and overwrite
            let backup = hooks_dir.join("pre-commit.cora.bak");
            std::fs::copy(&hook_path, &backup)?;
            debug!(path = %backup.display(), "backed up existing hook");
        } else {
            // Different hook — back up and append cora
            let backup = hooks_dir.join("pre-commit.pre-cora.bak");
            std::fs::copy(&hook_path, &backup)?;
            debug!(path = %backup.display(), "backed up existing hook");
        }
    }

    std::fs::write(&hook_path, HOOK_TEMPLATE)
        .with_context(|| format!("failed to write {}", hook_path.display()))?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o755);
        std::fs::set_permissions(&hook_path, perms)?;
    }

    let path_str = hook_path.display().to_string();
    debug!(path = %path_str, "installed pre-commit hook");
    Ok(path_str)
}

/// Uninstall the cora pre-commit hook.
///
/// Restores from backup if one exists, otherwise removes the hook.
pub fn uninstall_hook() -> Result<()> {
    let hooks_dir = find_git_hooks_dir()?;
    let hook_path = hooks_dir.join("pre-commit");
    let backup_path = hooks_dir.join("pre-commit.cora.bak");
    let pre_backup = hooks_dir.join("pre-commit.pre-cora.bak");

    if !hook_path.is_file() {
        return Ok(()); // nothing to do
    }

    // Check if it's a cora hook
    let content = std::fs::read_to_string(&hook_path).unwrap_or_default();
    if !content.contains("cora") {
        debug!("hook exists but is not a cora hook — leaving it");
        return Ok(());
    }

    if backup_path.is_file() {
        std::fs::rename(&backup_path, &hook_path).context("failed to restore backup hook")?;
        debug!("restored hook from backup");
    } else if pre_backup.is_file() {
        std::fs::rename(&pre_backup, &hook_path).context("failed to restore pre-cora backup")?;
        debug!("restored pre-cora hook from backup");
    } else {
        std::fs::remove_file(&hook_path).context("failed to remove hook")?;
        debug!("removed cora hook");
    }

    Ok(())
}

/// Find the .git/hooks directory for the current repository.
fn find_git_hooks_dir() -> Result<std::path::PathBuf> {
    // Try git rev-parse --git-dir
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .context("failed to run git — are you in a git repository?")?;

    if !output.status.success() {
        anyhow::bail!("not inside a git repository");
    }

    let git_dir = String::from_utf8(output.stdout)
        .context("git dir path is not valid UTF-8")?
        .trim()
        .to_string();

    let hooks_dir = std::path::PathBuf::from(&git_dir).join("hooks");

    if !hooks_dir.exists() {
        std::fs::create_dir_all(&hooks_dir)
            .with_context(|| format!("failed to create {}", hooks_dir.display()))?;
    }

    Ok(hooks_dir)
}

/// Check whether the cora pre-commit hook is installed.
///
/// Kept for API completeness — useful for future `cora hook status` and
/// guard logic in pre-commit hook template.
#[allow(dead_code)]
pub fn is_hook_installed() -> Result<bool> {
    let hooks_dir = find_git_hooks_dir()?;
    let hook_path = hooks_dir.join("pre-commit");

    if !hook_path.is_file() {
        return Ok(false);
    }

    let content = std::fs::read_to_string(&hook_path).unwrap_or_default();
    Ok(content.contains("cora"))
}
