use super::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn load_config_missing_file_returns_default() {
    let cfg = load_config(Path::new("nonexistent.yml"));
    assert!(cfg.rules.is_empty());
}

#[test]
fn load_config_invalid_yaml_returns_default() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("bad.yml");
    fs::write(&path, "invalid: [unclosed: {bracket").unwrap();
    let cfg = load_config(&path);
    assert!(cfg.rules.is_empty());
}

#[test]
fn warn_invalid_glob_pattern_does_not_panic() {
    let cfg = Config {
        rules: vec![Rule {
            pattern: "[invalid".into(),
            warn: None,
            error: None,
            warn_message: None,
            error_message: None,
        }],
        exclude: vec!["[also-invalid".into()],
    };
    warn_invalid_patterns(&cfg, Path::new("test.yml"));
}
