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
            focus: vec![
                "security".into(),
                "performance".into(),
                "bugs".into(),
                "best_practice".into(),
            ],
            rules: vec![],
            ignore: IgnoreConfig {
                files: vec![
                    "node_modules/**".into(),
                    "dist/**".into(),
                    "target/**".into(),
                    ".git/**".into(),
                ],
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
            if let Some(v) = &p.provider {
                config.provider.provider = v.clone();
            }
            if let Some(v) = &p.model {
                config.provider.model = v.clone();
            }
            if let Some(v) = &p.base_url {
                config.provider.base_url = v.clone();
            }
        }
        if let Some(v) = &self.focus {
            config.focus = v.clone();
        }
        if let Some(v) = &self.rules {
            config.rules = v.clone();
        }
        if let Some(ig) = &self.ignore {
            if let Some(v) = &ig.files {
                config.ignore.files = v.clone();
            }
            if let Some(v) = &ig.rules {
                config.ignore.rules = v.clone();
            }
        }
        if let Some(h) = &self.hook {
            if let Some(v) = &h.mode {
                config.hook.mode = v.clone();
            }
            if let Some(v) = &h.min_severity {
                config.hook.min_severity = v.clone();
            }
            if let Some(v) = h.max_diff_size {
                config.hook.max_diff_size = v;
            }
        }
        if let Some(o) = &self.output {
            if let Some(v) = &o.format {
                config.output.format = v.clone();
            }
            if let Some(v) = o.color {
                config.output.color = v;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─── Config::default() ───

    #[test]
    fn config_default_provider() {
        let cfg = Config::default();
        assert_eq!(cfg.provider.provider, "openai");
        assert_eq!(cfg.provider.model, "gpt-4o-mini");
        assert_eq!(cfg.provider.base_url, "https://api.openai.com/v1");
    }

    #[test]
    fn config_default_focus() {
        let cfg = Config::default();
        assert_eq!(
            cfg.focus,
            vec!["security", "performance", "bugs", "best_practice"]
        );
    }

    #[test]
    fn config_default_rules_empty() {
        let cfg = Config::default();
        assert!(cfg.rules.is_empty());
    }

    #[test]
    fn config_default_ignore_files() {
        let cfg = Config::default();
        assert!(cfg.ignore.files.contains(&"node_modules/**".to_string()));
        assert!(cfg.ignore.files.contains(&"dist/**".to_string()));
        assert!(cfg.ignore.files.contains(&"target/**".to_string()));
        assert!(cfg.ignore.files.contains(&".git/**".to_string()));
    }

    #[test]
    fn config_default_hook() {
        let cfg = Config::default();
        assert_eq!(cfg.hook.mode, "warn");
        assert_eq!(cfg.hook.min_severity, "major");
        assert_eq!(cfg.hook.max_diff_size, 50 * 1024);
    }

    #[test]
    fn config_default_output() {
        let cfg = Config::default();
        assert_eq!(cfg.output.format, "pretty");
        assert!(cfg.output.color);
    }

    // ─── CoraFile::merge_into ───

    #[test]
    fn merge_empty_cora_file_leaves_defaults() {
        let mut cfg = Config::default();
        let cora = CoraFile::default();
        cora.merge_into(&mut cfg);
        assert_eq!(cfg.provider.provider, "openai");
        assert_eq!(cfg.provider.model, "gpt-4o-mini");
        assert_eq!(cfg.output.format, "pretty");
    }

    #[test]
    fn merge_provider_overrides() {
        let mut cfg = Config::default();
        let cora = CoraFile {
            provider: Some(ProviderSection {
                provider: Some("anthropic".to_string()),
                model: Some("claude-3-haiku".to_string()),
                base_url: Some("https://api.anthropic.com/v1".to_string()),
            }),
            ..Default::default()
        };
        cora.merge_into(&mut cfg);
        assert_eq!(cfg.provider.provider, "anthropic");
        assert_eq!(cfg.provider.model, "claude-3-haiku");
        assert_eq!(cfg.provider.base_url, "https://api.anthropic.com/v1");
    }

    #[test]
    fn merge_partial_provider() {
        let mut cfg = Config::default();
        let cora = CoraFile {
            provider: Some(ProviderSection {
                provider: Some("ollama".to_string()),
                model: None,
                base_url: None,
            }),
            ..Default::default()
        };
        cora.merge_into(&mut cfg);
        assert_eq!(cfg.provider.provider, "ollama");
        assert_eq!(cfg.provider.model, "gpt-4o-mini"); // unchanged
        assert_eq!(cfg.provider.base_url, "https://api.openai.com/v1"); // unchanged
    }

    #[test]
    fn merge_focus() {
        let mut cfg = Config::default();
        let cora = CoraFile {
            focus: Some(vec!["security".to_string(), "bugs".to_string()]),
            ..Default::default()
        };
        cora.merge_into(&mut cfg);
        assert_eq!(cfg.focus, vec!["security", "bugs"]);
    }

    #[test]
    fn merge_rules() {
        let mut cfg = Config::default();
        let cora = CoraFile {
            rules: Some(vec!["no unwrap".to_string()]),
            ..Default::default()
        };
        cora.merge_into(&mut cfg);
        assert_eq!(cfg.rules, vec!["no unwrap"]);
    }

    #[test]
    fn merge_ignore() {
        let mut cfg = Config::default();
        let cora = CoraFile {
            ignore: Some(IgnoreSection {
                files: Some(vec!["vendor/**".to_string()]),
                rules: Some(vec!["skip-rule-1".to_string()]),
            }),
            ..Default::default()
        };
        cora.merge_into(&mut cfg);
        assert_eq!(cfg.ignore.files, vec!["vendor/**"]);
        assert_eq!(cfg.ignore.rules, vec!["skip-rule-1"]);
    }

    #[test]
    fn merge_hook() {
        let mut cfg = Config::default();
        let cora = CoraFile {
            hook: Some(HookSection {
                mode: Some("block".to_string()),
                min_severity: Some("critical".to_string()),
                max_diff_size: Some(1024),
            }),
            ..Default::default()
        };
        cora.merge_into(&mut cfg);
        assert_eq!(cfg.hook.mode, "block");
        assert_eq!(cfg.hook.min_severity, "critical");
        assert_eq!(cfg.hook.max_diff_size, 1024);
    }

    #[test]
    fn merge_output() {
        let mut cfg = Config::default();
        let cora = CoraFile {
            output: Some(OutputSection {
                format: Some("json".to_string()),
                color: Some(false),
            }),
            ..Default::default()
        };
        cora.merge_into(&mut cfg);
        assert_eq!(cfg.output.format, "json");
        assert!(!cfg.output.color);
    }

    // ─── CoraFile::from_str (YAML parsing) ───

    #[test]
    fn parse_cora_file_empty() {
        let cora = CoraFile::from_str("").unwrap();
        assert!(cora.provider.is_none());
        assert!(cora.focus.is_none());
    }

    #[test]
    fn parse_cora_file_full() {
        let yaml = r#"
provider:
  provider: anthropic
  model: claude-3-haiku
  base_url: https://api.anthropic.com/v1
focus:
  - security
  - bugs
rules:
  - no unwrap
ignore:
  files:
    - vendor/**
hook:
  mode: block
  min_severity: critical
output:
  format: json
  color: false
"#;
        let cora = CoraFile::from_str(yaml).unwrap();
        assert_eq!(
            cora.provider.as_ref().unwrap().provider.as_deref(),
            Some("anthropic")
        );
        assert_eq!(cora.focus.as_ref().unwrap().len(), 2);
        assert_eq!(cora.rules.as_ref().unwrap().len(), 1);
        assert_eq!(
            cora.output.as_ref().unwrap().format.as_deref(),
            Some("json")
        );
        assert_eq!(cora.output.as_ref().unwrap().color, Some(false));
    }

    // ─── HookConfig::min_severity_level ───

    #[test]
    fn hook_min_severity_level() {
        let cfg = HookConfig {
            mode: "warn".to_string(),
            min_severity: "critical".to_string(),
            max_diff_size: 1024,
        };
        assert_eq!(cfg.min_severity_level(), Severity::Critical);
    }

    #[test]
    fn hook_min_severity_level_unknown() {
        let cfg = HookConfig {
            mode: "warn".to_string(),
            min_severity: "whatever".to_string(),
            max_diff_size: 1024,
        };
        assert_eq!(cfg.min_severity_level(), Severity::Info);
    }

    // ─── CoraFile serde round-trip ───

    #[test]
    fn cora_file_yaml_roundtrip() {
        let cora = CoraFile {
            provider: Some(ProviderSection {
                provider: Some("ollama".to_string()),
                model: Some("llama3".to_string()),
                base_url: Some("http://localhost:11434".to_string()),
            }),
            focus: Some(vec!["security".to_string()]),
            ..Default::default()
        };
        let yaml = serde_yaml::to_string(&cora).unwrap();
        let back: CoraFile = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(
            back.provider.as_ref().unwrap().provider.as_deref(),
            Some("ollama")
        );
        assert_eq!(back.focus.as_ref().unwrap().len(), 1);
    }

    // ─── Config serde round-trip ───

    #[test]
    fn config_json_roundtrip() {
        let cfg = Config::default();
        let json = serde_json::to_string(&cfg).unwrap();
        let back: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(back.provider.provider, cfg.provider.provider);
        assert_eq!(back.output.format, cfg.output.format);
    }
}
