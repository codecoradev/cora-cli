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
