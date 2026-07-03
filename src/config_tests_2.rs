use super::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn resolver_returns_none_for_parentless_path() {
    let mut resolver = ConfigResolver::new(None, "linecheck.yml");
    assert!(resolver.resolve(Path::new("")).is_none());
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
    assert!(resolver.resolve(&file).is_none());
}
