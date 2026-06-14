//! Call graph traversal for `cora callers` and `cora impact`.
//!
//! Uses the existing `engine/context/extraction.rs` symbol reference extraction
//! to build call edges at index time, then traverse them for queries.

use rusqlite::Connection;

#[allow(unused_imports)]
use super::symbols::{IndexedSymbol, SymbolKind};

/// A directed edge in the call graph: caller → callee.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CallEdge {
    /// Symbol that makes the call.
    pub caller: String,
    /// Symbol that is called.
    pub callee: String,
    /// File where the call happens.
    pub file: String,
    /// Line number of the call site.
    pub line: u32,
}

/// Store call edges in the database.
#[allow(dead_code)]
pub fn store_edges(conn: &Connection, edges: &[CallEdge]) -> anyhow::Result<usize> {
    let tx = conn.unchecked_transaction()?;
    let mut count = 0;
    for edge in edges {
        tx.execute(
            "INSERT INTO call_graph (caller, callee, file, line) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![edge.caller, edge.callee, edge.file, edge.line as i64],
        )?;
        count += 1;
    }
    tx.commit()?;
    Ok(count)
}

/// Clear call graph edges for a specific file (before re-indexing).
#[allow(dead_code)]
pub fn clear_edges_for_file(conn: &Connection, file: &str) -> anyhow::Result<()> {
    conn.execute(
        "DELETE FROM call_graph WHERE file = ?1",
        rusqlite::params![file],
    )?;
    Ok(())
}

/// Find all callers of a symbol (who calls this?).
///
/// Returns symbols that call the given name, grouped by file.
pub fn find_callers(
    conn: &Connection,
    symbol_name: &str,
    limit: usize,
) -> anyhow::Result<Vec<CallerResult>> {
    let pattern = format!("%{symbol_name}%");

    let mut stmt = conn.prepare(
        "SELECT DISTINCT cg.caller, cg.file, cg.line
         FROM call_graph cg
         WHERE cg.callee LIKE ?1
         LIMIT ?2",
    )?;

    let rows = stmt.query_map(rusqlite::params![pattern, limit as i64], |row| {
        Ok(CallerResult {
            caller: row.get(0)?,
            file: row.get(1)?,
            line: row.get::<_, i64>(2)? as u32,
        })
    })?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

/// Find all callees of a symbol (what does this call?).
///
/// Returns symbols that are called by the given name.
#[allow(dead_code)]
pub fn find_callees(
    conn: &Connection,
    symbol_name: &str,
    limit: usize,
) -> anyhow::Result<Vec<CalleeResult>> {
    let pattern = format!("%{symbol_name}%");

    let mut stmt = conn.prepare(
        "SELECT DISTINCT cg.callee, cg.file, cg.line
         FROM call_graph cg
         WHERE cg.caller LIKE ?1
         LIMIT ?2",
    )?;

    let rows = stmt.query_map(rusqlite::params![pattern, limit as i64], |row| {
        Ok(CalleeResult {
            callee: row.get(0)?,
            file: row.get(1)?,
            line: row.get::<_, i64>(2)? as u32,
        })
    })?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

/// Impact analysis: what breaks if a symbol changes?
///
/// Uses reverse traversal: find all callers recursively up to `depth`.
pub fn impact_analysis(
    conn: &Connection,
    symbol_name: &str,
    depth: u32,
) -> anyhow::Result<Vec<ImpactNode>> {
    let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut result = Vec::new();
    let mut current_level = vec![symbol_name.to_string()];
    let mut current_depth = 0u32;

    while current_depth < depth && !current_level.is_empty() {
        let mut next_level = Vec::new();

        for sym in &current_level {
            if !visited.insert(sym.clone()) {
                continue;
            }

            let callers = find_callers(conn, sym, 100)?;
            for caller in callers {
                let node = ImpactNode {
                    symbol: caller.caller.clone(),
                    file: caller.file.clone(),
                    line: caller.line,
                    depth: current_depth + 1,
                };

                if !visited.contains(&caller.caller) {
                    next_level.push(caller.caller.clone());
                }

                result.push(node);
            }
        }

        current_level = next_level;
        current_depth += 1;
    }

    // Sort by depth then file
    result.sort_by(|a, b| {
        a.depth
            .cmp(&b.depth)
            .then_with(|| a.file.cmp(&b.file))
            .then_with(|| a.line.cmp(&b.line))
    });

    Ok(result)
}

/// A caller result entry.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CallerResult {
    pub caller: String,
    pub file: String,
    pub line: u32,
}

/// A callee result entry.
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize)]
pub struct CalleeResult {
    pub callee: String,
    pub file: String,
    pub line: u32,
}

/// An impact analysis node.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ImpactNode {
    pub symbol: String,
    pub file: String,
    pub line: u32,
    pub depth: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mem_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        super::super::schema::run_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_store_and_find_callers() {
        let conn = mem_conn();

        let edges = vec![
            CallEdge {
                caller: "main".to_string(),
                callee: "authenticate".to_string(),
                file: "main.rs".to_string(),
                line: 10,
            },
            CallEdge {
                caller: "handler".to_string(),
                callee: "authenticate".to_string(),
                file: "handler.rs".to_string(),
                line: 25,
            },
        ];
        store_edges(&conn, &edges).unwrap();

        let callers = find_callers(&conn, "authenticate", 10).unwrap();
        assert_eq!(callers.len(), 2);
        let names: Vec<&str> = callers.iter().map(|c| c.caller.as_str()).collect();
        assert!(names.contains(&"main"));
        assert!(names.contains(&"handler"));
    }

    #[test]
    fn test_find_callees() {
        let conn = mem_conn();

        let edges = vec![
            CallEdge {
                caller: "main".to_string(),
                callee: "init".to_string(),
                file: "main.rs".to_string(),
                line: 5,
            },
            CallEdge {
                caller: "main".to_string(),
                callee: "run".to_string(),
                file: "main.rs".to_string(),
                line: 10,
            },
        ];
        store_edges(&conn, &edges).unwrap();

        let callees = find_callees(&conn, "main", 10).unwrap();
        assert_eq!(callees.len(), 2);
    }

    #[test]
    fn test_clear_edges_for_file() {
        let conn = mem_conn();
        store_edges(
            &conn,
            &[CallEdge {
                caller: "a".to_string(),
                callee: "b".to_string(),
                file: "test.rs".to_string(),
                line: 1,
            }],
        )
        .unwrap();

        clear_edges_for_file(&conn, "test.rs").unwrap();
        let callers = find_callers(&conn, "b", 10).unwrap();
        assert!(callers.is_empty());
    }

    #[test]
    fn test_impact_analysis_depth() {
        let conn = mem_conn();
        // a → b → c
        // If c changes, impact should find b (depth 1) and a (depth 2)
        let edges = vec![
            CallEdge {
                caller: "b".to_string(),
                callee: "c".to_string(),
                file: "b.rs".to_string(),
                line: 1,
            },
            CallEdge {
                caller: "a".to_string(),
                callee: "b".to_string(),
                file: "a.rs".to_string(),
                line: 1,
            },
        ];
        store_edges(&conn, &edges).unwrap();

        let impact = impact_analysis(&conn, "c", 3).unwrap();
        // Should find b at depth 1, a at depth 2
        assert!(impact.iter().any(|n| n.symbol == "b" && n.depth == 1));
        assert!(impact.iter().any(|n| n.symbol == "a" && n.depth == 2));
    }
}
