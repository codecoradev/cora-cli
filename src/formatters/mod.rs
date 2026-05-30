pub mod compact;
pub mod json_fmt;
pub mod pretty;
pub mod sarif;

use anyhow::Result;
use crate::engine::ReviewResponse;
use crate::engine::ScanResponse;

/// Output format supported by cora.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Pretty,
    Json,
    Compact,
    Sarif,
}

impl OutputFormat {
    /// Parse from string, case-insensitive.
    pub fn from_str_loose(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "pretty" => Ok(Self::Pretty),
            "json" => Ok(Self::Json),
            "compact" => Ok(Self::Compact),
            "sarif" => Ok(Self::Sarif),
            other => anyhow::bail!("unknown format: {other}"),
        }
    }
}

/// Trait for formatting review output.
pub trait Formatter {
    /// Format a diff review response for display.
    fn format_review(&self, response: &ReviewResponse) -> Result<String>;

    /// Format a scan response for display.
    fn format_scan(&self, response: &ScanResponse) -> Result<String>;
}

/// Create a formatter for the given output format.
pub fn formatter_for(format: OutputFormat) -> Box<dyn Formatter> {
    match format {
        OutputFormat::Pretty => Box::new(pretty::PrettyFormatter),
        OutputFormat::Compact => Box::new(compact::CompactFormatter),
        OutputFormat::Json => Box::new(json_fmt::JsonFormatter),
        OutputFormat::Sarif => Box::new(sarif::SarifFormatter),
    }
}
