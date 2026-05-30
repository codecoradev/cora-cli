use std::path::Path;

use anyhow::{Context, Result};
use colored::Colorize;
use tracing::debug;

use crate::config::schema::Config;
use crate::engine::scanner::{batch_files, format_batch_for_prompt, walk_project};
use crate::formatters::{OutputFormat, formatter_for};

/// Scan command options.
pub struct ScanOptions {
    /// Root directory to scan.
    pub path: Option<String>,
    /// Include glob patterns.
    pub include: Vec<String>,
    /// Exclude glob patterns.
    pub exclude: Vec<String>,
    /// Additional file extensions to include.
    pub extensions: Vec<String>,
    /// Only scan files changed since last scan.
    pub incremental: bool,
}

/// Execute the scan command.
///
/// Walks the project directory, filters files, batches them, calls the LLM,
/// and formats the output.
pub async fn execute_scan(
    config: &Config,
    llm_config: &crate::engine::LLMConfig,
    opts: &ScanOptions,
    format: OutputFormat,
) -> Result<i32> {
    let root = match &opts.path {
        Some(p) => Path::new(p).to_path_buf(),
        None => std::env::current_dir()?,
    };

    if !root.is_dir() {
        anyhow::bail!("scan path '{}' is not a directory", root.display());
    }

    // Merge include/exclude with config ignore patterns
    let include = opts.include.clone();
    let mut exclude = config.ignore.files.clone();
    exclude.extend(opts.exclude.clone());

    debug!(root = %root.display(), "starting scan");

    // 1. Walk and collect files
    let mut files = walk_project(&root, &include, &exclude, &opts.extensions)?;

    // 1b. Incremental: filter out unchanged files
    if opts.incremental {
        let cache = ScanCache::load()?;
        let before_count = files.len();
        let root_abs = root.canonicalize().unwrap_or_else(|_| root.clone());
        files.retain(|f| {
            let abs_path = root_abs.join(&f.path);
            let hash = file_content_hash(&abs_path);
            match cache.get(&root_abs, &f.path) {
                Some(cached_hash) if cached_hash == hash => {
                    debug!(file = %f.path, "skipping unchanged file (incremental)");
                    false
                }
                _ => true,
            }
        });
        let skipped = before_count - files.len();
        if skipped > 0 {
            println!(
                "  {} skipped (unchanged since last scan)",
                skipped.to_string().dimmed()
            );
        }
    }

    if files.is_empty() {
        println!("{}", "No files to scan.".yellow());
        return Ok(0);
    }

    println!("🔍 {} files to review…", files.len().to_string().cyan(),);

    // 2. Calculate total lines
    let total_lines: usize = files.iter().map(|f| f.lines).sum();

    // 3. Batch files
    let batches = batch_files(&files, 60_000, 20);
    debug!(batches = batches.len(), "batched files");

    // 4. Process batches and collect issues
    let mut all_issues = Vec::new();
    let mut total_tokens = None;

    for (batch_idx, batch) in batches.iter().enumerate() {
        let files_content = format_batch_for_prompt(batch);
        let batch_label = if batches.len() > 1 {
            format!(" (batch {}/{})", batch_idx + 1, batches.len())
        } else {
            String::new()
        };

        println!("  Reviewing{}…", batch_label);

        let (issues, _summary, tokens) = crate::engine::llm::scan_files(
            llm_config,
            &files_content,
            &config.focus,
            &config.rules,
        )
        .await?;

        all_issues.extend(issues);
        if tokens.is_some() {
            total_tokens = tokens;
        }
    }

    // 5. Build response and format
    let issue_count = all_issues.len();
    let min_severity = config.hook.min_severity_level();
    let should_block = all_issues.iter().any(|i| i.severity >= min_severity);

    let response = crate::engine::ScanResponse {
        issues: all_issues,
        files_scanned: files.len(),
        lines_scanned: total_lines,
        summary: format!(
            "Scanned {} files ({} lines), found {} issues.",
            files.len(),
            total_lines,
            issue_count
        ),
        tokens_used: total_tokens,
        should_block,
    };

    let formatter = formatter_for(format);
    let output = formatter.format_scan(&response)?;
    println!("{}", output);

    // 6. Save scan cache for incremental mode
    if opts.incremental {
        let root_abs = root.canonicalize().unwrap_or_else(|_| root.clone());
        let mut cache = ScanCache::load().unwrap_or_default();
        for f in &files {
            let abs_path = root_abs.join(&f.path);
            let hash = file_content_hash(&abs_path);
            cache.set(&root_abs, &f.path, &hash);
        }
        cache.save()?;
        debug!(cached = files.len(), "saved scan cache");
    }

    if response.should_block && config.hook.mode == "block" {
        Ok(2)
    } else {
        Ok(0)
    }
}

/// Compute a short SHA256 hash of a file's content for incremental scanning.
fn file_content_hash(path: &std::path::Path) -> String {
    match std::fs::read(path) {
        Ok(bytes) => {
            use std::hash::{Hash, Hasher};
            // Use a fast non-crypto hash — we just need change detection, not security.
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            bytes.hash(&mut hasher);
            format!("{:016x}", hasher.finish())
        }
        Err(_) => String::new(),
    }
}

/// Cache of file content hashes for incremental scanning.
/// Stored as JSON in ~/.cora/scan-cache.json.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct ScanCache {
    /// Key: canonical root path, Value: { file_path: hash }
    projects: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
}

impl ScanCache {
    fn cache_path() -> anyhow::Result<std::path::PathBuf> {
        let home = dirs::home_dir().context("cannot determine home directory")?;
        Ok(home.join(".cora").join("scan-cache.json"))
    }

    fn load() -> Result<Self> {
        let path = Self::cache_path()?;
        if !path.is_file() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        serde_json::from_str(&content).context("failed to parse scan cache").map_err(Into::into)
    }

    fn save(&self) -> Result<()> {
        let path = Self::cache_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    fn get(&self, root: &std::path::Path, file: &str) -> Option<String> {
        let root_key = root.to_string_lossy().to_string();
        self.projects.get(&root_key)?.get(file).cloned()
    }

    fn set(&mut self, root: &std::path::Path, file: &str, hash: &str) {
        let root_key = root.to_string_lossy().to_string();
        self.projects
            .entry(root_key)
            .or_default()
            .insert(file.to_string(), hash.to_string());
    }
}
