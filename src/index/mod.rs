//! Symbol index engine — persistent SQLite-backed symbol store.
//!
//! Build, query, and maintain a symbol index for code intelligence.
//! Uses regex-based extraction (same approach as `engine/context/extraction.rs`)
//! stored in SQLite with FTS5 for fast full-text search.

mod extract;
pub mod graph;
mod schema;
mod symbols;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rusqlite::Connection;
use sha2::{Digest, Sha256};
use tracing::{debug, info};

#[allow(unused_imports)]
pub use graph::{CallEdge, CalleeResult, CallerResult, ImpactNode};
pub use symbols::{SearchResult, SymbolKind, SymbolQuery};

/// Default index database path relative to project root.
pub const INDEX_DB_NAME: &str = ".cora/index.db";

/// Open or create the symbol index database.
pub fn open_index(db_path: &Path) -> anyhow::Result<Connection> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(db_path)?;

    // Enable WAL mode for better concurrent read performance
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

    schema::run_migrations(&conn)?;

    debug!("Opened index at {}", db_path.display());
    Ok(conn)
}

/// Resolve the default index database path for a project.
pub fn default_db_path(project_root: &Path) -> PathBuf {
    project_root.join(INDEX_DB_NAME)
}

/// Index a single file: extract symbols and store in the database.
///
/// Returns the number of symbols indexed.
pub fn index_file(
    conn: &Connection,
    file_path: &str,
    content: &str,
    language: &str,
) -> anyhow::Result<usize> {
    let fingerprint = file_fingerprint(content);
    let symbols = extract::extract_symbols(content, language, file_path);

    // Begin transaction
    let tx = conn.unchecked_transaction()?;

    // Delete existing symbols for this file
    tx.execute(
        "DELETE FROM symbols WHERE file = ?1",
        rusqlite::params![file_path],
    )?;

    // Update file fingerprint
    tx.execute(
        "INSERT OR REPLACE INTO files (path, fingerprint, last_indexed, language, symbol_count)
         VALUES (?1, ?2, datetime('now'), ?3, ?4)",
        rusqlite::params![file_path, fingerprint, language, symbols.len() as i64],
    )?;

    // Insert symbols
    let mut count = 0;
    for sym in &symbols {
        tx.execute(
            "INSERT INTO symbols (name, kind, file, line, signature, language)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                sym.name,
                sym.kind.as_str(),
                sym.file,
                sym.line as i64,
                sym.signature,
                language,
            ],
        )?;
        count += 1;
    }

    // Extract and store call graph edges
    graph::clear_edges_for_file(&tx, file_path)?;
    let call_sites = extract::extract_calls(content, language, file_path);
    for site in &call_sites {
        tx.execute(
            "INSERT INTO call_graph (caller, callee, file, line) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![site.caller, site.callee, site.file, site.line as i64],
        )?;
    }

    tx.commit()?;

    debug!(
        "Indexed {file_path}: {count} symbols, {} edges ({language})",
        call_sites.len()
    );
    Ok(count)
}

/// Check if a file needs re-indexing based on content hash.
pub fn needs_reindex(conn: &Connection, file_path: &str, content: &str) -> bool {
    let fingerprint = file_fingerprint(content);

    let stored: Option<String> = conn
        .query_row(
            "SELECT fingerprint FROM files WHERE path = ?1",
            rusqlite::params![file_path],
            |row| row.get(0),
        )
        .ok();

    match stored {
        Some(fp) => fp != fingerprint,
        None => true,
    }
}

/// Index a project directory, respecting .gitignore.
///
/// Returns summary stats.
pub fn index_project(conn: &Connection, root: &Path, verbose: bool) -> anyhow::Result<IndexStats> {
    let mut stats = IndexStats::default();

    let walker = ignore::WalkBuilder::new(root)
        .hidden(true)
        .git_ignore(true)
        .git_exclude(true)
        .build();

    for entry in walker {
        let entry = entry?;
        if !entry.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }

        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap_or(path);
        let rel_str = rel.to_string_lossy().to_string();

        let language = crate::engine::diff_parser::detect_language(&rel_str);
        if language == "unknown" || language == "text" {
            continue;
        }

        stats.files_scanned += 1;

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if !needs_reindex(conn, &rel_str, &content) {
            stats.files_skipped += 1;
            continue;
        }

        stats.files_indexed += 1;
        match index_file(conn, &rel_str, &content, language) {
            Ok(n) => stats.symbols_indexed += n,
            Err(e) => {
                stats.errors += 1;
                if verbose {
                    eprintln!("  ⚠ Failed to index {rel_str}: {e}");
                }
            }
        }
    }

    info!(
        "Index complete: {} files scanned, {} indexed, {} symbols, {} errors",
        stats.files_scanned, stats.files_indexed, stats.symbols_indexed, stats.errors
    );

    Ok(stats)
}

/// Search the symbol index using FTS5 full-text search.
pub fn search(conn: &Connection, query: &SymbolQuery) -> anyhow::Result<Vec<SearchResult>> {
    symbols::search(conn, query)
}

/// Get index statistics.
pub fn index_stats(conn: &Connection) -> anyhow::Result<IndexSummary> {
    let total_symbols: i64 =
        conn.query_row("SELECT COUNT(*) FROM symbols", [], |row| row.get(0))?;
    let total_files: i64 = conn.query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))?;
    let db_size: i64 = conn
        .query_row("PRAGMA page_count", [], |row| row.get(0))
        .unwrap_or(0)
        * 4096; // page_size default

    // Symbols by kind
    let mut kind_counts: HashMap<String, usize> = HashMap::new();
    let mut stmt = conn.prepare("SELECT kind, COUNT(*) FROM symbols GROUP BY kind")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)? as usize))
    })?;
    for row in rows {
        let (kind, count) = row?;
        kind_counts.insert(kind, count);
    }

    // Symbols by language
    let mut lang_counts: HashMap<String, usize> = HashMap::new();
    let mut stmt = conn.prepare("SELECT language, COUNT(*) FROM symbols GROUP BY language")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)? as usize))
    })?;
    for row in rows {
        let (lang, count) = row?;
        lang_counts.insert(lang, count);
    }

    Ok(IndexSummary {
        total_symbols: total_symbols as usize,
        total_files: total_files as usize,
        db_size_bytes: db_size as u64,
        symbols_by_kind: kind_counts,
        symbols_by_language: lang_counts,
    })
}

/// Remove symbols for files that no longer exist on disk.
pub fn prune_deleted(conn: &Connection, root: &Path) -> anyhow::Result<usize> {
    let mut deleted = 0;

    let mut stmt = conn.prepare("SELECT path FROM files")?;
    let paths: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(0))?
        .filter_map(|r| r.ok())
        .collect();

    // Batch all deletes in a single transaction instead of per-file
    let to_prune: Vec<&String> = paths
        .iter()
        .filter(|path| !root.join(path).exists())
        .collect();

    if !to_prune.is_empty() {
        let tx = conn.unchecked_transaction()?;
        for path in &to_prune {
            tx.execute(
                "DELETE FROM symbols WHERE file = ?1",
                rusqlite::params![path],
            )?;
            tx.execute("DELETE FROM files WHERE path = ?1", rusqlite::params![path])?;
        }
        tx.commit()?;
        deleted = to_prune.len();
    }

    if deleted > 0 {
        info!("Pruned {deleted} deleted files from index");
    }

    Ok(deleted)
}

/// Compute a SHA-256 fingerprint for file content.
fn file_fingerprint(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Statistics from an index build run.
#[derive(Debug, Clone, Default)]
pub struct IndexStats {
    pub files_scanned: usize,
    pub files_indexed: usize,
    pub files_skipped: usize,
    pub symbols_indexed: usize,
    pub errors: usize,
}

/// Summary of the current index state.
#[derive(Debug, Clone)]
pub struct IndexSummary {
    pub total_symbols: usize,
    pub total_files: usize,
    pub db_size_bytes: u64,
    pub symbols_by_kind: HashMap<String, usize>,
    pub symbols_by_language: HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mem_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        schema::run_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_open_and_migrate() {
        let conn = mem_conn();
        // Tables should exist
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM symbols", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_index_rust_file() {
        let conn = mem_conn();
        let code = r#"
use std::collections::HashMap;

pub struct Cache {
    inner: HashMap<String, String>,
}

impl Cache {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(key)
    }
}
"#;
        let count = index_file(&conn, "src/cache.rs", code, "rs").unwrap();
        assert!(count > 0, "Should extract symbols from Rust code");
    }

    #[test]
    fn test_needs_reindex() {
        let conn = mem_conn();
        let code = "fn hello() {}";

        // First time → needs reindex
        assert!(needs_reindex(&conn, "test.rs", code));

        // Index it
        index_file(&conn, "test.rs", code, "rs").unwrap();

        // Same content → no reindex needed
        assert!(!needs_reindex(&conn, "test.rs", code));

        // Changed content → needs reindex
        assert!(needs_reindex(&conn, "test.rs", "fn world() {}"));
    }

    #[test]
    fn test_search() {
        let conn = mem_conn();
        let code = r#"
pub fn authenticate(token: &str) -> bool {
    false
}

pub struct AuthService {
    secret: String,
}
"#;
        index_file(&conn, "src/auth.rs", code, "rs").unwrap();

        let query = SymbolQuery::text("authenticate");
        let results = search(&conn, &query).unwrap();
        assert!(!results.is_empty());
        assert!(results[0].symbol.name.contains("authenticate"));
    }

    #[test]
    fn test_index_stats() {
        let conn = mem_conn();
        index_file(&conn, "a.rs", "fn foo() {}", "rs").unwrap();
        index_file(&conn, "b.rs", "struct Bar {}", "rs").unwrap();

        let stats = index_stats(&conn).unwrap();
        assert!(stats.total_symbols >= 2);
        assert_eq!(stats.total_files, 2);
        assert!(stats.symbols_by_kind.contains_key("function"));
        assert!(stats.symbols_by_kind.contains_key("struct"));
    }

    #[test]
    fn test_prune_deleted() {
        let conn = mem_conn();
        index_file(&conn, "gone.rs", "fn removed() {}", "rs").unwrap();

        // Create temp dir to use as root
        let tmp = tempfile::tempdir().unwrap();
        // gone.rs doesn't exist in temp dir → should be pruned
        let deleted = prune_deleted(&conn, tmp.path()).unwrap();
        assert_eq!(deleted, 1);

        let stats = index_stats(&conn).unwrap();
        assert_eq!(stats.total_symbols, 0);
    }

    #[test]
    fn test_reindex_replaces_symbols() {
        let conn = mem_conn();
        index_file(&conn, "test.rs", "fn old_name() {}", "rs").unwrap();
        index_file(&conn, "test.rs", "fn new_name() {}", "rs").unwrap();

        let stats = index_stats(&conn).unwrap();
        // Should have 1 symbol (replaced, not 2)
        assert_eq!(stats.total_symbols, 1);
    }
}
