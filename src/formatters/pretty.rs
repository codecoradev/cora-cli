use anyhow::Result;
use colored::Colorize;

use crate::engine::{ReviewIssue, ReviewResponse, ScanResponse, Severity};
use crate::formatters::Formatter;

/// Pretty formatter with colored terminal output and severity icons.
pub struct PrettyFormatter;

impl Formatter for PrettyFormatter {
    fn format_review(&self, response: &ReviewResponse) -> Result<String> {
        let mut output = String::new();

        // Header
        output.push_str(&format!(
            "{}\n\n",
            "╔══════════════════════════════════════════╗
║          Cora Code Review Results        ║
╚══════════════════════════════════════════╝"
                .cyan()
                .bold()
        ));

        if response.issues.is_empty() {
            output.push_str(
                &"✅ No issues found!".green().bold().to_string(),
            );
            if !response.summary.is_empty() {
                output.push_str(&format!("\n\n{}", response.summary.dimmed()));
            }
            return Ok(output);
        }

        // Summary line
        let crit = response.issues.iter().filter(|i| i.severity == Severity::Critical).count();
        let maj = response.issues.iter().filter(|i| i.severity == Severity::Major).count();
        let min = response.issues.iter().filter(|i| i.severity == Severity::Minor).count();
        let inf = response.issues.iter().filter(|i| i.severity == Severity::Info).count();

        output.push_str(&format!(
            "Found {} issue{}: {} {} {} {} {}\n\n",
            response.issues.len(),
            if response.issues.len() == 1 { "" } else { "s" },
            format!("{} critical", crit).red(),
            format!("{} major", maj).yellow(),
            format!("{} minor", min).blue(),
            format!("{} info", inf).white(),
            if response.should_block {
                format!("\n{}", "⚠ BLOCKED".red().bold())
            } else {
                String::new()
            },
        ));

        // Issues
        for (idx, issue) in response.issues.iter().enumerate() {
            output.push_str(&format_issue_pretty(issue, idx + 1));
            output.push('\n');
        }

        // Summary
        if !response.summary.is_empty() {
            output.push_str(&format!(
                "\n{}\n{}",
                "─".repeat(60).dimmed(),
                response.summary.white().bold()
            ));
        }

        Ok(output)
    }

    fn format_scan(&self, response: &ScanResponse) -> Result<String> {
        let mut output = String::new();

        output.push_str(&format!(
            "{}\n",
            "╔══════════════════════════════════════════╗
║        Cora Project Scan Results           ║
╚══════════════════════════════════════════╝"
                .cyan()
                .bold()
        ));

        output.push_str(&format!(
            "Scanned {} files ({} lines)\n\n",
            response.files_scanned, response.lines_scanned
        ));

        if response.issues.is_empty() {
            output.push_str(&"✅ No issues found!".green().bold().to_string());
            if !response.summary.is_empty() {
                output.push_str(&format!("\n\n{}", response.summary.dimmed()));
            }
            return Ok(output);
        }

        let crit = response.issues.iter().filter(|i| i.severity == Severity::Critical).count();
        let maj = response.issues.iter().filter(|i| i.severity == Severity::Major).count();
        let min = response.issues.iter().filter(|i| i.severity == Severity::Minor).count();
        let inf = response.issues.iter().filter(|i| i.severity == Severity::Info).count();

        output.push_str(&format!(
            "Found {} issue{}: {} {} {} {} {}\n\n",
            response.issues.len(),
            if response.issues.len() == 1 { "" } else { "s" },
            format!("{} critical", crit).red(),
            format!("{} major", maj).yellow(),
            format!("{} minor", min).blue(),
            format!("{} info", inf).white(),
            if response.should_block {
                format!("\n{}", "⚠ BLOCKED".red().bold())
            } else {
                String::new()
            },
        ));

        for (idx, issue) in response.issues.iter().enumerate() {
            output.push_str(&format_issue_pretty(issue, idx + 1));
            output.push('\n');
        }

        if !response.summary.is_empty() {
            output.push_str(&format!(
                "\n{}\n{}",
                "─".repeat(60).dimmed(),
                response.summary.white().bold()
            ));
        }

        Ok(output)
    }
}

/// Format a single issue with colors and icons.
fn format_issue_pretty(issue: &ReviewIssue, num: usize) -> String {
    let mut out = String::new();

    // Header: icon + severity + type
    let icon = issue.severity.icon();
    let sev_colored = match issue.severity {
        Severity::Critical => issue.severity.label().red().bold(),
        Severity::Major => issue.severity.label().yellow().bold(),
        Severity::Minor => issue.severity.label().blue().bold(),
        Severity::Info => issue.severity.label().white().bold(),
    };

    out.push_str(&format!("{icon} [{sev_colored}] "));
    if let Some(ref itype) = issue.issue_type {
        out.push_str(&format!("{}: ", itype));
    }

    // File:line location
    let loc = match issue.line {
        Some(l) => format!("{}:{}", issue.file, l),
        None => issue.file.clone(),
    };
    out.push_str(&loc.white().bold().to_string());
    out.push('\n');

    // Title
    out.push_str(&format!("  {} {}\n", format!("#{num}").dimmed(), issue.title.bold()));

    // Body
    if !issue.body.is_empty() {
        for line in issue.body.lines().take(10) {
            out.push_str(&format!("    {}\n", line.dimmed()));
        }
    }

    // Suggested fix
    if let Some(ref fix) = issue.suggested_fix {
        if !fix.trim().is_empty() {
            out.push_str(&format!(
                "  {}\n",
                "💡 Suggested fix:".green().bold()
            ));
            out.push_str(&format!("    {}\n", fix));
        }
    }

    out
}
