use std::io::Read;

use anyhow::{Context, Result, bail};
use colored::Colorize;
use serde_json::json;

/// Upload options for the upload-sarif subcommand.
pub struct UploadOptions {
    /// Path to SARIF file to upload (None = read from stdin).
    pub file: Option<String>,
    /// GitHub repository in "owner/repo" format.
    pub repo: Option<String>,
    /// Git ref name for the upload.
    pub ref_name: Option<String>,
    /// GitHub token for authentication.
    pub token: Option<String>,
}

/// Upload a SARIF file to GitHub Code Scanning.
///
/// Reads the SARIF content from the specified file or stdin, then POSTs it
/// to the GitHub Code Scanning API.
pub async fn execute_upload(opts: &UploadOptions) -> Result<i32> {
    let token = opts
        .token
        .as_deref()
        .context("GITHUB_TOKEN is required (set env var or pass --token)")?
        .to_string();

    let (owner, repo) = resolve_repo(opts.repo.as_deref())?;
    let ref_name = resolve_ref(opts.ref_name.as_deref())?;

    tracing::debug!(
        owner = %owner,
        repo = %repo,
        ref_name = %ref_name,
        "uploading SARIF to GitHub Code Scanning"
    );

    // Read SARIF content
    let sarif_content = read_sarif(opts.file.as_deref())?;

    // Validate it's valid JSON (basic sanity check)
    let parsed: serde_json::Value =
        serde_json::from_str(&sarif_content).context("SARIF file does not contain valid JSON")?;

    // Ensure it has the $schema field (basic SARIF validation)
    if parsed.get("$schema").is_none() {
        bail!("File does not appear to be a valid SARIF document (missing $schema)");
    }

    println!(
        "{} Uploading SARIF to {}/{} (ref: {})",
        "→".cyan(),
        owner,
        repo,
        ref_name
    );

    // Build the upload request
    let url = format!(
        "https://api.github.com/repos/{}/{}/code-scanning/sarifs",
        owner, repo
    );

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({ "commit_sha": ref_name, "sarif": &sarif_content }))
        .send()
        .await
        .context("Failed to send request to GitHub API")?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if status.is_success() {
        println!(
            "{} SARIF uploaded successfully to GitHub Code Scanning.",
            "✓".green()
        );

        // Parse the response for additional info
        if let Ok(resp_json) = serde_json::from_str::<serde_json::Value>(&body) {
            if let Some(id) = resp_json.get("id").and_then(|v| v.as_str()) {
                println!("   Analysis ID: {}", id);
            }
        }

        Ok(0)
    } else {
        // Try to extract a useful error message
        let error_msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| {
                v.get("message")
                    .or_else(|| v.get("errors")?.get(0)?.get("message"))
                    .and_then(|m| m.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| body.clone());

        eprintln!(
            "{} Upload failed (HTTP {}): {}",
            "✗".red(),
            status,
            error_msg
        );

        // Common error hints
        let hint = match status.as_u16() {
            401 => Some("Check that your GITHUB_TOKEN is valid and not expired."),
            403 => Some(
                "The token may lack security_events write permission. Use a token with the 'security_events' scope.",
            ),
            404 => Some("Check the repository name and that you have access to it."),
            422 => Some("The SARIF file may be invalid or exceed size limits (max 10MB)."),
            429 => Some("Rate limited. Wait a moment and try again."),
            _ => None,
        };

        if let Some(h) = hint {
            eprintln!("   {}", h.yellow());
        }

        Ok(1)
    }
}

/// Read SARIF content from a file or stdin.
fn read_sarif(file: Option<&str>) -> Result<String> {
    match file {
        Some(path) => std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read SARIF file: {}", path)),
        None => {
            eprintln!(
                "{} Reading SARIF from stdin (Ctrl+D to finish)...",
                "ℹ".blue()
            );
            let mut content = String::new();
            std::io::stdin()
                .read_to_string(&mut content)
                .context("Failed to read from stdin")?;
            Ok(content)
        }
    }
}

/// Resolve the GitHub repository from CLI flag, env var, or git remote.
fn resolve_repo(cli_repo: Option<&str>) -> Result<(String, String)> {
    if let Some(repo) = cli_repo {
        let parts: Vec<&str> = repo.split('/').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            bail!(
                "Invalid repository format '{}'. Expected 'owner/repo'.",
                repo
            );
        }
        return Ok((parts[0].to_string(), parts[1].to_string()));
    }

    // Try to detect from git remote
    match crate::git::get_repo_info() {
        Ok((owner, repo_name, _branch)) => {
            if owner == "unknown" || repo_name == "unknown" {
                bail!(
                    "Could not detect repository from git remote. \
                     Set GITHUB_REPOSITORY or pass --repo owner/repo."
                );
            }
            Ok((owner, repo_name))
        }
        Err(_) => {
            bail!(
                "Could not detect repository. Set GITHUB_REPOSITORY env var or pass --repo owner/repo."
            );
        }
    }
}

/// Resolve the git ref name from CLI flag, env var, or current HEAD.
fn resolve_ref(cli_ref: Option<&str>) -> Result<String> {
    if let Some(r) = cli_ref {
        return Ok(r.to_string());
    }

    // Try to get from git
    match crate::git::get_current_branch() {
        Ok(branch) => Ok(branch),
        Err(_) => {
            // Try to get the current HEAD commit SHA as fallback
            let output = std::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .output();

            match output {
                Ok(out) if out.status.success() => {
                    let sha = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    Ok(sha)
                }
                _ => {
                    bail!(
                        "Could not detect current git ref. Pass --ref-name or run inside a git repo."
                    );
                }
            }
        }
    }
}
