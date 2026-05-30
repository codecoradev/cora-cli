use std::path::Path;

use anyhow::{Context, Result};
use git2::{Repository, Status, StatusOptions, StatusShow, TreeWalkMode, TreeWalkResult};
use glob::Pattern;

/// Open the git repository at or above the current working directory.
pub fn open_repo() -> Result<Repository> {
    Repository::discover(std::env::current_dir()?).context("not inside a git repository")
}

/// List all tracked files in the repository.
pub fn list_tracked_files() -> Result<Vec<String>> {
    let repo = open_repo()?;
    let head = repo.head().context("no HEAD reference")?;
    let commit = head.peel_to_commit().context("HEAD is not a commit")?;
    let tree = commit.tree().context("failed to read tree")?;

    let mut files = Vec::new();
    tree.walk(TreeWalkMode::PreOrder, |_root, entry| {
        if entry.kind() == Some(git2::ObjectType::Blob) {
            if let Some(name) = entry.name() {
                if !is_ignored_dir(name) {
                    files.push(name.to_string());
                }
            }
        }
        TreeWalkResult::Ok
    })
    .context("failed to walk tree")?;

    Ok(files)
}

/// Check if a file path falls in a commonly ignored directory.
fn is_ignored_dir(path: &str) -> bool {
    let ignored_prefixes = [
        "node_modules/",
        ".git/",
        "vendor/",
        "__pycache__/",
        ".venv/",
        "venv/",
        "target/",
        "build/",
        "dist/",
        ".next/",
        "coverage/",
    ];
    for prefix in &ignored_prefixes {
        if path.starts_with(prefix) {
            return true;
        }
    }
    false
}

/// List files that changed on the current branch compared to `base_branch`.
pub fn list_changed_files(base_branch: &str) -> Result<Vec<String>> {
    let repo = open_repo()?;

    let base_oid = repo
        .revparse_single(base_branch)
        .with_context(|| format!("cannot resolve branch '{base_branch}'"))?
        .peel_to_commit()?
        .id();

    let _head_oid = repo.head()?.peel_to_commit()?.id();

    let base_tree = repo
        .find_tree(base_oid)
        .context("failed to find base tree")?;
    let head_commit = repo
        .head()?
        .peel_to_commit()
        .context("HEAD is not a commit")?;
    let head_tree = repo
        .find_tree(head_commit.tree_id())
        .context("failed to find HEAD tree")?;

    let mut opts = git2::DiffOptions::new();
    let diff = repo
        .diff_tree_to_tree(Some(&base_tree), Some(&head_tree), Some(&mut opts))
        .context("failed to diff trees")?;

    let mut files = Vec::new();
    for delta in diff.deltas() {
        if let Some(path) = delta.new_file().path() {
            if let Some(name) = path.to_str() {
                files.push(name.to_string());
            }
        }
    }

    Ok(files)
}

/// List files that are currently staged (in the index).
pub fn list_staged_files() -> Result<Vec<String>> {
    let repo = open_repo()?;
    let head = repo.head().ok();
    let head_tree = head
        .and_then(|h| h.peel_to_commit().ok())
        .and_then(|c| c.tree().ok());

    let mut opts = git2::DiffOptions::new();
    let diff = repo
        .diff_tree_to_index(head_tree.as_ref(), None, Some(&mut opts))
        .context("failed to diff staged files")?;

    let mut files = Vec::new();
    for delta in diff.deltas() {
        if let Some(path) = delta.new_file().path() {
            if let Some(name) = path.to_str() {
                files.push(name.to_string());
            }
        }
    }

    Ok(files)
}

/// List all modified/added/deleted files (staged + unstaged).
pub fn list_all_changed_files() -> Result<Vec<String>> {
    let repo = open_repo()?;

    let mut opts = StatusOptions::new();
    opts.show(StatusShow::IndexAndWorkdir)
        .include_untracked(true)
        .recurse_untracked_dirs(true);

    let statuses = repo
        .statuses(Some(&mut opts))
        .context("failed to get git status")?;

    let mut files = Vec::new();
    for entry in statuses.iter() {
        if entry.status().intersects(
            Status::INDEX_NEW
                | Status::INDEX_MODIFIED
                | Status::INDEX_DELETED
                | Status::WT_MODIFIED
                | Status::WT_NEW
                | Status::WT_DELETED,
        ) {
            if let Some(path) = entry.path() {
                files.push(path.to_string());
            }
        }
    }

    files.sort();
    files.dedup();
    Ok(files)
}

/// Filter a list of file paths using glob include/exclude patterns.
pub fn filter_by_globs(files: &[String], include: &[String], exclude: &[String]) -> Vec<String> {
    let include_patterns: Vec<Pattern> = include
        .iter()
        .filter_map(|p| Pattern::new(p).ok())
        .collect();

    let exclude_patterns: Vec<Pattern> = exclude
        .iter()
        .filter_map(|p| Pattern::new(p).ok())
        .collect();

    files
        .iter()
        .filter(|f| {
            let path = Path::new(f);
            let included = include_patterns.is_empty()
                || include_patterns.iter().any(|p| p.matches_path(path));

            let excluded = exclude_patterns.iter().any(|p| p.matches_path(path));

            included && !excluded
        })
        .cloned()
        .collect()
}

/// Check if a given path is gitignored.
pub fn is_gitignored(path: &Path) -> Result<bool> {
    let repo = open_repo()?;
    let relative = path
        .strip_prefix(repo.workdir().unwrap_or(path))
        .unwrap_or(path);

    Ok(repo.is_path_ignored(relative)?)
}
