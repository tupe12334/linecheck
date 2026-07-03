use super::json::print_json;
use super::test_helpers::{make_file, resolver, unlimited};
use linecheck::checker::CheckOptions;
use linecheck::config::ConfigResolver;
use std::fs;
use tempfile::TempDir;
#[test]
fn print_json_status_mode_no_limits_skips_file() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "file.txt", 5);
    let mut has_error = false;
    print_json(&[path], &mut resolver(), &unlimited(), true, &mut has_error).unwrap();
}
#[test]
fn print_json_message_field_included() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("linecheck.yml"),
        "rules:\n  - pattern: '*.txt'\n    error: 5\n    error_message: 'too long!'\n",
    )
    .unwrap();
    let path = make_file(&dir, "big.txt", 10);
    let mut has_error = false;
    let mut res = ConfigResolver::new(None, "linecheck.yml");
    print_json(&[path], &mut res, &unlimited(), false, &mut has_error).unwrap();
    assert!(has_error);
}
#[test]
fn print_json_zero_limit_zero_percent() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "empty.txt", 0);
    let mut has_error = false;
    let opts = CheckOptions {
        max_lines: Some(0),
        fallback_warn: None,
        fallback_error: None,
    };
    print_json(&[path], &mut resolver(), &opts, true, &mut has_error).unwrap();
}
