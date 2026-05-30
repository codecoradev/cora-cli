// Suppress unused warnings for utility functions not yet called from main
// These will be used as the codebase grows
#![allow(dead_code)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod commands;
mod config;
mod engine;
mod formatters;
mod git;
mod hook;

use commands::{auth, hook_cmd, init, review, scan};
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

        /// Review unstaged changes (working tree)
        #[clap(long)]
        unstaged: bool,
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
        Command::Review { staged, unpushed, base, unstaged } => {
            cmd_review(&cli.global, ReviewOpts { staged, unpushed, base, unstaged }).await?
        }
        Command::Scan { path, include, exclude, extensions } => {
            cmd_scan(&cli.global, ScanOpts { path, include, exclude, extensions }).await?
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
    };

    std::process::exit(exit_code);
}

/// Struct to hold review options from CLI.
struct ReviewOpts {
    staged: bool,
    unpushed: bool,
    base: Option<String>,
    unstaged: bool,
}

/// Struct to hold scan options from CLI.
struct ScanOpts {
    path: Option<String>,
    include: Vec<String>,
    exclude: Vec<String>,
    extensions: Vec<String>,
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

    let format = resolve_format(globals.format.as_deref(), &config)?;

    let review_opts = review::ReviewOptions {
        staged: opts.staged,
        unpushed: opts.unpushed,
        base: opts.base,
        unstaged: opts.unstaged,
        max_diff_size: None,
    };

    review::execute_review(&config, &llm_config, &review_opts, format).await
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
    };

    scan::execute_scan(&config, &llm_config, &scan_opts, format).await
}

/// Resolve the output format: CLI flag > config > default.
fn resolve_format(cli_format: Option<&str>, config: &crate::config::schema::Config) -> Result<OutputFormat> {
    let fmt_str = cli_format.map(|s| s.to_string()).unwrap_or_else(|| config.output.format.clone());
    OutputFormat::from_str_loose(&fmt_str)
}
