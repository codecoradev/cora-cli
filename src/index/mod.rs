//! Symbol index engine — persistent SQLite-backed symbol store.
//!
//! Build, query, and maintain a symbol index for code intelligence.
//! Uses regex-based extraction (same approach as `engine/context/extraction.rs`)
//! stored in SQLite with FTS5 for fast full-text search.

#[cfg(feature = "tree-sitter")]
mod ast;
pub mod brain;
mod extract;
pub mod graph;
pub mod schema;
mod symbols;
pub mod vector;

use std::collections::HashMap;
use std::path::Path;

use rusqlite::Connection;
use sha2::{Digest, Sha256};
use tracing::{debug, info};

#[allow(unused_imports)]
pub use graph::{CallEdge, CalleeResult, CallerResult, ImpactNode};
pub use symbols::{SearchResult, SymbolKind, SymbolQuery};

/// Open or create the **global** symbol index database.
///
/// All projects share a single SQLite database at `~/.codecora/cora-code/graph.db`.
/// Project isolation is handled via the `project_id` foreign key.
pub fn open_global_index() -> anyhow::Result<Connection> {
    crate::data_dir::ensure_data_dir()?;
    let db_path = crate::data_dir::graph_db_path();

    let conn = Connection::open(&db_path)?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
    schema::run_migrations(&conn)?;

    debug!("Opened global index at {}", db_path.display());
    Ok(conn)
}
/// Resolve the `project_id` for a given root path, creating the project row if needed.
pub fn ensure_project(conn: &Connection, root: &Path) -> anyhow::Result<i64> {
    let root_str = root.to_string_lossy().to_string();
    schema::get_or_create_project(conn, &root_str)
}

/// Index a single file: extract symbols and store in the database.
///
/// `project_id` is written to every row so data is scoped per-project
/// in the global database.
///
/// Returns the number of symbols indexed.
pub fn index_file(
    conn: &Connection,
    project_id: i64,
    file_path: &str,
    content: &str,
    language: &str,
) -> anyhow::Result<usize> {
    let fingerprint = file_fingerprint(content);
    let symbols = extract::extract_symbols(content, language, file_path);

    let tx = conn.unchecked_transaction()?;

    // Delete existing symbols for this file within this project
    tx.execute(
        "DELETE FROM symbols WHERE file = ?1 AND project_id = ?2",
        rusqlite::params![file_path, project_id],
    )?;

    // Upsert file fingerprint
    tx.execute(
        "INSERT INTO files (path, fingerprint, last_indexed, language, symbol_count, project_id)
         VALUES (?1, ?2, datetime('now'), ?3, ?4, ?5)
         ON CONFLICT(path) DO UPDATE SET
           fingerprint = excluded.fingerprint,
           last_indexed = excluded.last_indexed,
           language = excluded.language,
           symbol_count = excluded.symbol_count,
           project_id = excluded.project_id",
        rusqlite::params![
            file_path,
            fingerprint,
            language,
            symbols.len() as i64,
            project_id
        ],
    )?;

    // Insert symbols
    let mut count = 0;
    for sym in &symbols {
        tx.execute(
            "INSERT INTO symbols (name, kind, file, line, signature, language, project_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                sym.name,
                sym.kind.as_str(),
                sym.file,
                sym.line as i64,
                sym.signature,
                language,
                project_id,
            ],
        )?;
        count += 1;
    }

    // Extract and store call graph edges
    graph::clear_edges_for_file(&tx, file_path, project_id)?;
    let call_sites = extract::extract_calls(content, language, file_path);
    for site in &call_sites {
        tx.execute(
            "INSERT INTO call_graph (caller, callee, file, line, project_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![site.caller, site.callee, site.file, site.line as i64, project_id],
        )?;
    }

    #[cfg(feature = "tree-sitter")]
    {
        graph::clear_kg_edges_for_file(&tx, file_path, project_id)?;
        let kg_edges = extract::extract_edges(content, language, file_path);
        for e in &kg_edges {
            tx.execute(
                "INSERT INTO edges (source, kind, target, file, line, project_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![e.source, e.kind.as_str(), e.target, e.file, e.line as i64, project_id],
            )?;
        }
    }

    tx.commit()?;

    debug!(
        "Indexed {file_path}: {count} symbols, {} call edges ({language})",
        call_sites.len()
    );
    Ok(count)
}

/// Check if a file needs re-indexing based on content hash.
pub fn needs_reindex(conn: &Connection, project_id: i64, file_path: &str, content: &str) -> bool {
    let fingerprint = file_fingerprint(content);

    let stored: Option<String> = conn
        .query_row(
            "SELECT fingerprint FROM files WHERE path = ?1 AND project_id = ?2",
            rusqlite::params![file_path, project_id],
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
    let project_id = ensure_project(conn, root)?;
    index_project_with_id(conn, project_id, root, verbose)
}

/// Internal: index a project with an already-resolved `project_id`.
fn index_project_with_id(
    conn: &Connection,
    project_id: i64,
    root: &Path,
    verbose: bool,
) -> anyhow::Result<IndexStats> {
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

        if !needs_reindex(conn, project_id, &rel_str, &content) {
            stats.files_skipped += 1;
            continue;
        }

        stats.files_indexed += 1;
        match index_file(conn, project_id, &rel_str, &content, language) {
            Ok(n) => stats.symbols_indexed += n,
            Err(e) => {
                stats.errors += 1;
                if verbose {
                    eprintln!("  ⚠ Failed to index {rel_str}: {e}");
                }
            }
        }
    }

    // Update project's last_indexed timestamp
    conn.execute(
        "UPDATE projects SET last_indexed = datetime('now') WHERE id = ?1",
        rusqlite::params![project_id],
    )?;

    info!(
        "Index complete: {} files scanned, {} indexed, {} symbols, {} errors",
        stats.files_scanned, stats.files_indexed, stats.symbols_indexed, stats.errors
    );

    // Embed symbols into vector index for brain search
    match brain::embed_project(conn, project_id) {
        Ok(n) => {
            stats.embedded_symbols = Some(n);
            info!("Brain: embedded {n} symbols");
        }
        Err(e) => {
            if verbose {
                eprintln!("  ⚠ Embedding failed (non-fatal): {e}");
            }
            tracing::warn!("Embedding failed: {e}");
        }
    }

    Ok(stats)
}

/// Search the symbol index using FTS5 full-text search, scoped to a project.
pub fn search(
    conn: &Connection,
    project_id: i64,
    query: &SymbolQuery,
) -> anyhow::Result<Vec<SearchResult>> {
    symbols::search(conn, project_id, query)
}

/// Get index statistics for a specific project.
pub fn index_stats(conn: &Connection, project_id: i64) -> anyhow::Result<IndexSummary> {
    let total_symbols: i64 = conn.query_row(
        "SELECT COUNT(*) FROM symbols WHERE project_id = ?1",
        rusqlite::params![project_id],
        |row| row.get(0),
    )?;
    let total_files: i64 = conn.query_row(
        "SELECT COUNT(*) FROM files WHERE project_id = ?1",
        rusqlite::params![project_id],
        |row| row.get(0),
    )?;
    let db_size: i64 = {
        let page_size: i64 = conn
            .query_row("PRAGMA page_size", [], |row| row.get(0))
            .unwrap_or(4096);
        let page_count: i64 = conn
            .query_row("PRAGMA page_count", [], |row| row.get(0))
            .unwrap_or(0);
        page_size * page_count
    };

    let mut kind_counts: HashMap<String, usize> = HashMap::new();
    let mut stmt =
        conn.prepare("SELECT kind, COUNT(*) FROM symbols WHERE project_id = ?1 GROUP BY kind")?;
    let rows = stmt.query_map(rusqlite::params![project_id], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)? as usize))
    })?;
    for row in rows {
        let (kind, count) = row?;
        kind_counts.insert(kind, count);
    }

    let mut lang_counts: HashMap<String, usize> = HashMap::new();
    let mut stmt = conn.prepare(
        "SELECT language, COUNT(*) FROM symbols WHERE project_id = ?1 GROUP BY language",
    )?;
    let rows = stmt.query_map(rusqlite::params![project_id], |row| {
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

/// Remove symbols for files that no longer exist on disk, scoped to a project.
pub fn prune_deleted(conn: &Connection, project_id: i64, root: &Path) -> anyhow::Result<usize> {
    let mut deleted = 0;

    let mut stmt = conn.prepare("SELECT path FROM files WHERE project_id = ?1")?;
    let paths: Vec<String> = stmt
        .query_map(rusqlite::params![project_id], |row| row.get::<_, String>(0))?
        .filter_map(|r| r.ok())
        .collect();

    let to_prune: Vec<&String> = paths
        .iter()
        .filter(|path| !root.join(path).exists())
        .collect();

    if !to_prune.is_empty() {
        let tx = conn.unchecked_transaction()?;
        for path in &to_prune {
            tx.execute(
                "DELETE FROM symbols WHERE file = ?1 AND project_id = ?2",
                rusqlite::params![path, project_id],
            )?;
            tx.execute(
                "DELETE FROM call_graph WHERE file = ?1 AND project_id = ?2",
                rusqlite::params![path, project_id],
            )?;
            tx.execute(
                "DELETE FROM files WHERE path = ?1 AND project_id = ?2",
                rusqlite::params![path, project_id],
            )?;
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
    pub embedded_symbols: Option<usize>,
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

    /// Create a test project and return its project_id.
    fn test_project(conn: &Connection) -> i64 {
        schema::get_or_create_project(conn, "/tmp/test-project").unwrap()
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
        let project_id = test_project(&conn);
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
        let count = index_file(&conn, project_id, "src/cache.rs", code, "rs").unwrap();
        assert!(count > 0, "Should extract symbols from Rust code");
    }

    #[test]
    fn test_needs_reindex() {
        let conn = mem_conn();
        let project_id = test_project(&conn);
        let code = "fn hello() {}";

        // First time → needs reindex
        assert!(needs_reindex(&conn, project_id, "test.rs", code));

        // Index it
        index_file(&conn, project_id, "test.rs", code, "rs").unwrap();

        // Same content → no reindex needed
        assert!(!needs_reindex(&conn, project_id, "test.rs", code));

        // Changed content → needs reindex
        assert!(needs_reindex(&conn, project_id, "test.rs", "fn world() {}"));
    }

    #[test]
    fn test_search() {
        let conn = mem_conn();
        let project_id = test_project(&conn);
        let code = r#"
pub fn authenticate(token: &str) -> bool {
    false
}

pub struct AuthService {
    secret: String,
}
"#;
        index_file(&conn, project_id, "src/auth.rs", code, "rs").unwrap();

        let query = SymbolQuery::text("authenticate");
        let results = search(&conn, project_id, &query).unwrap();
        assert!(!results.is_empty());
        assert!(results[0].symbol.name.contains("authenticate"));
    }

    #[test]
    fn test_index_stats() {
        let conn = mem_conn();
        let project_id = test_project(&conn);
        index_file(&conn, project_id, "a.rs", "fn foo() {}", "rs").unwrap();
        index_file(&conn, project_id, "b.rs", "struct Bar {}", "rs").unwrap();

        let stats = index_stats(&conn, project_id).unwrap();
        assert!(stats.total_symbols >= 2);
        assert_eq!(stats.total_files, 2);
        assert!(stats.symbols_by_kind.contains_key("function"));
        assert!(stats.symbols_by_kind.contains_key("struct"));
    }

    #[test]
    fn test_prune_deleted() {
        let conn = mem_conn();
        let project_id = test_project(&conn);
        index_file(&conn, project_id, "gone.rs", "fn removed() {}", "rs").unwrap();

        // Create temp dir to use as root
        let tmp = tempfile::tempdir().unwrap();
        // gone.rs doesn't exist in temp dir → should be pruned
        let deleted = prune_deleted(&conn, project_id, tmp.path()).unwrap();
        assert_eq!(deleted, 1);

        let stats = index_stats(&conn, project_id).unwrap();
        assert_eq!(stats.total_symbols, 0);
    }

    #[test]
    fn test_reindex_replaces_symbols() {
        let conn = mem_conn();
        let project_id = test_project(&conn);
        index_file(&conn, project_id, "test.rs", "fn old_name() {}", "rs").unwrap();
        index_file(&conn, project_id, "test.rs", "fn new_name() {}", "rs").unwrap();

        let stats = index_stats(&conn, project_id).unwrap();
        // Should have 1 symbol (replaced, not 2)
        assert_eq!(stats.total_symbols, 1);
    }
}
