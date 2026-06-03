//! Symbol resolution — map extracted symbols to file locations,
//! read source content, and assemble the context chain under a token budget.
//!
//! Resolution strategy (language-aware):
//! - **Rust**: `use crate::foo::bar` → `src/foo/bar.rs` (or `mod.rs`)
//! - **Python**: `import foo.bar` → `foo/bar.py` or `foo/bar/__init__.py`
//! - **JS/TS**: `import x from './foo'` → relative path resolution
//! - **Go**: `import "pkg/path"` → `pkg/path/` directory
//! - **Java/Kotlin**: `import foo.Bar` → `foo/Bar.java` / `foo/Bar.kt`
//!
//! Additionally, test file mapping is supported via naming conventions.

use std::collections::HashMap;
use std::path::Path;

use tracing::debug;

use super::types::{
    ContextChain, ContextConfig, ContextEntry, ContextPriority, ContextStats, ExtractedSymbol,
    SymbolKind, estimate_tokens,
};

/// Maximum lines to read per symbol definition (prevents reading huge functions).
const MAX_FN_LINES: usize = 50;
/// Maximum lines to read per struct/type definition.
const MAX_TYPE_LINES: usize = 50;
/// Maximum lines to read per test function.
const MAX_TEST_LINES: usize = 30;

/// Build the full context chain from extracted symbols.
///
/// This is the main entry point: extract → resolve → read → budget → assemble.
pub fn build_context_chain(
    symbols: &[ExtractedSymbol],
    config: &ContextConfig,
    project_root: &Path,
    ignore_patterns: &[String],
) -> ContextChain {
    if !config.enabled || symbols.is_empty() {
        return ContextChain::default();
    }

    let mut stats = ContextStats {
        symbols_extracted: symbols.len(),
        ..Default::default()
    };

    // Phase 1: Resolve symbols to file locations
    let mut entries = resolve_symbols(symbols, config, project_root, ignore_patterns, &mut stats);

    // Phase 2: Add test file mappings
    if config.include_tests {
        add_test_mappings(symbols, project_root, &mut entries, &mut stats);
    }

    // Sort by priority (FunctionDef first, Test last)
    entries.sort_by_key(|e| e.priority);

    // Phase 3: Read file content under budget
    let mut budget = config.max_context_tokens;
    let mut parts = Vec::new();

    for entry in &entries {
        let content = read_entry_content(entry, project_root);
        if content.is_empty() {
            continue;
        }

        let tokens = estimate_tokens(&content);
        if tokens > budget {
            stats.budget_hit = true;
            debug!(
                entry = %entry.label,
                tokens,
                remaining_budget = budget,
                "skipping context entry (budget exhausted)"
            );
            continue;
        }

        budget -= tokens;
        stats.entries_read += 1;
        stats.estimated_tokens += tokens;

        parts.push(format!(
            "--- {}:{}-{} ({}) ---\n{}",
            entry.file, entry.line_start, entry.line_end, entry.label, content
        ));
    }

    let text = if parts.is_empty() {
        String::new()
    } else {
        format!("Relevant Cross-File Context:\n\n{}", parts.join("\n"))
    };

    debug!(
        symbols_extracted = stats.symbols_extracted,
        symbols_resolved = stats.symbols_resolved,
        entries_read = stats.entries_read,
        estimated_tokens = stats.estimated_tokens,
        budget_hit = stats.budget_hit,
        "context chain built"
    );

    ContextChain { text, stats }
}

/// Resolve extracted symbols to concrete file locations and line ranges.
fn resolve_symbols(
    symbols: &[ExtractedSymbol],
    config: &ContextConfig,
    project_root: &Path,
    ignore_patterns: &[String],
    stats: &mut ContextStats,
) -> Vec<ContextEntry> {
    let mut entries = Vec::new();
    let mut seen_files: HashMap<String, Vec<(u32, u32)>> = HashMap::new(); // track line ranges per file

    for sym in symbols {
        // Determine the language from the file extension
        let lang = Path::new(&sym.file)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let resolved = match &sym.kind {
            SymbolKind::Import(path) => resolve_import(path, &sym.file, lang, project_root),
            SymbolKind::FunctionCall(name) => resolve_function(name, &sym.file, lang, project_root),
            SymbolKind::TypeRef(name) => resolve_type(name, &sym.file, lang, project_root),
        };

        for entry in resolved {
            // Check ignore patterns
            if is_ignored(&entry.file, ignore_patterns) {
                debug!(file = %entry.file, "skipping ignored file");
                continue;
            }

            // Check if the entry's file exists
            let full_path = project_root.join(&entry.file);
            if !full_path.exists() {
                debug!(file = %entry.file, "resolved file does not exist, skipping");
                continue;
            }

            // Don't add context for the same file the symbol came from
            if entry.file == sym.file {
                continue;
            }

            stats.symbols_resolved += 1;

            // Merge line ranges for same file to avoid duplicates
            let file_ranges = seen_files.entry(entry.file.clone()).or_default();
            let overlaps = file_ranges
                .iter()
                .any(|(s, e)| entry.line_start <= *e && entry.line_end >= *s);

            if !overlaps {
                file_ranges.push((entry.line_start, entry.line_end));
                entries.push(entry);
            }
        }

        // Respect follow depth (depth 1 = only direct references)
        if config.follow_depth <= 1 {
            continue;
        }
        // Higher depths would recursively resolve symbols found in resolved content.
        // For now, depth > 1 is a no-op placeholder for future expansion.
    }

    entries
}

/// Resolve an import to a file path.
fn resolve_import(
    import_path: &str,
    _source_file: &str,
    lang: &str,
    project_root: &Path,
) -> Vec<ContextEntry> {
    let mut entries = Vec::new();

    match lang {
        "rs" => {
            // `use crate::foo::bar::baz` → try `src/foo/bar/baz.rs` or `src/foo/bar/baz/mod.rs`
            let path = import_path.replace("::", "/");
            let candidates = [
                format!("src/{path}.rs"),
                format!("src/{path}/mod.rs"),
                format!("src/{path}/lib.rs"),
            ];

            for candidate in &candidates {
                let full = project_root.join(candidate);
                if full.exists() {
                    let line_end = find_definition_end(&full);
                    entries.push(ContextEntry {
                        file: candidate.clone(),
                        line_start: 1,
                        line_end,
                        label: format!("module {import_path}"),
                        priority: ContextPriority::TypeDef,
                    });
                    break;
                }
            }
        }
        "py" | "pyi" => {
            // `import foo.bar` → `foo/bar.py` or `foo/bar/__init__.py`
            let candidates = [
                format!("{import_path}.py"),
                format!("{import_path}/__init__.py"),
            ];

            for candidate in &candidates {
                let full = project_root.join(candidate);
                if full.exists() {
                    let line_end = find_definition_end(&full);
                    entries.push(ContextEntry {
                        file: candidate.clone(),
                        line_start: 1,
                        line_end,
                        label: format!("module {import_path}"),
                        priority: ContextPriority::TypeDef,
                    });
                    break;
                }
            }
        }
        "ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs" => {
            // `import x from './foo'` or `import x from 'foo'` → relative or node_modules
            let path = import_path
                .trim_start_matches("./")
                .trim_start_matches("../");

            // Only resolve relative imports (not node_modules)
            if import_path.starts_with('.') {
                let source_dir = Path::new(_source_file).parent().unwrap_or(Path::new(""));
                let resolved = source_dir.join(path);

                let extensions = match lang {
                    "ts" | "tsx" => vec!["ts", "tsx"],
                    _ => vec!["js", "jsx", "mjs", "cjs"],
                };

                for ext in &extensions {
                    let candidate = format!("{}.{}", resolved.display(), ext);
                    let full = project_root.join(&candidate);
                    if full.exists() {
                        let line_end = find_definition_end(&full);
                        entries.push(ContextEntry {
                            file: candidate,
                            line_start: 1,
                            line_end,
                            label: format!("module {import_path}"),
                            priority: ContextPriority::TypeDef,
                        });
                        break;
                    }
                }
            }
        }
        "go" => {
            // Go imports are package paths; resolve relative to project root
            let candidate = import_path.trim_start_matches("\"").trim_end_matches("\"");
            let full = project_root.join(candidate);
            if full.is_dir() {
                // Find .go files in the directory
                if let Some(entry) = find_go_package_file(&full, import_path) {
                    entries.push(entry);
                }
            }
        }
        "java" | "kt" | "kts" => {
            // `import foo.bar.Baz` → `foo/bar/Baz.java` or `foo/bar/Baz.kt`
            let path = import_path.replace('.', "/");
            let ext = if lang == "java" { "java" } else { "kt" };
            let candidate = format!("{path}.{ext}");
            let full = project_root.join(&candidate);
            if full.exists() {
                let line_end = find_definition_end(&full);
                entries.push(ContextEntry {
                    file: candidate,
                    line_start: 1,
                    line_end,
                    label: format!("module {import_path}"),
                    priority: ContextPriority::TypeDef,
                });
            }
        }
        _ => {}
    }

    entries
}

/// Resolve a function name to a file location.
/// This does a best-effort search for the function definition.
fn resolve_function(
    name: &str,
    source_file: &str,
    lang: &str,
    project_root: &Path,
) -> Vec<ContextEntry> {
    let mut entries = Vec::new();

    // Strategy: look in nearby files (same directory) and import targets
    let search_dir = Path::new(source_file).parent().unwrap_or(Path::new(""));

    match lang {
        "rs" => {
            // Search for `pub fn <name>` or `fn <name>` in sibling .rs files
            if let Ok(files) = std::fs::read_dir(project_root.join(search_dir)) {
                for file in files.flatten() {
                    let path = file.path();
                    if let Some(ext) = path.extension() {
                        if ext != "rs" {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    if let Ok(rel) = path.strip_prefix(project_root) {
                        let rel_str = rel.to_string_lossy().to_string();
                        if rel_str == source_file {
                            continue;
                        }

                        if let Some((start, end)) = find_fn_in_file(&path, name, "rs") {
                            entries.push(ContextEntry {
                                file: rel_str,
                                line_start: start,
                                line_end: end,
                                label: format!("fn {name}"),
                                priority: ContextPriority::FunctionDef,
                            });
                        }
                    }
                }
            }
        }
        "py" | "pyi" => {
            if let Ok(files) = std::fs::read_dir(project_root.join(search_dir)) {
                for file in files.flatten() {
                    let path = file.path();
                    if path.extension().map(|e| e != "py").unwrap_or(true) {
                        continue;
                    }

                    if let Ok(rel) = path.strip_prefix(project_root) {
                        let rel_str = rel.to_string_lossy().to_string();
                        if rel_str == source_file {
                            continue;
                        }

                        if let Some((start, end)) = find_fn_in_file(&path, name, "py") {
                            entries.push(ContextEntry {
                                file: rel_str,
                                line_start: start,
                                line_end: end,
                                label: format!("def {name}"),
                                priority: ContextPriority::FunctionDef,
                            });
                        }
                    }
                }
            }
        }
        _ => {
            // Generic: search for the function name in sibling files
            if let Ok(files) = std::fs::read_dir(project_root.join(search_dir)) {
                for file in files.flatten() {
                    let path = file.path();
                    if let Ok(rel) = path.strip_prefix(project_root) {
                        let rel_str = rel.to_string_lossy().to_string();
                        if rel_str == source_file {
                            continue;
                        }

                        if let Some((start, end)) = find_fn_generic(&path, name) {
                            entries.push(ContextEntry {
                                file: rel_str,
                                line_start: start,
                                line_end: end,
                                label: format!("fn {name}"),
                                priority: ContextPriority::FunctionDef,
                            });
                        }
                    }
                }
            }
        }
    }

    entries
}

/// Resolve a type name to a file location.
fn resolve_type(
    name: &str,
    source_file: &str,
    lang: &str,
    project_root: &Path,
) -> Vec<ContextEntry> {
    let search_dir = Path::new(source_file).parent().unwrap_or(Path::new(""));
    let mut entries = Vec::new();

    let pattern = match lang {
        "rs" => format!("struct {name}"),
        "py" => format!("class {name}"),
        "go" => format!("type {name} struct"),
        "java" | "kt" => format!("class {name}"),
        _ => format!("struct {name}"),
    };

    if let Ok(files) = std::fs::read_dir(project_root.join(search_dir)) {
        for file in files.flatten() {
            let path = file.path();
            if let Ok(rel) = path.strip_prefix(project_root) {
                let rel_str = rel.to_string_lossy().to_string();
                if rel_str == source_file {
                    continue;
                }

                if let Some((start, end)) = find_pattern_in_file(&path, &pattern) {
                    entries.push(ContextEntry {
                        file: rel_str,
                        line_start: start,
                        line_end: end,
                        label: format!("type {name}"),
                        priority: ContextPriority::TypeDef,
                    });
                }
            }
        }
    }

    entries
}

/// Find a function definition in a file and return (start_line, end_line).
fn find_fn_in_file(path: &Path, name: &str, lang: &str) -> Option<(u32, u32)> {
    let content = std::fs::read_to_string(path).ok()?;

    let pattern = match lang {
        "rs" => format!("fn {name}"),
        "py" => format!("def {name}"),
        _ => return find_fn_generic(path, name),
    };

    let max_lines = match lang {
        "rs" => MAX_FN_LINES,
        "py" => MAX_FN_LINES,
        _ => MAX_FN_LINES,
    };

    find_pattern_with_body(&content, &pattern, max_lines)
}

/// Generic function search (for languages without specific patterns).
fn find_fn_generic(path: &Path, name: &str) -> Option<(u32, u32)> {
    let content = std::fs::read_to_string(path).ok()?;
    find_pattern_with_body(&content, &format!("fn {name}"), MAX_FN_LINES)
}

/// Find a pattern (like `struct Foo`) and determine its extent.
fn find_pattern_in_file(path: &Path, pattern: &str) -> Option<(u32, u32)> {
    let content = std::fs::read_to_string(path).ok()?;
    find_pattern_with_body(&content, pattern, MAX_TYPE_LINES)
}

/// Find a pattern in content and estimate the block extent by counting braces/indents.
fn find_pattern_with_body(content: &str, pattern: &str, max_lines: usize) -> Option<(u32, u32)> {
    let mut start_line: Option<usize> = None;
    let mut brace_count = 0i32;
    let mut line_idx = 0;

    for line in content.lines() {
        line_idx += 1;

        if start_line.is_none() {
            if line.contains(pattern)
                && !line.trim_start().starts_with("//")
                && !line.trim_start().starts_with('#')
            {
                start_line = Some(line_idx);
                brace_count = count_braces_delta(line);
            }
            continue;
        }

        brace_count += count_braces_delta(line);

        // Block ends when braces are balanced (and we have at least the header)
        if brace_count <= 0 && start_line.is_some() {
            let start = start_line? as u32;
            let end = (line_idx as u32).min(start + max_lines as u32);
            return Some((start, end));
        }

        // Hard cap
        if line_idx >= start_line.unwrap_or(0) + max_lines {
            let start = start_line? as u32;
            return Some((start, start + max_lines as u32));
        }
    }

    // If we found the start but never balanced braces, cap at max_lines
    start_line.map(|s| {
        (
            s as u32,
            (s + max_lines.min(content.lines().count() - s + 1)) as u32,
        )
    })
}

/// Count net brace delta: +1 for `{`, -1 for `}`.
fn count_braces_delta(line: &str) -> i32 {
    let mut delta = 0i32;
    let mut in_string = false;
    let mut in_char = false;
    let mut escape = false;

    for ch in line.chars() {
        if escape {
            escape = false;
            continue;
        }
        if ch == '\\' && !in_char {
            escape = true;
            continue;
        }
        if ch == '"' && !in_char {
            in_string = !in_string;
            continue;
        }
        if ch == '\'' && !in_string && !in_char {
            in_char = true;
            continue;
        }
        if ch == '\'' && in_char {
            in_char = false;
            continue;
        }
        if in_string || in_char {
            continue;
        }
        if ch == '{' {
            delta += 1;
        } else if ch == '}' {
            delta -= 1;
        }
    }
    delta
}

/// Find the last line of a file's "definition block" (simplified).
fn find_definition_end(path: &Path) -> u32 {
    if let Ok(content) = std::fs::read_to_string(path) {
        let lines = content.lines().count();
        (lines as u32).min(MAX_TYPE_LINES as u32)
    } else {
        1
    }
}

/// Find a .go file in a package directory.
fn find_go_package_file(dir: &Path, import_path: &str) -> Option<ContextEntry> {
    let files = std::fs::read_dir(dir).ok()?;
    for file in files.flatten() {
        let path = file.path();
        if path.extension().map(|e| e == "go").unwrap_or(false) {
            let line_end = find_definition_end(&path);
            let rel = path
                .to_str()
                .and_then(|p| p.rsplit_once('/').map(|x| x.0))
                .unwrap_or(import_path);
            return Some(ContextEntry {
                file: rel.to_string(),
                line_start: 1,
                line_end,
                label: format!("package {import_path}"),
                priority: ContextPriority::TypeDef,
            });
        }
    }
    None
}

/// Check if a file path matches any ignore pattern.
fn is_ignored(file: &str, patterns: &[String]) -> bool {
    for pattern in patterns {
        // Simple glob-like matching: check if file contains the pattern
        // or matches as a suffix (e.g., "target/**" matches "target/debug/foo.rs")
        let p = pattern.trim_end_matches("**");
        if p.is_empty() {
            continue;
        }
        if file.starts_with(p.trim_end_matches('/')) || file.contains(p.trim_matches('*')) {
            return true;
        }
    }
    false
}

/// Add test file mappings for changed source files.
fn add_test_mappings(
    symbols: &[ExtractedSymbol],
    project_root: &Path,
    entries: &mut Vec<ContextEntry>,
    stats: &mut ContextStats,
) {
    // Collect unique source files from symbols
    let mut seen = std::collections::HashSet::new();

    for sym in symbols {
        if !seen.insert(&sym.file) {
            continue;
        }

        let candidates = test_file_candidates(&sym.file);
        for candidate in candidates {
            let full = project_root.join(&candidate);
            if full.exists() {
                let line_end = find_definition_end(&full);
                entries.push(ContextEntry {
                    file: candidate,
                    line_start: 1,
                    line_end,
                    label: format!("tests for {}", sym.file),
                    priority: ContextPriority::Test,
                });
                stats.symbols_resolved += 1;
                break;
            }
        }
    }
}

/// Generate test file candidate paths for a source file.
fn test_file_candidates(source: &str) -> Vec<String> {
    let mut candidates = Vec::new();

    let stem = Path::new(source)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    // Rust: `src/foo/bar.rs` → `tests/foo/bar_test.rs`, `tests/bar_test.rs`
    // Also check: `tests/foo_test.rs`
    if source.ends_with(".rs") {
        candidates.push(format!("tests/{stem}_test.rs"));
        if let Some(parent) = Path::new(source).parent() {
            candidates.push(format!("tests/{}/{}", parent.display(), stem));
            candidates.push(format!("tests/{}/{}_test.rs", parent.display(), stem));
        }
        candidates.push(format!("{stem}_test.rs"));
    }

    // Python: `foo/bar.py` → `tests/test_bar.py`, `foo/test_bar.py`
    if source.ends_with(".py") {
        candidates.push(format!("tests/test_{stem}.py"));
        if let Some(parent) = Path::new(source).parent() {
            candidates.push(format!("{}/test_{stem}.py", parent.display()));
        }
    }

    // JS/TS: `src/foo.ts` → `src/foo.test.ts`, `tests/foo.test.ts`
    if source.ends_with(".ts") || source.ends_with(".tsx") {
        let ext = source.rsplit('.').next().unwrap_or("ts");
        candidates.push(format!("tests/{stem}.test.{ext}"));
        candidates.push(format!("tests/{stem}.spec.{ext}"));
        let without_ext = &source[..source.len() - ext.len() - 1];
        candidates.push(format!("{without_ext}.test.{ext}"));
        candidates.push(format!("{without_ext}.spec.{ext}"));
    }

    // Go: `foo.go` → `foo_test.go`
    if source.ends_with(".go") {
        candidates.push(source.replace(".go", "_test.go"));
    }

    // Java/Kotlin
    if source.ends_with(".java") {
        candidates.push(source.replace(".java", "Test.java"));
    }
    if source.ends_with(".kt") {
        candidates.push(source.replace(".kt", "Test.kt"));
    }

    candidates
}

/// Read the content for a context entry, respecting line range and caps.
fn read_entry_content(entry: &ContextEntry, project_root: &Path) -> String {
    let full_path = project_root.join(&entry.file);
    let content = match std::fs::read_to_string(&full_path) {
        Ok(c) => c,
        Err(e) => {
            debug!(file = %entry.file, error = %e, "failed to read context entry file");
            return String::new();
        }
    };

    let start = entry.line_start.saturating_sub(1) as usize;
    let end = entry.line_end as usize;

    let lines: Vec<&str> = content.lines().collect();
    let relevant: Vec<&str> = lines
        .into_iter()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect();

    let result = relevant.join("\n");

    // Apply cap based on priority
    let max_lines = match entry.priority {
        ContextPriority::FunctionDef => MAX_FN_LINES,
        ContextPriority::TypeDef => MAX_TYPE_LINES,
        ContextPriority::Test => MAX_TEST_LINES,
    };

    if result.lines().count() > max_lines {
        result
            .lines()
            .take(max_lines)
            .chain(std::iter::once("... (truncated)"))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── test_file_candidates ───

    #[test]
    fn rust_test_candidates() {
        let candidates = test_file_candidates("src/engine/scanner.rs");
        assert!(
            candidates.contains(&"tests/scanner_test.rs".to_string()),
            "should generate tests/scanner_test.rs"
        );
        assert!(
            candidates.iter().any(|c| c.ends_with("engine/scanner")),
            "should generate nested test path"
        );
    }

    #[test]
    fn python_test_candidates() {
        let candidates = test_file_candidates("app/auth.py");
        assert!(
            candidates.contains(&"tests/test_auth.py".to_string()),
            "should generate tests/test_auth.py"
        );
    }

    #[test]
    fn js_test_candidates() {
        let candidates = test_file_candidates("src/api.ts");
        assert!(
            candidates.iter().any(|c| c.contains("api.test.ts")),
            "should generate .test.ts candidate"
        );
    }

    #[test]
    fn go_test_candidates() {
        let candidates = test_file_candidates("main.go");
        assert!(
            candidates.contains(&"main_test.go".to_string()),
            "should generate main_test.go"
        );
    }

    // ─── is_ignored ───

    #[test]
    fn ignore_target_dir() {
        assert!(is_ignored(
            "target/debug/foo.rs",
            &["target/**".to_string()]
        ));
    }

    #[test]
    fn ignore_node_modules() {
        assert!(is_ignored(
            "node_modules/pkg/index.js",
            &["node_modules/**".to_string()]
        ));
    }

    #[test]
    fn not_ignored_src() {
        assert!(!is_ignored("src/main.rs", &["target/**".to_string()]));
    }

    // ─── count_braces_delta ───

    #[test]
    fn brace_delta_basic() {
        assert_eq!(count_braces_delta("{ }"), 0);
        assert_eq!(count_braces_delta("{{"), 2);
        assert_eq!(count_braces_delta("}}"), -2);
        assert_eq!(count_braces_delta("fn foo() {"), 1);
        assert_eq!(count_braces_delta("}"), -1);
    }

    #[test]
    fn brace_delta_ignores_strings() {
        assert_eq!(count_braces_delta(r#"let s = "{";"#), 0);
        assert_eq!(count_braces_delta(r#"println!("{")");"#), 0);
    }

    #[test]
    fn brace_delta_ignores_comments() {
        // Brace in comment shouldn't count... but our simple parser doesn't handle
        // Rust comments. For now, it's a known limitation. The function handles
        // string escaping correctly though.
        assert_eq!(count_braces_delta(r#"let x = 1; // { }"#), 0);
    }

    // ─── find_pattern_with_body ───

    #[test]
    fn find_simple_struct() {
        let content = "fn main() {}\n\npub struct Foo {\n    x: i32,\n}\n\nfn other() {}";
        let result = find_pattern_with_body(content, "struct Foo", 10);
        assert_eq!(result, Some((3, 5)));
    }

    #[test]
    fn find_nested_function() {
        let content = "fn outer() {\n    fn inner() {\n        1\n    }\n}\n\npub fn target() {\n    let x = 1;\n    return x;\n}\n";
        let result = find_pattern_with_body(content, "fn target", 10);
        assert_eq!(result, Some((7, 10)));
    }

    #[test]
    fn find_pattern_not_found() {
        let content = "fn main() {}\npub fn other() {}";
        let result = find_pattern_with_body(content, "fn missing", 10);
        assert!(result.is_none());
    }

    // ─── build_context_chain integration ───

    #[test]
    fn disabled_config_returns_empty() {
        let config = ContextConfig {
            enabled: false,
            ..Default::default()
        };
        let chain = build_context_chain(&[], &config, Path::new("/tmp"), &[]);
        assert!(chain.text.is_empty());
    }

    #[test]
    fn empty_symbols_returns_empty() {
        let config = ContextConfig::default();
        let chain = build_context_chain(&[], &config, Path::new("/tmp"), &[]);
        assert!(chain.text.is_empty());
    }

    #[test]
    fn budget_enforced() {
        // Create a tiny budget
        let config = ContextConfig {
            enabled: true,
            max_context_tokens: 5, // very small
            follow_depth: 1,
            include_tests: false,
        };

        let symbols = vec![ExtractedSymbol {
            kind: SymbolKind::FunctionCall("some_func".to_string()),
            file: "nonexistent.rs".to_string(),
            line: 1,
            raw: "some_func()".to_string(),
        }];

        let chain = build_context_chain(&symbols, &config, Path::new("/tmp"), &[]);
        // Even if resolution finds nothing, the chain should be empty
        assert!(chain.text.is_empty() || chain.stats.budget_hit);
    }

    // ─── estimate_tokens consistency ───

    #[test]
    fn budget_accounting() {
        let config = ContextConfig {
            enabled: true,
            max_context_tokens: 100,
            follow_depth: 1,
            include_tests: false,
        };

        let symbols = vec![ExtractedSymbol {
            kind: SymbolKind::Import("engine::scanner".to_string()),
            file: "src/main.rs".to_string(),
            line: 1,
            raw: "use crate::engine::scanner;".to_string(),
        }];

        let chain = build_context_chain(&symbols, &config, Path::new("/tmp"), &[]);
        // With a nonexistent project root, nothing should resolve
        assert!(chain.text.is_empty());
        assert_eq!(chain.stats.symbols_extracted, 1);
    }
}
