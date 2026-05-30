pub mod compact;
pub mod json_fmt;
pub mod pretty;
pub mod sarif;

use crate::engine::ReviewResponse;
use crate::engine::ScanResponse;
use anyhow::Result;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_format_pretty() {
        assert_eq!(OutputFormat::from_str_loose("pretty").unwrap(), OutputFormat::Pretty);
    }

    #[test]
    fn output_format_json() {
        assert_eq!(OutputFormat::from_str_loose("json").unwrap(), OutputFormat::Json);
    }

    #[test]
    fn output_format_compact() {
        assert_eq!(OutputFormat::from_str_loose("compact").unwrap(), OutputFormat::Compact);
    }

    #[test]
    fn output_format_sarif() {
        assert_eq!(OutputFormat::from_str_loose("sarif").unwrap(), OutputFormat::Sarif);
    }

    #[test]
    fn output_format_case_insensitive() {
        assert_eq!(OutputFormat::from_str_loose("JSON").unwrap(), OutputFormat::Json);
        assert_eq!(OutputFormat::from_str_loose("Pretty").unwrap(), OutputFormat::Pretty);
        assert_eq!(OutputFormat::from_str_loose("SARIF").unwrap(), OutputFormat::Sarif);
    }

    #[test]
    fn output_format_unknown_errors() {
        assert!(OutputFormat::from_str_loose("yaml").is_err());
        assert!(OutputFormat::from_str_loose("").is_err());
    }

    #[test]
    fn formatter_for_returns_box() {
        let fmt = formatter_for(OutputFormat::Json);
        let _ = fmt; // just verify it compiles
    }
}
