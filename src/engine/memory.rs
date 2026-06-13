//! Uteke memory integration — optional recall/learn during reviews.
//!
//! Cora works 100% without Uteke. Integration is purely additive:
//! - `--memory`        → recall project patterns before review, enrich prompt
//! - `--memory --learn` → recall + save findings after review
//!
//! Uteke CLI is called via subprocess. No Rust library dependency.

use std::process::Command;

/// Whether Uteke memory integration is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryLevel {
    /// No memory — standalone review (default).
    None,
    /// Recall context before review.
    Context,
    /// Recall + save findings after review.
    Learning,
}

/// Backend for Uteke memory integration.
pub struct MemoryBackend {
    enabled: bool,
    namespace: String,
}

impl Default for MemoryBackend {
    fn default() -> Self {
        Self {
            enabled: false,
            namespace: "cora".to_string(),
        }
    }
}

impl MemoryBackend {
    /// Create a new memory backend with the given namespace.
    #[allow(dead_code)]
    pub fn new(namespace: &str) -> Self {
        Self {
            enabled: false,
            namespace: namespace.to_string(),
        }
    }

    /// Auto-detect if `uteke` CLI is available on PATH.
    pub fn detect(&mut self) {
        self.enabled = which::which("uteke").is_ok();
        if self.enabled {
            tracing::info!("Uteke detected — memory integration enabled");
        } else {
            tracing::debug!("Uteke not found — memory integration disabled");
        }
    }

    /// Check if memory is available.
    pub fn is_available(&self) -> bool {
        self.enabled
    }

    /// Recall project patterns from Uteke before review.
    ///
    /// Calls: `uteke recall "{project} code-pattern" --namespace cora --limit 5 --json`
    pub fn recall_context(&self, project: &str) -> Vec<String> {
        if !self.enabled {
            return Vec::new();
        }

        let query = format!("{project} code-pattern review-history");

        let output = match Command::new("uteke")
            .args([
                "recall",
                &query,
                "--namespace",
                &self.namespace,
                "--limit",
                "5",
                "--json",
            ])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                tracing::warn!("Failed to recall from Uteke: {e}");
                return Vec::new();
            }
        };

        if !output.status.success() {
            tracing::debug!(
                "Uteke recall returned non-zero: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Vec::new();
        }

        // Parse JSON output — uteke --json returns a JSON array:
        // [{"memory":{"content":"...",...},"score":0.xx}, ...]
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut memories = Vec::new();

        if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&stdout) {
            for val in arr {
                if let Some(content) = val
                    .get("memory")
                    .and_then(|m| m.get("content"))
                    .and_then(|c| c.as_str())
                {
                    memories.push(content.to_string());
                }
            }
        }

        tracing::info!("Recalled {} memories from Uteke", memories.len());
        memories
    }

    /// Save review findings to Uteke after review.
    ///
    /// Calls: `uteke remember "cora:{project}:stats:..." --tags cora,pattern`
    pub fn save_findings(
        &self,
        project: &str,
        total_issues: usize,
        severity_summary: &str,
        categories: &[String],
    ) {
        if !self.enabled {
            return;
        }

        // Save review stats
        let stats_content =
            format!("cora:{project}:stats:issues={total_issues},severities={severity_summary}");
        self.remember(&stats_content, &["cora", "stats"]);

        // Save category patterns
        for cat in categories {
            let pattern_content = format!("cora:{project}:pattern:{cat}");
            self.remember(&pattern_content, &["cora", "pattern", cat]);
        }
    }

    /// Save false positive feedback to Uteke.
    ///
    /// Calls: `uteke remember "cora:{project}:fp:{issue_id}" --tags cora,false-positive`
    #[allow(dead_code)]
    pub fn save_feedback(&self, project: &str, issue_id: &str, feedback: &str) {
        if !self.enabled {
            return;
        }

        let content = format!("cora:{project}:fp:{issue_id}: {feedback}");
        self.remember(&content, &["cora", "false-positive"]);
    }

    /// Build enriched system prompt section from recalled memories.
    pub fn build_memory_context(&self, memories: &[String]) -> String {
        if memories.is_empty() {
            return String::new();
        }

        let mut parts = Vec::new();
        parts.push("PROJECT HISTORY (from Uteke memory):".to_string());
        parts.push(
            "The following patterns were observed in previous reviews. Use them to reduce false positives."
                .to_string(),
        );

        for (i, mem) in memories.iter().enumerate() {
            parts.push(format!("{}. {mem}", i + 1));
        }

        parts.join("\n")
    }

    /// Internal: call `uteke remember` with content and tags.
    fn remember(&self, content: &str, tags: &[&str]) {
        let tags_str = tags.join(",");

        let result = Command::new("uteke")
            .args([
                "remember",
                content,
                "--namespace",
                &self.namespace,
                "--tags",
                &tags_str,
            ])
            .output();

        match result {
            Ok(output) if output.status.success() => {
                tracing::debug!("Saved to Uteke: {content}");
            }
            Ok(output) => {
                tracing::warn!(
                    "Uteke remember failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            Err(e) => {
                tracing::warn!("Failed to call Uteke remember: {e}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_backend_default() {
        let backend = MemoryBackend::default();
        assert!(!backend.is_available());
        assert_eq!(backend.namespace, "cora");
    }

    #[test]
    fn test_memory_backend_new() {
        let backend = MemoryBackend::new("custom-ns");
        assert!(!backend.is_available());
        assert_eq!(backend.namespace, "custom-ns");
    }

    #[test]
    fn test_detect_without_uteke() {
        let mut backend = MemoryBackend::default();
        // uteke almost certainly not on PATH in test env
        backend.detect();
        // Should not panic regardless of whether uteke exists
    }

    #[test]
    fn test_recall_context_disabled() {
        let backend = MemoryBackend::default();
        let result = backend.recall_context("test/project");
        assert!(result.is_empty());
    }

    #[test]
    fn test_save_findings_disabled() {
        let backend = MemoryBackend::default();
        // Should not panic
        backend.save_findings(
            "test/project",
            5,
            "2 warning, 3 error",
            &["security".to_string()],
        );
    }

    #[test]
    fn test_save_feedback_disabled() {
        let backend = MemoryBackend::default();
        backend.save_feedback("test/project", "issue-1", "false positive");
    }

    #[test]
    fn test_build_memory_context_empty() {
        let backend = MemoryBackend::default();
        let ctx = backend.build_memory_context(&[]);
        assert!(ctx.is_empty());
    }

    #[test]
    fn test_build_memory_context_with_memories() {
        let backend = MemoryBackend::default();
        let memories = vec![
            "unwrap() is common in test code".to_string(),
            "SQL queries use parameterized statements".to_string(),
        ];
        let ctx = backend.build_memory_context(&memories);
        assert!(ctx.contains("PROJECT HISTORY"));
        assert!(ctx.contains("unwrap()"));
        assert!(ctx.contains("SQL queries"));
        assert!(ctx.contains("1."));
        assert!(ctx.contains("2."));
    }

    #[test]
    fn test_memory_level_none() {
        assert_eq!(MemoryLevel::None, MemoryLevel::None);
        assert_ne!(MemoryLevel::None, MemoryLevel::Context);
    }

    #[test]
    fn test_memory_level_context() {
        assert_eq!(MemoryLevel::Context, MemoryLevel::Context);
        assert_ne!(MemoryLevel::Context, MemoryLevel::Learning);
    }

    #[test]
    fn test_memory_level_learning() {
        assert_eq!(MemoryLevel::Learning, MemoryLevel::Learning);
    }
}
