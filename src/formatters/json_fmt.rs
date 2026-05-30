use anyhow::Result;

use crate::engine::{ReviewResponse, ScanResponse};
use crate::formatters::Formatter;

/// JSON formatter: outputs raw JSON.
pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format_review(&self, response: &ReviewResponse) -> Result<String> {
        let json = serde_json::to_string_pretty(response)?;
        Ok(json)
    }

    fn format_scan(&self, response: &ScanResponse) -> Result<String> {
        let json = serde_json::to_string_pretty(response)?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{ReviewIssue, Severity, TokenUsage};

    fn sample_issue() -> ReviewIssue {
        ReviewIssue {
            file: "src/main.rs".to_string(),
            line: Some(42),
            severity: Severity::Critical,
            issue_type: Some("security".to_string()),
            title: "SQL Injection".to_string(),
            body: "User input concatenated into query.".to_string(),
            suggested_fix: Some("Use parameterized queries.".to_string()),
        }
    }

    fn sample_response() -> ReviewResponse {
        ReviewResponse {
            issues: vec![sample_issue()],
            summary: "Found 1 critical issue.".to_string(),
            tokens_used: Some(TokenUsage {
                input_tokens: 100,
                output_tokens: 200,
                estimated_cost_usd: 0.005,
            }),
            should_block: false,
        }
    }

    fn empty_response() -> ReviewResponse {
        ReviewResponse {
            issues: vec![],
            summary: String::new(),
            tokens_used: None,
            should_block: false,
        }
    }

    fn sample_scan_response() -> ScanResponse {
        ScanResponse {
            issues: vec![sample_issue()],
            summary: "Scan complete.".to_string(),
            files_scanned: 10,
            lines_scanned: 500,
            tokens_used: None,
            should_block: false,
        }
    }

    #[test]
    fn format_review_output_is_valid_json() {
        let fmt = JsonFormatter;
        let output = fmt.format_review(&sample_response()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(parsed.get("issues").is_some());
        assert!(parsed.get("summary").is_some());
    }

    #[test]
    fn format_review_contains_issue_data() {
        let fmt = JsonFormatter;
        let output = fmt.format_review(&sample_response()).unwrap();
        assert!(output.contains("src/main.rs"));
        assert!(output.contains("SQL Injection"));
        assert!(output.contains("critical"));
    }

    #[test]
    fn format_review_empty_issues() {
        let fmt = JsonFormatter;
        let output = fmt.format_review(&empty_response()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["issues"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn format_scan_output_is_valid_json() {
        let fmt = JsonFormatter;
        let output = fmt.format_scan(&sample_scan_response()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(parsed.get("files_scanned").is_some());
        assert!(parsed.get("lines_scanned").is_some());
        assert_eq!(parsed["files_scanned"].as_u64().unwrap(), 10);
    }

    #[test]
    fn format_review_multiple_issues() {
        let mut response = sample_response();
        response.issues.push(ReviewIssue {
            file: "src/lib.rs".to_string(),
            line: Some(10),
            severity: Severity::Minor,
            issue_type: Some("style".to_string()),
            title: "Naming".to_string(),
            body: "Use snake_case.".to_string(),
            suggested_fix: None,
        });
        let fmt = JsonFormatter;
        let output = fmt.format_review(&response).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(parsed["issues"].as_array().unwrap().len(), 2);
    }
}
