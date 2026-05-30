use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::engine::Severity;

/// Runtime configuration — merged from defaults + .cora.yaml + CLI flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Provider configuration.
    pub provider: ProviderConfig,
    /// Focus areas for review.
    pub focus: Vec<String>,
    /// Custom review rules.
    pub rules: Vec<String>,
    /// Ignore configuration.
    pub ignore: IgnoreConfig,
    /// Hook configuration.
    pub hook: HookConfig,
    /// Output configuration.
    pub output: OutputConfig,
}

/// Provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: String,
    pub model: String,
    pub base_url: String,
}

/// Ignore configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IgnoreConfig {
    pub files: Vec<String>,
    pub rules: Vec<String>,
}

/// Hook configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookConfig {
    pub mode: String,
    pub min_severity: String,
    pub max_diff_size: usize,
}

/// Output configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: String,
    pub color: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: ProviderConfig {
                provider: "openai".to_string(),
                model: "gpt-4o-mini".to_string(),
                base_url: "https://api.openai.com/v1".to_string(),
            },
            focus: vec!["security".into(), "performance".into(), "bugs".into(), "best_practice".into()],
            rules: vec![],
            ignore: IgnoreConfig {
                files: vec!["node_modules/**".into(), "dist/**".into(), "target/**".into(), ".git/**".into()],
                rules: vec![],
            },
            hook: HookConfig {
                mode: "warn".to_string(),
                min_severity: "major".to_string(),
                max_diff_size: 50 * 1024,
            },
            output: OutputConfig {
                format: "pretty".to_string(),
                color: true,
            },
        }
    }
}

impl HookConfig {
    /// Parse the min_severity string into a Severity enum.
    pub fn min_severity_level(&self) -> Severity {
        Severity::from_str_lossy(&self.min_severity)
    }
}

/// Serde-compatible schema for the `.cora.yaml` configuration file.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoraFile {
    pub provider: Option<ProviderSection>,
    pub focus: Option<Vec<String>>,
    pub rules: Option<Vec<String>>,
    pub ignore: Option<IgnoreSection>,
    pub hook: Option<HookSection>,
    pub output: Option<OutputSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderSection {
    pub provider: Option<String>,
    pub model: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IgnoreSection {
    pub files: Option<Vec<String>>,
    pub rules: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HookSection {
    pub mode: Option<String>,
    pub min_severity: Option<String>,
    pub max_diff_size: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutputSection {
    pub format: Option<String>,
    pub color: Option<bool>,
}

impl CoraFile {
    pub fn from_str(content: &str) -> Result<Self> {
        serde_yaml::from_str(content).context("failed to parse .cora.yaml")
    }

    /// Merge this file config into a `Config`, overwriting only fields that are present.
    pub fn merge_into(&self, config: &mut Config) {
        if let Some(p) = &self.provider {
            if let Some(v) = &p.provider { config.provider.provider = v.clone(); }
            if let Some(v) = &p.model { config.provider.model = v.clone(); }
            if let Some(v) = &p.base_url { config.provider.base_url = v.clone(); }
        }
        if let Some(v) = &self.focus { config.focus = v.clone(); }
        if let Some(v) = &self.rules { config.rules = v.clone(); }
        if let Some(ig) = &self.ignore {
            if let Some(v) = &ig.files { config.ignore.files = v.clone(); }
            if let Some(v) = &ig.rules { config.ignore.rules = v.clone(); }
        }
        if let Some(h) = &self.hook {
            if let Some(v) = &h.mode { config.hook.mode = v.clone(); }
            if let Some(v) = &h.min_severity { config.hook.min_severity = v.clone(); }
            if let Some(v) = h.max_diff_size { config.hook.max_diff_size = v; }
        }
        if let Some(o) = &self.output {
            if let Some(v) = &o.format { config.output.format = v.clone(); }
            if let Some(v) = o.color { config.output.color = v; }
        }
    }
}
