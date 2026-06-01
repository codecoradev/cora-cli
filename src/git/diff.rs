use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};
use git2::Repository;

/// Characters that should not appear in git refs passed to our diff commands.
/// This prevents shell injection and path traversal via crafted ref names.
const DANGEROUS_REF_CHARS: &[char] = &[
    ';', '&', '|', '`', '$', '(', ')', '<', '>', '{', '}', '\\', '\n', '\r', '\t',
];

/// Validate a git ref string for safety before passing it to git commands.
///
/// Rejects refs containing shell metacharacters or path traversal sequences (..).
fn validate_ref(ref_str: &str) -> Result<()> {
    if ref_str.contains("..") {
        // ".." is only allowed in the middle of range expressions like "HEAD..HEAD" or "main...HEAD"
        // But we need to be careful about path traversal. Allow it only for git range syntax.
        // Actually, git diff ranges use ".." and "..." which are valid git syntax.
        // The risk is when ".." appears as part of a path (e.g., "../../etc/passwd").
        // Since git refs shouldn't contain path components with "..", we reject refs
        // that start with ".." or contain "/.." or ".." at the end.
        if ref_str.starts_with("..") || ref_str.contains("/..") || ref_str.ends_with("..") {
            anyhow::bail!(
                "invalid ref '{}': contains path traversal sequence '..'",
                ref_str
            );
        }
    }

    if ref_str.contains(DANGEROUS_REF_CHARS) {
        let found: String = ref_str
            .chars()
            .filter(|c| DANGEROUS_REF_CHARS.contains(c))
            .collect();
        anyhow::bail!(
            "invalid ref '{}': contains unsafe characters: '{}'",
            ref_str,
            found
        );
    }

    Ok(())
}

/// Open the git repository at or above the current working directory.
pub fn open_repo() -> Result<Repository> {
    Repository::discover(std::env::current_dir()?).context("not inside a git repository")
}

/// Run a git command and return its stdout as a string.
fn git_cmd(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("failed to execute git command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git command failed: {stderr}");
    }

    String::from_utf8(output.stdout).context("git output is not valid UTF-8")
}

/// Get the diff of currently staged changes (git diff --cached).
pub fn get_staged_diff() -> Result<String> {
    git_cmd(&["diff", "--cached"])
}

/// Get the diff of unstaged changes (working tree vs index).
pub fn get_unstaged_diff() -> Result<String> {
    git_cmd(&["diff"])
}

/// Get the diff between the current branch and `base_branch`.
pub fn get_branch_diff(base_branch: &str) -> Result<String> {
    validate_ref(base_branch)?;
    let arg = format!("{}...HEAD", base_branch);
    git_cmd(&["diff", &arg])
}

/// Get the diff of unpushed commits (HEAD vs @{u}).
pub fn get_unpushed_diff() -> Result<String> {
    git_cmd(&["log", "-p", "@{u}..HEAD"])
}

/// Get the diff for a commit reference.
///
/// - If the ref contains `..` (e.g. `HEAD~3..HEAD`), uses `git diff <ref>`.
/// - Otherwise (e.g. `HEAD`, `abc123`), uses `git show <ref> --format=""` to
///   get just the diff without commit metadata.
pub fn get_commit_diff(ref_str: &str) -> Result<String> {
    validate_ref(ref_str)?;
    if ref_str.contains("..") {
        git_cmd(&["diff", ref_str])
    } else {
        git_cmd(&["show", ref_str, "--format="])
    }
}

/// Get the current branch name.
pub fn get_current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .context("failed to execute git rev-parse")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git rev-parse failed: {stderr}");
    }

    let name = String::from_utf8(output.stdout).context("branch name is not valid UTF-8")?;
    let name = name.trim().to_string();

    if name.is_empty() || name == "HEAD" {
        anyhow::bail!("HEAD is detached -- cannot determine branch name");
    }

    Ok(name)
}

/// Try to get repository info: (owner, repo_name, branch).
/// Owner is derived from the remote URL if possible.
pub fn get_repo_info() -> Result<(String, String, String)> {
    let repo = open_repo()?;
    let branch = get_current_branch()?;

    let remote_url = repo
        .find_remote("origin")
        .ok()
        .and_then(|r| r.url().map(|s| s.to_string()))
        .unwrap_or_default();

    let (owner, repo_name) = parse_remote_url(&remote_url);
    Ok((owner, repo_name, branch))
}

/// Parse a remote URL into (owner, repo_name).
fn parse_remote_url(url: &str) -> (String, String) {
    // Handle git@host:owner/repo.git, https://host/owner/repo.git, etc.
    let clean = url
        .trim_end_matches(".git")
        .trim_end_matches('/')
        .trim_start_matches("git@");

    let parts: Vec<&str> = clean.split('/').collect();
    if parts.len() >= 2 {
        // git@host:owner/repo  => parts = ["host:owner", "repo"]
        let maybe_owner = parts[parts.len() - 2];
        let (owner, _) = maybe_owner.split_once(':').unwrap_or((maybe_owner, ""));
        let repo_name = parts[parts.len() - 1];
        (owner.to_string(), repo_name.to_string())
    } else {
        ("unknown".to_string(), "unknown".to_string())
    }
}

/// Check if a path is inside a git worktree.
pub fn is_inside_git_repo(dir: Option<&Path>) -> bool {
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let search_dir = dir.unwrap_or(&cwd);
    Repository::discover(search_dir).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_ref_simple_name() {
        assert!(validate_ref("main").is_ok());
        assert!(validate_ref("HEAD").is_ok());
        assert!(validate_ref("feature/my-branch").is_ok());
        assert!(validate_ref("abc123").is_ok());
    }

    #[test]
    fn validate_ref_range_syntax() {
        assert!(validate_ref("HEAD~3..HEAD").is_ok());
        assert!(validate_ref("main...develop").is_ok());
        assert!(validate_ref("abc123..def456").is_ok());
    }

    #[test]
    fn validate_ref_rejects_path_traversal() {
        assert!(validate_ref("../../etc/passwd").is_err());
        assert!(validate_ref("..HEAD").is_err());
        assert!(validate_ref("HEAD..").is_err());
        assert!(validate_ref("branch/../other").is_err());
    }

    #[test]
    fn validate_ref_rejects_shell_metacharacters() {
        assert!(validate_ref("main;rm -rf /").is_err());
        assert!(validate_ref("main`whoami`").is_err());
        assert!(validate_ref("$(whoami)").is_err());
        assert!(validate_ref("main && echo pwned").is_err());
        assert!(validate_ref("main | cat").is_err());
        assert!(validate_ref("main\nextra").is_err());
    }

    #[test]
    fn validate_ref_rejects_angle_brackets() {
        assert!(validate_ref("main > /etc/passwd").is_err());
    }
}
