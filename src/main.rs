// Suppress unused warnings for utility functions not yet called from main
// These will be used as the codebase grows
#![allow(dead_code)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod commands;
mod config;
mod engine;
mod formatters;
mod git;
mod hook;

use commands::{auth, completion, config_cmd, hook_cmd, init, providers, review, scan, upload};
use config::loader;
use formatters::OutputFormat;

/// Cora — AI Code Review CLI with BYOK (Bring Your Own Key)
///
/// Review diffs and scan projects using any OpenAI-compatible LLM.
/// Configure via .cora.yaml, CLI flags, or environment variables.
#[derive(Parser, Debug)]
#[clap(
    name = "cora",
    version,
    about = "CLI-first AI code review — BYOK, diff/scan/branch, pre-commit hooks",
    long_version = concat!(env!("CARGO_PKG_VERSION"), "\n", env!("CARGO_PKG_REPOSITORY"))
)]
struct Cli {
    /// Global options
    #[command(flatten)]
    global: GlobalOptions,

    /// Subcommand
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Args, Debug, Clone)]
struct GlobalOptions {
    /// Path to config file (default: auto-discover .cora.yaml)
    #[clap(long, global = true, env = "CORA_CONFIG")]
    pub config: Option<String>,

    /// Output format: pretty, json, compact, sarif
    #[clap(long, global = true, value_parser = ["pretty", "json", "compact", "sarif"], env = "CORA_FORMAT")]
    pub format: Option<String>,

    /// Disable colored output
    #[clap(long, global = true, env = "CORA_NO_COLOR")]
    pub no_color: bool,

    /// LLM provider name (e.g. openai, anthropic, ollama)
    #[clap(long, global = true, env = "CORA_PROVIDER")]
    pub provider: Option<String>,

    /// LLM model name (e.g. gpt-4o-mini, claude-3-haiku)
    #[clap(long, global = true, env = "CORA_MODEL")]
    pub model: Option<String>,

    /// API base URL
    #[clap(long, global = true, env = "CORA_BASE_URL")]
    pub base_url: Option<String>,

    /// API key (or set CORA_API_KEY env var, or use `cora auth login`)
    #[clap(long, global = true, env = "CORA_API_KEY")]
    pub api_key: Option<String>,

    /// Enable verbose logging
    #[clap(long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Review a git diff (staged, unpushed, or branch)
    Review {
        /// Review staged changes (git diff --cached)
        #[clap(long)]
        staged: bool,

        /// Review unpushed changes (HEAD vs @{u})
        #[clap(long)]
        unpushed: bool,

        /// Review changes vs a base branch
        #[clap(long)]
        base: Option<String>,

        /// Review changes from a git commit ref (e.g. HEAD, HEAD~3..HEAD, abc123)
        #[clap(long)]
        commit: Option<String>,

        /// Read diff from a file instead of git
        #[clap(long)]
        diff_file: Option<String>,

        /// Review unstaged changes (working tree)
        #[clap(long)]
        unstaged: bool,

        /// Stream LLM response tokens in real-time
        #[clap(long)]
        stream: bool,

        /// Suppress all output except the formatted review result
        #[clap(long, short)]
        quiet: bool,

        /// Filter results by minimum severity (info, minor, major, critical)
        #[clap(long, value_parser = ["info", "minor", "major", "critical"])]
        severity: Option<String>,

        /// Upload SARIF output to GitHub Code Scanning after review
        /// (implies --format sarif)
        #[clap(long)]
        upload: bool,

        /// GitHub repository for upload (default: from git remote origin)
        #[clap(long, env = "GITHUB_REPOSITORY")]
        repo: Option<String>,

        /// GitHub ref for upload (default: current branch)
        #[clap(long, env = "GITHUB_REF")]
        ref_name: Option<String>,

        /// GitHub token for upload (default: GITHUB_TOKEN env var)
        #[clap(long, env = "GITHUB_TOKEN")]
        token: Option<String>,
    },

    /// Scan a project directory for issues
    Scan {
        /// Root directory to scan (default: current directory)
        #[clap(long)]
        path: Option<String>,

        /// Include glob patterns (e.g. "src/**/*.rs")
        #[clap(long)]
        include: Vec<String>,

        /// Exclude glob patterns (e.g. "vendor/**")
        #[clap(long)]
        exclude: Vec<String>,

        /// Additional file extensions to scan
        #[clap(long)]
        extensions: Vec<String>,

        /// Only scan files changed since last scan (uses ~/.cora/scan-cache.json)
        #[clap(long)]
        incremental: bool,

        /// Focus areas for review (overrides config)
        #[clap(long)]
        focus: Vec<String>,
    },

    /// Upload a SARIF file to GitHub Code Scanning
    UploadSarif {
        /// Path to SARIF file to upload (default: reads from stdin)
        file: Option<String>,

        /// GitHub repository (default: from git remote origin)
        #[clap(long, env = "GITHUB_REPOSITORY")]
        repo: Option<String>,

        /// GitHub ref (default: current branch)
        #[clap(long, env = "GITHUB_REF")]
        ref_name: Option<String>,

        /// GitHub token (default: GITHUB_TOKEN env var)
        #[clap(long, env = "GITHUB_TOKEN")]
        token: Option<String>,
    },

    /// Create a .cora.yaml config file in the current directory
    Init {
        /// Overwrite existing config file
        #[clap(long)]
        force: bool,
    },

    /// Manage pre-commit git hooks
    Hook {
        #[command(subcommand)]
        action: HookAction,
    },

    /// Manage API key authentication
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },

    /// View or set configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// List detected/available LLM providers
    Providers,

    /// Generate shell completion scripts
    Completion {
        /// Shell name: bash, zsh, fish, or powershell
        shell: String,
    },
}

#[derive(Subcommand, Debug)]
enum HookAction {
    /// Install the pre-commit hook
    Install,
    /// Uninstall the pre-commit hook
    Uninstall,
}

#[derive(Subcommand, Debug)]
enum AuthAction {
    /// Save an API key to local config
    Login,
    /// Check if an API key is configured
    Status,
    /// Remove the stored API key
    Remove,
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    /// Show the current resolved configuration
    Show,
    /// Set a configuration value (keys: model, provider, format, severity)
    Set {
        /// Configuration key to set
        key: String,
        /// Value to assign
        value: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging based on verbosity
    let log_level = if cli.global.verbose {
        Level::DEBUG
    } else {
        Level::WARN
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("cora=warn")),
        )
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Handle --no-color
    if cli.global.no_color {
        colored::control::set_override(false);
    }

    // Dispatch based on subcommand
    let exit_code = match cli.command {
        Command::Review {
            staged,
            unpushed,
            base,
            commit,
            diff_file,
            unstaged,
            stream,
            quiet,
            severity,
            upload,
            repo,
            ref_name,
            token,
        } => {
            cmd_review(
                &cli.global,
                ReviewOpts {
                    staged,
                    unpushed,
                    base,
                    commit,
                    diff_file,
                    unstaged,
                    stream,
                    quiet,
                    severity,
                    upload,
                    repo,
                    ref_name,
                    token,
                },
            )
            .await?
        }
        Command::Scan {
            path,
            include,
            exclude,
            extensions,
            incremental,
            focus,
        } => {
            cmd_scan(
                &cli.global,
                ScanOpts {
                    path,
                    include,
                    exclude,
                    extensions,
                    incremental,
                    focus,
                },
            )
            .await?
        }
        Command::UploadSarif {
            file,
            repo,
            ref_name,
            token,
        } => {
            let opts = upload::UploadOptions {
                file,
                repo,
                ref_name,
                token,
            };
            upload::execute_upload(&opts).await?
        }
        Command::Init { force } => {
            if force {
                init::execute_init_force()?;
            } else {
                init::execute_init()?;
            }
            0
        }
        Command::Hook { action } => match action {
            HookAction::Install => {
                hook_cmd::execute_hook_install()?;
                0
            }
            HookAction::Uninstall => {
                hook_cmd::execute_hook_uninstall()?;
                0
            }
        },
        Command::Auth { action } => {
            match action {
                AuthAction::Login => auth::execute_auth_login()?,
                AuthAction::Status => auth::execute_auth_status()?,
                AuthAction::Remove => auth::execute_auth_remove()?,
            }
            0
        }
        Command::Config { action } => {
            match action {
                ConfigAction::Show => {
                    config_cmd::execute_config_show()?;
                    0
                }
                ConfigAction::Set { key, value } => {
                    config_cmd::execute_config_set(&key, &value)?;
                    0
                }
            }
        }
        Command::Providers => {
            providers::execute_providers()?;
            0
        }
        Command::Completion { shell } => {
            completion::execute_completion(&shell)?;
            0
        }
    };

    std::process::exit(exit_code);
}

/// Struct to hold review options from CLI.
struct ReviewOpts {
    staged: bool,
    unpushed: bool,
    base: Option<String>,
    commit: Option<String>,
    diff_file: Option<String>,
    unstaged: bool,
    stream: bool,
    quiet: bool,
    severity: Option<String>,
    upload: bool,
    repo: Option<String>,
    ref_name: Option<String>,
    token: Option<String>,
}

/// Struct to hold scan options from CLI.
struct ScanOpts {
    path: Option<String>,
    include: Vec<String>,
    exclude: Vec<String>,
    extensions: Vec<String>,
    incremental: bool,
    focus: Vec<String>,
}

/// Handle the `review` subcommand.
async fn cmd_review(globals: &GlobalOptions, opts: ReviewOpts) -> Result<i32> {
    let config = loader::load_config(
        globals.config.as_deref(),
        globals.provider.as_deref(),
        globals.model.as_deref(),
        globals.base_url.as_deref(),
        globals.api_key.as_deref(),
        globals.format.as_deref(),
        globals.no_color,
    )?;

    let llm_config = loader::build_llm_config(&config, globals.api_key.as_deref())?;

    // If --upload is set, force SARIF format
    let effective_format = if opts.upload {
        OutputFormat::Sarif
    } else {
        resolve_format(globals.format.as_deref(), &config)?
    };

    let review_opts = review::ReviewOptions {
        staged: opts.staged,
        unpushed: opts.unpushed,
        base: opts.base.clone(),
        commit: opts.commit.clone(),
        diff_file: opts.diff_file.clone(),
        unstaged: opts.unstaged,
        max_diff_size: None,
        stream: opts.stream,
        quiet: opts.quiet,
        severity: opts.severity.clone(),
    };

    // When streaming and not quiet, show a simpler message
    if opts.stream && !opts.quiet {
        eprintln!(
            "{}",
            format!(
                "⏳ Streaming review from {} ({})…",
                llm_config.provider, llm_config.model
            )
            .dimmed()
        );
    }

    // Execute the review (returns formatted output)
    let result =
        review::execute_review(&config, &llm_config, &review_opts, effective_format).await?;

    // Print the formatted output
    print!("{}", result.output);

    // If --upload, send the SARIF output to GitHub Code Scanning
    if opts.upload {
        let sarif_content = result.output;
        upload_sarif_content(&sarif_content, &opts.repo, &opts.ref_name, &opts.token).await?;
    }

    Ok(result.exit_code)
}

/// Upload a SARIF string to GitHub Code Scanning.
async fn upload_sarif_content(
    sarif_content: &str,
    repo: &Option<String>,
    ref_name: &Option<String>,
    token: &Option<String>,
) -> Result<i32> {
    use std::io::Write;

    // Write SARIF to a temp file and upload it
    let tmp_dir = std::env::temp_dir();
    let tmp_path = tmp_dir.join("cora-sarif-upload.json");

    {
        let mut file = std::fs::File::create(&tmp_path)?;
        file.write_all(sarif_content.as_bytes())?;
    }

    println!(
        "{}",
        "\n→ Uploading SARIF to GitHub Code Scanning...".cyan()
    );

    let upload_opts = upload::UploadOptions {
        file: Some(tmp_path.to_string_lossy().to_string()),
        repo: repo.clone(),
        ref_name: ref_name.clone(),
        token: token.clone(),
    };

    let exit = upload::execute_upload(&upload_opts).await?;

    // Clean up temp file
    let _ = std::fs::remove_file(&tmp_path);

    Ok(exit)
}

/// Handle the `scan` subcommand.
async fn cmd_scan(globals: &GlobalOptions, opts: ScanOpts) -> Result<i32> {
    let config = loader::load_config(
        globals.config.as_deref(),
        globals.provider.as_deref(),
        globals.model.as_deref(),
        globals.base_url.as_deref(),
        globals.api_key.as_deref(),
        globals.format.as_deref(),
        globals.no_color,
    )?;

    let llm_config = loader::build_llm_config(&config, globals.api_key.as_deref())?;

    let format = resolve_format(globals.format.as_deref(), &config)?;

    let scan_opts = scan::ScanOptions {
        path: opts.path,
        include: opts.include,
        exclude: opts.exclude,
        extensions: opts.extensions,
        incremental: opts.incremental,
        focus: opts.focus,
    };

    scan::execute_scan(&config, &llm_config, &scan_opts, format).await
}

/// Resolve the output format: CLI flag > config > default.
fn resolve_format(
    cli_format: Option<&str>,
    config: &crate::config::schema::Config,
) -> Result<OutputFormat> {
    let fmt_str = cli_format
        .map(|s| s.to_string())
        .unwrap_or_else(|| config.output.format.clone());
    OutputFormat::from_str_loose(&fmt_str)
}
