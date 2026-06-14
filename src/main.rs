use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod commands;
mod config;
mod engine;
mod error;
mod formatters;
mod git;
mod hook;
mod index;
mod mcp;
mod progress;

use commands::{
    auth, commit_cmd, completion, config_cmd, debt, hook_cmd, init, profile, providers, review,
    scan, upload,
};
use config::loader;
use engine::memory;
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

    /// API key (or set `CORA_API_KEY` env var for CI, or use `cora auth login`)
    #[clap(long, global = true, env = "CORA_API_KEY")]
    pub api_key: Option<String>,

    /// Enable verbose logging
    #[clap(long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Build or update the symbol index for code intelligence
    Index {
        /// Show index statistics instead of building
        #[clap(long)]
        stats: bool,

        /// Prune deleted files from index
        #[clap(long)]
        prune: bool,

        /// Rebuild index from scratch (drop existing)
        #[clap(long)]
        rebuild: bool,

        /// Verbose output
        #[clap(long, short)]
        verbose: bool,
    },

    /// Search the symbol index for code intelligence
    Explore {
        /// Search query (symbol name or keyword)
        query: Option<String>,

        /// Filter by symbol kind (function, struct, enum, trait, etc.)
        #[clap(long)]
        kind: Option<String>,

        /// Filter by file path prefix
        #[clap(long)]
        file: Option<String>,

        /// Filter by language
        #[clap(long)]
        language: Option<String>,

        /// Maximum results
        #[clap(long, default_value = "50")]
        limit: usize,

        /// Output as JSON
        #[clap(long)]
        json: bool,
    },

    /// Review staged changes, generate commit message, and commit
    Commit {
        /// YOLO mode — auto-commit without prompts
        #[clap(long)]
        yolo: bool,

        /// Force commit even if quality gate fails
        #[clap(long)]
        force: bool,

        /// Skip review, only generate commit message
        #[clap(long)]
        no_review: bool,

        /// Always open editor to edit message
        #[clap(long)]
        edit: bool,

        /// Stream LLM response in real-time
        #[clap(long)]
        stream: bool,

        /// Suppress all output except the result
        #[clap(long, short)]
        quiet: bool,
    },

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

        /// Override max diff size in chars (default: 50000 from config)
        #[clap(long, name = "CHARS")]
        max_diff_size: Option<usize>,

        /// Stream LLM response tokens in real-time
        #[clap(long)]
        stream: bool,

        /// Disable auto-chunking for large diffs (auto-chunk is enabled by default)
        #[clap(long)]
        no_auto_chunk: bool,

        /// Output structured NDJSON progress events to stderr
        #[clap(long)]
        progress: bool,

        /// Suppress all output except the formatted review result
        #[clap(long, short)]
        quiet: bool,

        /// Write formatted output to a file instead of stdout
        #[clap(long, value_name = "PATH")]
        output_file: Option<String>,

        /// Filter results by minimum severity (info, minor, major, critical)
        #[clap(long, value_parser = ["info", "minor", "major", "critical"])]
        severity: Option<String>,

        /// Disable review caching
        #[clap(long)]
        no_cache: bool,

        /// CI mode: skip diff size limit, exit 2 if any findings
        #[clap(long)]
        ci: bool,

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

        /// GitHub token for upload (default: `GITHUB_TOKEN` env var)
        #[clap(long, env = "GITHUB_TOKEN")]
        token: Option<String>,

        /// Recall project patterns from Uteke before review (requires `uteke` CLI)
        #[clap(long)]
        memory: bool,

        /// Save review findings to Uteke after review (implies --memory)
        #[clap(long)]
        learn: bool,
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

    /// Manage quality profiles (preset rule sets)
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
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

        /// GitHub token (default: `GITHUB_TOKEN` env var)
        #[clap(long, env = "GITHUB_TOKEN")]
        token: Option<String>,
    },

    /// Create a .cora.yaml config file and install pre-commit hook
    Init {
        /// Overwrite existing config file
        #[clap(long)]
        force: bool,

        /// Skip pre-commit hook installation
        #[clap(long)]
        no_hook: bool,
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

    /// Start MCP server (Model Context Protocol for AI agents)
    Mcp,

    /// Show tech debt report from review history
    Debt {
        /// Output as JSON
        #[clap(long)]
        json: bool,

        /// Show quality score trend graph
        #[clap(long)]
        trend: bool,

        /// Only show data since a date (YYYY-MM-DD) or git tag
        #[clap(long)]
        since: Option<String>,

        /// Filter by branch name
        #[clap(long)]
        branch: Option<String>,

        /// Output shields.io badge JSON
        #[clap(long)]
        badge: bool,

        /// Show estimated debt fix time
        #[clap(long)]
        estimate: bool,
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
    Login {
        /// Provider name (e.g. openai, zai, anthropic, ollama) — skips interactive selection
        #[clap(long)]
        provider: Option<String>,
        /// API key — skips interactive prompt
        #[clap(long)]
        api_key: Option<String>,
        /// Model name (e.g. glm-5.1, gpt-4o-mini) — used with --provider
        #[clap(long)]
        model: Option<String>,
        /// Base URL — used with --provider for custom endpoints
        #[clap(long)]
        base_url: Option<String>,
        /// Skip confirmation when overwriting existing key
        #[clap(long)]
        force: bool,
    },
    /// Check if an API key is configured
    Status,
    /// Remove the stored API key
    Remove,
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    /// Show the current resolved configuration
    Show {
        /// Show only global config (~/.cora/config.yaml)
        #[clap(long, conflicts_with = "project")]
        global: bool,
        /// Show only project config (.cora.yaml)
        #[clap(long, conflicts_with = "global")]
        project: bool,
    },
    /// Set a configuration value (keys: model, provider, `base_url`, format, severity)
    Set {
        /// Configuration key to set
        key: String,
        /// Value to assign
        value: String,
        /// Write to global config (~/.cora/config.yaml) instead of project .cora.yaml
        #[clap(long)]
        global: bool,
    },
    /// Validate the current configuration and report status
    Validate,
}

#[derive(Subcommand, Debug)]
enum ProfileAction {
    /// List available quality profiles
    List,
    /// Show details of a specific profile
    Show {
        /// Profile name (e.g. security-first, rust-strict)
        name: String,
    },
    /// Validate a custom profile YAML file
    Validate {
        /// Path to the profile YAML file
        path: String,
    },
}

/// Format bytes as human-readable string.
fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

#[allow(clippy::too_many_lines)]
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
        Command::Index {
            stats: show_stats,
            prune,
            rebuild,
            verbose,
        } => {
            let project_root = std::env::current_dir()?;
            let db_path = index::default_db_path(&project_root);

            if rebuild && db_path.exists() {
                std::fs::remove_file(&db_path)?;
                eprintln!("{}", "Dropped existing index.".dimmed());
            }

            let conn = index::open_index(&db_path)?;

            if show_stats {
                let summary = index::index_stats(&conn)?;
                println!("{}", "SYMBOL INDEX".cyan().bold());
                println!("{}", "────────────────────────────".dimmed());
                println!("  Total symbols:  {}", summary.total_symbols);
                println!("  Total files:    {}", summary.total_files);
                println!("  Database size:  {}", format_bytes(summary.db_size_bytes));
                println!();
                println!("  {}", "By Kind".cyan());
                for (kind, count) in &summary.symbols_by_kind {
                    println!("    {kind:<16} {count}");
                }
                println!();
                println!("  {}", "By Language".cyan());
                for (lang, count) in &summary.symbols_by_language {
                    println!("    {lang:<16} {count}");
                }
            } else if prune {
                let deleted = index::prune_deleted(&conn, &project_root)?;
                println!(
                    "{}",
                    format!("Pruned {deleted} deleted files from index.").green()
                );
            } else {
                eprintln!("{}", "🔍 Indexing project...".cyan());
                let stats =
                    index::index_project(&conn, &project_root, verbose || cli.global.verbose)?;
                eprintln!(
                    "{}",
                    format!(
                        "✅ Indexed {} symbols from {} files ({} skipped, {} errors)",
                        stats.symbols_indexed,
                        stats.files_indexed,
                        stats.files_skipped,
                        stats.errors
                    )
                    .green()
                );
                eprintln!("{}", format!("   Database: {}", db_path.display()).dimmed());
            }
            0
        }

        Command::Explore {
            query,
            kind,
            file,
            language,
            limit,
            json,
        } => {
            let project_root = std::env::current_dir()?;
            let db_path = index::default_db_path(&project_root);

            if !db_path.exists() {
                eprintln!("{}", "No index found. Run `cora index` first.".yellow());
                std::process::exit(1);
            }

            let conn = index::open_index(&db_path)?;

            let sym_kind = kind.as_deref().map(index::SymbolKind::from_str);

            let q = index::SymbolQuery {
                text: query,
                kind: sym_kind,
                file_prefix: file,
                language,
                limit,
            };

            let results = index::search(&conn, &q)?;

            if json {
                let json_results: Vec<serde_json::Value> = results
                    .iter()
                    .map(|r| {
                        serde_json::json!({
                            "name": r.symbol.name,
                            "kind": r.symbol.kind.as_str(),
                            "file": r.symbol.file,
                            "line": r.symbol.line,
                            "signature": r.symbol.signature,
                            "language": r.symbol.language,
                            "score": r.score,
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&json_results)?);
            } else if results.is_empty() {
                eprintln!("{}", "No symbols found.".yellow());
            } else {
                println!("{}", format!("Found {} symbols:", results.len()).cyan());
                println!(
                    "{}",
                    "───────────────────────────────────────────────".dimmed()
                );
                for r in &results {
                    println!(
                        "  {} {} {}:{}",
                        r.symbol.kind.as_str().blue(),
                        r.symbol.name.white().bold(),
                        r.symbol.file.dimmed(),
                        r.symbol.line
                    );
                    if !r.symbol.signature.is_empty() {
                        println!(
                            "    {} {}",
                            "→".dimmed(),
                            r.symbol.signature.trim().dimmed()
                        );
                    }
                }
            }
            0
        }

        Command::Commit {
            yolo,
            force,
            no_review,
            edit,
            stream,
            quiet,
        } => {
            let config = loader::load_config(
                cli.global.config.as_deref(),
                cli.global.provider.as_deref(),
                cli.global.model.as_deref(),
                cli.global.base_url.as_deref(),
                cli.global.format.as_deref(),
                cli.global.no_color,
            )?;
            let llm_config = loader::build_llm_config(&config, cli.global.api_key.as_deref())?;
            let opts = commit_cmd::CommitOptions {
                yolo,
                force,
                no_review,
                edit,
                stream,
                quiet,
            };
            commit_cmd::execute_commit(&config, &llm_config, &opts).await?
        }
        Command::Review {
            staged,
            unpushed,
            base,
            commit,
            diff_file,
            unstaged,
            max_diff_size,
            stream,
            no_auto_chunk,
            progress,
            quiet,
            output_file,
            severity,
            no_cache,
            ci,
            upload,
            repo,
            ref_name,
            token,
            memory,
            learn,
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
                    max_diff_size,
                    stream,
                    no_auto_chunk,
                    progress,
                    quiet,
                    output_file,
                    severity,
                    upload,
                    repo,
                    ref_name,
                    token,
                    no_cache,
                    ci,
                    memory,
                    learn,
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
        Command::Profile { action } => {
            match action {
                ProfileAction::List => profile::execute_profile_list()?,
                ProfileAction::Show { name } => profile::execute_profile_show(&name)?,
                ProfileAction::Validate { path } => profile::execute_profile_validate(&path)?,
            }
            0
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
        Command::Init { force, no_hook } => {
            if force {
                init::execute_init_force(no_hook)?;
            } else {
                init::execute_init(no_hook)?;
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
                AuthAction::Login {
                    provider,
                    api_key,
                    model,
                    base_url,
                    force,
                } => {
                    auth::execute_auth_login(
                        provider.as_deref(),
                        api_key.as_deref(),
                        model.as_deref(),
                        base_url.as_deref(),
                        force,
                    )?;
                }
                AuthAction::Status => auth::execute_auth_status()?,
                AuthAction::Remove => auth::execute_auth_remove()?,
            }
            0
        }
        Command::Config { action } => match action {
            ConfigAction::Show { global, project } => {
                config_cmd::execute_config_show(global, project)?;
                0
            }
            ConfigAction::Set { key, value, global } => {
                config_cmd::execute_config_set(&key, &value, global)?;
                0
            }
            ConfigAction::Validate => config_cmd::execute_config_validate()?,
        },
        Command::Providers => {
            providers::execute_providers();
            0
        }
        Command::Completion { shell } => {
            completion::execute_completion(&shell)?;
            0
        }
        Command::Mcp => {
            mcp::server::run_server()?;
            0
        }
        Command::Debt {
            json,
            trend,
            since,
            branch,
            badge,
            estimate,
        } => {
            let opts = debt::DebtOptions {
                json,
                trend,
                since,
                branch,
                badge,
                estimate,
            };
            debt::execute_debt(&opts)?
        }
    };

    std::process::exit(exit_code);
}

/// Struct to hold review options from CLI.
#[allow(clippy::struct_excessive_bools)]
struct ReviewOpts {
    staged: bool,
    unpushed: bool,
    base: Option<String>,
    commit: Option<String>,
    diff_file: Option<String>,
    unstaged: bool,
    max_diff_size: Option<usize>,
    stream: bool,
    no_auto_chunk: bool,
    progress: bool,
    quiet: bool,
    output_file: Option<String>,
    severity: Option<String>,
    upload: bool,
    repo: Option<String>,
    ref_name: Option<String>,
    token: Option<String>,
    no_cache: bool,
    ci: bool,
    memory: bool,
    learn: bool,
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

    let progress_reporter = if opts.progress {
        progress::ProgressReporter::new()
    } else {
        progress::ProgressReporter::disabled()
    };

    // Emit started event if progress enabled
    progress_reporter.started("review", opts.base.as_deref());

    let review_opts = review::ReviewOptions {
        staged: opts.staged,
        unpushed: opts.unpushed,
        base: opts.base.clone(),
        commit: opts.commit.clone(),
        diff_file: opts.diff_file.clone(),
        unstaged: opts.unstaged,
        max_diff_size: opts.max_diff_size,
        stream: opts.stream,
        quiet: opts.quiet || opts.progress,
        severity: opts.severity.clone(),
        no_cache: opts.no_cache,
        ci: opts.ci,
        auto_chunk: !opts.no_auto_chunk,
    };

    // When streaming and not quiet/progress, show a simpler message
    if opts.stream && !opts.quiet && !opts.progress {
        eprintln!(
            "{}",
            format!(
                "⏳ Streaming review from {} ({})…",
                llm_config.provider, llm_config.model
            )
            .dimmed()
        );
    }

    // Uteke memory integration: recall context before review
    let mut memory_backend = memory::MemoryBackend::default();
    let memory_level = if opts.learn {
        memory::MemoryLevel::Learning
    } else if opts.memory {
        memory::MemoryLevel::Context
    } else {
        memory::MemoryLevel::None
    };

    let memory_context = if memory_level != memory::MemoryLevel::None {
        memory_backend.detect();
        if memory_backend.is_available() {
            let project = repo_name_from_git().unwrap_or_else(|| "unknown".to_string());
            let memories = memory_backend.recall_context(&project);
            memory_backend.build_memory_context(&memories)
        } else {
            if !opts.quiet {
                eprintln!(
                    "{}",
                    "⚠ --memory flag set but uteke not found on PATH. Continuing without memory."
                        .yellow()
                );
            }
            String::new()
        }
    } else {
        String::new()
    };

    // Execute the review (returns formatted output)
    let result = review::execute_review(
        &config,
        &llm_config,
        &review_opts,
        effective_format,
        &progress_reporter,
        if !memory_context.is_empty() {
            Some(memory_context.as_str())
        } else {
            None
        },
    )
    .await?;

    // Show memory status if context was used
    if !memory_context.is_empty() && !opts.quiet {
        eprintln!("{}", "ℹ Review enriched with Uteke memory context.".cyan());
    }

    // Print the formatted output (to file if --output-file, else stdout)
    if let Some(ref path) = opts.output_file {
        std::fs::write(path, &result.output)
            .with_context(|| format!("failed to write output file: {path}"))?;
        if !opts.quiet {
            eprintln!("{}", format!("Output written to {path}").dimmed());
        }
    } else {
        print!("{}", result.output);
    }

    // If --upload, send the SARIF output to GitHub Code Scanning
    if opts.upload {
        let sarif_content = result.output;
        upload_sarif_content(&sarif_content, &opts.repo, &opts.ref_name, &opts.token).await?;
    }

    // Uteke memory integration: save findings after review
    if memory_level == memory::MemoryLevel::Learning && memory_backend.is_available() {
        let project = repo_name_from_git().unwrap_or_else(|| "unknown".to_string());
        memory_backend.save_findings(
            &project,
            result.total_issues,
            &result.severity_summary,
            &result.categories,
        );
        if !opts.quiet {
            eprintln!(
                "{}",
                format!("💾 Saved {} findings to Uteke memory.", result.total_issues).dimmed()
            );
        }
    }

    Ok(result.exit_code)
}

/// Extract repo name from git remote origin URL.
fn repo_name_from_git() -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
    // Extract owner/repo from https://github.com/owner/repo.git
    let parts: Vec<&str> = url.split('/').collect();
    if parts.len() >= 2 {
        let repo = parts.last()?.trim_end_matches(".git").to_string();
        let owner = parts.get(parts.len() - 2)?;
        Some(format!("{}/{}", owner, repo))
    } else {
        None
    }
}

/// Upload a SARIF string to GitHub Code Scanning.
#[allow(clippy::ref_option)]
async fn upload_sarif_content(
    sarif_content: &str,
    repo: &Option<String>,
    ref_name: &Option<String>,
    token: &Option<String>,
) -> Result<i32> {
    use std::io::Write;

    // Write SARIF to a temp file and upload it
    let tmp_dir = std::env::temp_dir();
    let tmp_path = tmp_dir.join(format!("cora-sarif-upload-{}.json", std::process::id()));

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
    let fmt_str = cli_format.map_or_else(
        || config.output.format.clone(),
        std::string::ToString::to_string,
    );
    OutputFormat::from_str_loose(&fmt_str)
}
