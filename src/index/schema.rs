//! SQLite schema management for the symbol index.

use rusqlite::Connection;

/// Current schema version.
#[allow(dead_code)]
const SCHEMA_VERSION: i32 = 2;

/// Run database migrations (creates tables if not exist).
pub fn run_migrations(conn: &Connection) -> anyhow::Result<()> {
    // Schema version tracking
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT DEFAULT (datetime('now'))
        );",
    )?;

    let current: i32 = conn
        .query_row("SELECT MAX(version) FROM schema_version", [], |row| {
            row.get(0)
        })
        .unwrap_or(0);

    if current < 1 {
        migrate_v1(conn)?;
    }
    if current < 2 {
        migrate_v2(conn)?;
    }

    Ok(())
}

/// Migration v1: Initial schema — symbols, files, FTS5 index.
fn migrate_v1(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch(
        "
        -- Symbol definitions extracted from source files
        CREATE TABLE IF NOT EXISTS symbols (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            kind        TEXT NOT NULL,
            file        TEXT NOT NULL,
            line        INTEGER NOT NULL,
            signature   TEXT NOT NULL DEFAULT '',
            language    TEXT NOT NULL DEFAULT 'unknown',
            created_at  TEXT DEFAULT (datetime('now'))
        );

        -- Index for file-based queries
        CREATE INDEX IF NOT EXISTS idx_symbols_file ON symbols(file);

        -- Index for name-based lookups
        CREATE INDEX IF NOT EXISTS idx_symbols_name ON symbols(name);

        -- Index for kind-based filtering
        CREATE INDEX IF NOT EXISTS idx_symbols_kind ON symbols(kind);

        -- File tracking for incremental indexing
        CREATE TABLE IF NOT EXISTS files (
            path          TEXT PRIMARY KEY,
            fingerprint   TEXT NOT NULL,
            last_indexed  TEXT NOT NULL,
            language      TEXT NOT NULL DEFAULT 'unknown',
            symbol_count  INTEGER NOT NULL DEFAULT 0
        );

        -- FTS5 virtual table for full-text search on symbol names
        CREATE VIRTUAL TABLE IF NOT EXISTS symbols_fts USING fts5(
            name,
            signature,
            content='symbols',
            content_rowid='id',
            tokenize='unicode61 remove_diacritics 1'
        );

        -- Call graph edges: caller → callee relationships
        CREATE TABLE IF NOT EXISTS call_graph (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            caller      TEXT NOT NULL,
            callee      TEXT NOT NULL,
            file        TEXT NOT NULL,
            line        INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_cg_caller ON call_graph(caller);
        CREATE INDEX IF NOT EXISTS idx_cg_callee ON call_graph(callee);
        CREATE INDEX IF NOT EXISTS idx_cg_file ON call_graph(file);

        -- Triggers to keep FTS5 in sync with symbols table
        CREATE TRIGGER IF NOT EXISTS symbols_fts_insert
        AFTER INSERT ON symbols
        BEGIN
            INSERT INTO symbols_fts(rowid, name, signature)
            VALUES (new.id, new.name, new.signature);
        END;

        CREATE TRIGGER IF NOT EXISTS symbols_fts_delete
        AFTER DELETE ON symbols
        BEGIN
            INSERT INTO symbols_fts(symbols_fts, rowid, name, signature)
            VALUES ('delete', old.id, old.name, old.signature);
        END;

        CREATE TRIGGER IF NOT EXISTS symbols_fts_update
        AFTER UPDATE ON symbols
        BEGIN
            INSERT INTO symbols_fts(symbols_fts, rowid, name, signature)
            VALUES ('delete', old.id, old.name, old.signature);
            INSERT INTO symbols_fts(rowid, name, signature)
            VALUES (new.id, new.name, new.signature);
        END;
        ",
    )?;

    conn.execute("INSERT INTO schema_version (version) VALUES (1)", [])?;

    Ok(())
}

/// Migration v2: Multi-project support.
///
/// Adds `projects` table and `project_id` column to `symbols`, `files`,
/// and `call_graph`. The global DB at `~/.codecora/cora-code/graph.db`
/// stores data for all indexed projects, keyed by absolute path.
fn migrate_v2(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch(
        "
        -- Projects table: one row per indexed codebase
        CREATE TABLE IF NOT EXISTS projects (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            root_path     TEXT NOT NULL UNIQUE,
            name          TEXT NOT NULL DEFAULT '',
            last_indexed  TEXT NOT NULL DEFAULT (datetime('now')),
            created_at    TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- Add project_id to symbols (nullable for migration compat)
        ALTER TABLE symbols ADD COLUMN project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE;
        CREATE INDEX IF NOT EXISTS idx_symbols_project ON symbols(project_id);

        -- Add project_id to files (nullable for migration compat)
        ALTER TABLE files ADD COLUMN project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE;
        CREATE INDEX IF NOT EXISTS idx_files_project ON files(project_id);

        -- Add project_id to call_graph (nullable for migration compat)
        ALTER TABLE call_graph ADD COLUMN project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE;
        CREATE INDEX IF NOT EXISTS idx_cg_project ON call_graph(project_id);
        ",
    )?;

    conn.execute("INSERT INTO schema_version (version) VALUES (2)", [])?;

    Ok(())
}

/// Get or create a project entry by root path.
///
/// Returns the project ID.
pub fn get_or_create_project(conn: &Connection, root_path: &str) -> anyhow::Result<i64> {
    // Try to find existing project
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM projects WHERE root_path = ?1",
            rusqlite::params![root_path],
            |row| row.get(0),
        )
        .ok();

    if let Some(id) = existing {
        return Ok(id);
    }

    // Extract project name from directory name
    let name = std::path::Path::new(root_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    conn.execute(
        "INSERT INTO projects (root_path, name) VALUES (?1, ?2)",
        rusqlite::params![root_path, name],
    )?;

    Ok(conn.last_insert_rowid())
}

/// Remove a project and all its associated data (CASCADE).
///
/// Returns the number of rows deleted.
pub fn delete_project(conn: &Connection, project_id: i64) -> anyhow::Result<usize> {
    let affected = conn.execute(
        "DELETE FROM projects WHERE id = ?1",
        rusqlite::params![project_id],
    )?;
    Ok(affected)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mem_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        run_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_migration_creates_tables() {
        let conn = mem_conn();

        // Check symbols table
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM symbols", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);

        // Check files table
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);

        // Check FTS table exists
        conn.query_row("SELECT COUNT(*) FROM symbols_fts", [], |row| {
            row.get::<_, i64>(0)
        })
        .unwrap();

        // Check projects table
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);

        // Check schema version
        let version: i32 = conn
            .query_row("SELECT MAX(version) FROM schema_version", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(version, SCHEMA_VERSION);
    }

    #[test]
    fn test_migration_idempotent() {
        let conn = mem_conn();
        // Running again should not error
        run_migrations(&conn).unwrap();
    }

    #[test]
    fn test_v2_project_columns_exist() {
        let conn = mem_conn();

        // Verify project_id column exists on symbols
        let _: i64 = conn
            .query_row("SELECT project_id FROM symbols LIMIT 0", [], |row| {
                row.get(0)
            })
            .unwrap_or(0);

        // Verify project_id column exists on files
        let _: i64 = conn
            .query_row("SELECT project_id FROM files LIMIT 0", [], |row| row.get(0))
            .unwrap_or(0);

        // Verify project_id column exists on call_graph
        let _: i64 = conn
            .query_row("SELECT project_id FROM call_graph LIMIT 0", [], |row| {
                row.get(0)
            })
            .unwrap_or(0);
    }

    #[test]
    fn test_get_or_create_project() {
        let conn = mem_conn();

        // Create project
        let id1 = get_or_create_project(&conn, "/home/user/myproject").unwrap();
        assert!(id1 > 0);

        // Same path returns same id
        let id2 = get_or_create_project(&conn, "/home/user/myproject").unwrap();
        assert_eq!(id1, id2);

        // Different path returns different id
        let id3 = get_or_create_project(&conn, "/home/user/other").unwrap();
        assert_ne!(id1, id3);

        // Check name extraction
        let name: String = conn
            .query_row(
                "SELECT name FROM projects WHERE id = ?1",
                rusqlite::params![id1],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(name, "myproject");
    }

    #[test]
    fn test_delete_project_cascades() {
        let conn = mem_conn();

        let pid = get_or_create_project(&conn, "/tmp/testproj").unwrap();

        // Insert symbol and file linked to project
        conn.execute(
            "INSERT INTO symbols (name, kind, file, line, project_id) VALUES ('test_fn', 'function', 'main.rs', 1, ?1)",
            rusqlite::params![pid],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO files (path, fingerprint, last_indexed, project_id) VALUES ('main.rs', 'abc', datetime('now'), ?1)",
            rusqlite::params![pid],
        )
        .unwrap();

        // Delete project
        delete_project(&conn, pid).unwrap();

        // Symbols and files should be cascade-deleted
        let sym_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM symbols", [], |row| row.get(0))
            .unwrap();
        assert_eq!(sym_count, 0);

        let file_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))
            .unwrap();
        assert_eq!(file_count, 0);

        let proj_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))
            .unwrap();
        assert_eq!(proj_count, 0);
    }
}
