use anyhow::Result;
use colored::Colorize;

use crate::engine::{ReviewIssue, ReviewResponse, ScanResponse, Severity};
use crate::formatters::Formatter;

/// Compact formatter: one line per issue. Ideal for git hooks.
pub struct CompactFormatter;

impl Formatter for CompactFormatter {
    fn format_review(&self, response: &ReviewResponse) -> Result<String> {
        let mut output = String::new();

        if response.issues.is_empty() {
            output.push_str("No issues found.\n");
            return Ok(output);
        }

        output.push_str(&format!(
            "Found {} issue{}.\n",
            response.issues.len(),
            if response.issues.len() == 1 { "" } else { "s" },
        ));

        for issue in &response.issues {
            output.push_str(&format_issue_compact(issue));
        }

        Ok(output)
    }

    fn format_scan(&self, response: &ScanResponse) -> Result<String> {
        let mut output = String::new();

        output.push_str(&format!(
            "Scanned {} files — Found {} issue{}.\n",
            response.files_scanned,
            response.issues.len(),
            if response.issues.len() == 1 { "" } else { "s" },
        ));

        for issue in &response.issues {
            output.push_str(&format_issue_compact(issue));
        }

        Ok(output)
    }
}

/// Format a single issue as one line: [SEVERITY] file:line: title
fn format_issue_compact(issue: &ReviewIssue) -> String {
    let sev = match issue.severity {
        Severity::Critical => "CRITICAL".red().bold().to_string(),
        Severity::Major => "MAJOR".yellow().bold().to_string(),
        Severity::Minor => "MINOR".blue().to_string(),
        Severity::Info => "INFO".white().dimmed().to_string(),
    };

    let loc = match issue.line {
        Some(l) => format!("{}:{}", issue.file, l),
        None => issue.file.clone(),
    };

    format!("[{sev}] {loc}: {}\n", issue.title)
}
