use anyhow::Result;
use chrono::Utc;
use serde_json::{json, Value};

use crate::engine::{ReviewIssue, ReviewResponse, ScanResponse, Severity};
use crate::formatters::Formatter;

/// SARIF formatter for GitHub Code Scanning integration.
///
/// Output conforms to the SARIF v2.1.0 specification:
/// https://docs.oasis-open.org/sarif/sarif/v2.1.0/
pub struct SarifFormatter;

impl Formatter for SarifFormatter {
    fn format_review(&self, response: &ReviewResponse) -> Result<String> {
        let sarif = build_sarif(&response.issues);
        let json = serde_json::to_string_pretty(&sarif)?;
        Ok(json)
    }

    fn format_scan(&self, response: &ScanResponse) -> Result<String> {
        let sarif = build_sarif(&response.issues);
        let json = serde_json::to_string_pretty(&sarif)?;
        Ok(json)
    }
}

/// Build a SARIF v2.1.0 document from a list of issues.
fn build_sarif(issues: &[ReviewIssue]) -> Value {
    let rules: Vec<Value> = issues
        .iter()
        .map(|issue| {
            json!({
                "id": issue.issue_type.as_ref().map(|t| t.to_string()).unwrap_or_else(|| "unknown".to_string()),
                "shortDescription": {
                    "text": issue.title.clone()
                },
                "fullDescription": {
                    "text": issue.body.clone()
                },
                "helpUri": null,
                "defaultConfiguration": {
                    "level": severity_to_sarif_level(&issue.severity)
                }
            })
        })
        .collect();

    let results: Vec<Value> = issues
        .iter()
        .map(|issue| {
            let mut result = json!({
                "ruleId": issue.issue_type.as_ref().map(|t| t.to_string()).unwrap_or_else(|| "unknown".to_string()),
                "level": severity_to_sarif_level(&issue.severity),
                "message": {
                    "text": issue.body.clone()
                },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": {
                            "uri": issue.file.clone()
                        }
                    }
                }]
            });

            // Add line number if available
            if let Some(line) = issue.line {
                result["locations"][0]["physicalLocation"]["region"] = json!({
                    "startLine": line
                });
            }

            // Add fix suggestion if available
            if let Some(ref fix) = issue.suggested_fix {
                let mut replacements = Vec::new();
                replacements.push(json!({
                    "description": {
                        "text": "Suggested fix"
                    }
                    // We don't have a full replacement spec, but we can include the text
                }));
                result["fixes"] = json!([{
                    "description": {
                        "text": fix.clone()
                    }
                }]);
            }

            result
        })
        .collect();

    json!({
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "Cora",
                    "version": env!("CARGO_PKG_VERSION"),
                    "informationUri": env!("CARGO_PKG_REPOSITORY"),
                    "rules": rules
                }
            },
            "results": results,
            "invocation": {
                "executionSuccessful": true,
                "startTimeUtc": Utc::now().to_rfc3339(),
                "endTimeUtc": Utc::now().to_rfc3339()
            }
        }]
    })
}

/// Map our severity to SARIF failure level.
fn severity_to_sarif_level(severity: &Severity) -> &str {
    match severity {
        Severity::Critical => "error",
        Severity::Major => "error",
        Severity::Minor => "warning",
        Severity::Info => "note",
    }
}
