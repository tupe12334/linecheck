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
        rules: vec![Rule { pattern: "[invalid".into(), warn: None, error: None, warn_message: None, error_message: None }],
        exclude: vec!["[also-invalid".into()],
    };
    // Should not panic — just prints to stderr
    warn_invalid_patterns(&cfg, Path::new("test.yml"));
}

#[test]
fn resolver_returns_none_when_no_config_exists() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("file.txt");
    fs::write(&file, "hello\n").unwrap();
    let mut resolver = ConfigResolver::new(None, "linecheck.yml");
    assert!(resolver.resolve(&file).is_none());
}

#[test]
fn resolver_explicit_invalid_yaml_returns_none() {
    let dir = TempDir::new().unwrap();
    let cfg_path = dir.path().join("linecheck.yml");
    fs::write(&cfg_path, "invalid: [unclosed").unwrap();
    let file = dir.path().join("file.txt");
    fs::write(&file, "hello\n").unwrap();
    let mut resolver = ConfigResolver::new(Some(cfg_path), "linecheck.yml");
    // load_cached returns None on parse error
    assert!(resolver.resolve(&file).is_none());
}
