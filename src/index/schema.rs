//! SQLite schema management for the symbol index.

use rusqlite::Connection;

/// Current schema version.
const SCHEMA_VERSION: i32 = 1;

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

    conn.execute(
        "INSERT INTO schema_version (version) VALUES (?1)",
        rusqlite::params![SCHEMA_VERSION],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_creates_tables() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        run_migrations(&conn).unwrap();

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
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        // Running again should not error
        run_migrations(&conn).unwrap();
    }
}
