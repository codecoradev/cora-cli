use std::io::Write;

use assert_cmd::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;

fn cora_cmd() -> Command {
    Command::cargo_bin("cora").unwrap()
}

/// Helper: write content to a temp file and return its path.
fn write_temp_yaml(content: &str) -> NamedTempFile {
    let mut file = tempfile::Builder::new().suffix(".yaml").tempfile().unwrap();
    write!(file, "{}", content).unwrap();
    file.flush().unwrap();
    file
}

// ═══════════════════════════════════════════════════════════
// Config file parsing via init + manual verification
// ═══════════════════════════════════════════════════════════

#[test]
fn init_creates_valid_yaml() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let config_path = tmp_dir.path().join(".cora.yaml");

    cora_cmd()
        .args(["init"])
        .current_dir(tmp_dir.path())
        .assert()
        .success();

    // Verify the created file is valid YAML
    let content = std::fs::read_to_string(&config_path).unwrap();
    let parsed: serde_yml::Value =
        serde_yml::from_str(&content).expect("init should create valid YAML");

    // Verify it has expected top-level keys
    assert!(
        parsed.get("provider").is_some(),
        "should have provider section"
    );
    assert!(parsed.get("focus").is_some(), "should have focus section");
    assert!(parsed.get("output").is_some(), "should have output section");
}

#[test]
fn config_file_with_custom_provider() {
    let yaml = r#"
provider:
  provider: anthropic
  model: claude-3-haiku
  base_url: https://api.anthropic.com/v1
focus:
  - security
output:
  format: json
"#;
    let file = write_temp_yaml(yaml);

    // Parse and verify using serde_yml directly
    let content = std::fs::read_to_string(file.path()).unwrap();
    let parsed: serde_yml::Value = serde_yml::from_str(&content).unwrap();

    assert_eq!(parsed["provider"]["provider"].as_str(), Some("anthropic"));
    assert_eq!(parsed["provider"]["model"].as_str(), Some("claude-3-haiku"));
    assert_eq!(parsed["focus"].as_sequence().unwrap().len(), 1);
    assert_eq!(parsed["output"]["format"].as_str(), Some("json"));
}

#[test]
fn config_file_roundtrip_yaml() {
    let yaml = r#"
provider:
  provider: ollama
  model: llama3
  base_url: http://localhost:11434
hook:
  mode: block
  min_severity: critical
ignore:
  files:
    - vendor/**
    - generated/**
"#;
    let file = write_temp_yaml(yaml);
    let content = std::fs::read_to_string(file.path()).unwrap();

    // Parse to Value, serialize back
    let val: serde_yml::Value = serde_yml::from_str(&content).unwrap();
    let reser = serde_yml::to_string(&val).unwrap();

    // Re-parse the re-serialized version to ensure round-trip
    let val2: serde_yml::Value = serde_yml::from_str(&reser).unwrap();
    assert_eq!(val2["provider"]["provider"], val["provider"]["provider"]);
    assert_eq!(val2["hook"]["mode"], val["hook"]["mode"]);
    assert_eq!(
        val2["ignore"]["files"].as_sequence().unwrap().len(),
        val["ignore"]["files"].as_sequence().unwrap().len()
    );
}

#[test]
fn config_file_empty_yaml_is_valid() {
    let yaml = "";
    let file = write_temp_yaml(yaml);
    let content = std::fs::read_to_string(file.path()).unwrap();
    let parsed: serde_yml::Value = serde_yml::from_str(&content).unwrap();
    assert!(parsed.is_null());
}

#[test]
fn config_file_minimal_sections() {
    let yaml = r#"
focus:
  - security
"#;
    let file = write_temp_yaml(yaml);
    let content = std::fs::read_to_string(file.path()).unwrap();
    let parsed: serde_yml::Value = serde_yml::from_str(&content).unwrap();
    assert_eq!(parsed["focus"].as_sequence().unwrap().len(), 1);
    assert!(parsed.get("provider").is_none());
}

#[test]
fn cli_uses_config_file_via_flag() {
    // Create a temp config and verify the CLI at least accepts the flag
    let yaml = r#"
provider:
  provider: test-provider
model: test-model
"#;
    let file = write_temp_yaml(yaml);

    // We can't run a full review without an API key, but we can verify
    // the --config flag is accepted
    cora_cmd()
        .args([
            "--config",
            &file.path().to_string_lossy(),
            "review",
            "--help",
        ])
        .assert()
        .success();
}
