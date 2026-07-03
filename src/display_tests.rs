use super::{digits, print_json, print_status, print_violations};
use linecheck::checker::CheckOptions;
use linecheck::config::ConfigResolver;
use std::fs;
use tempfile::TempDir;

fn make_file(dir: &TempDir, name: &str, line_count: usize) -> std::path::PathBuf {
    let content = (0..line_count).map(|i| format!("line{i}\n")).collect::<String>();
    let path = dir.path().join(name);
    fs::write(&path, &content).unwrap();
    path
}

fn unlimited() -> CheckOptions {
    CheckOptions { max_lines: None, fallback_warn: None, fallback_error: None }
}

fn with_limits(warn: usize, error: usize) -> CheckOptions {
    CheckOptions { max_lines: None, fallback_warn: Some(warn), fallback_error: Some(error) }
}

fn resolver() -> ConfigResolver { ConfigResolver::new(None, "linecheck.yml") }

// --- digits ---

#[test]
fn digits_zero() { assert_eq!(digits(0), 1); }

#[test]
fn digits_single_digit() { assert_eq!(digits(9), 1); }

#[test]
fn digits_two_digits() { assert_eq!(digits(10), 2); }

#[test]
fn digits_large() { assert_eq!(digits(1000), 4); }

// --- print_violations ---

#[test]
fn print_violations_ok_file_is_silent() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "small.txt", 2);
    let mut has_error = false;
    print_violations(&[path], &mut resolver(), &with_limits(10, 20), &mut has_error).unwrap();
    assert!(!has_error);
}

#[test]
fn print_violations_error_sets_flag() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "big.txt", 20);
    let mut has_error = false;
    print_violations(&[path], &mut resolver(), &with_limits(5, 10), &mut has_error).unwrap();
    assert!(has_error);
}

#[test]
fn print_violations_warn_does_not_set_error_flag() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "medium.txt", 8);
    let mut has_error = false;
    print_violations(&[path], &mut resolver(), &with_limits(5, 20), &mut has_error).unwrap();
    assert!(!has_error);
}

#[test]
fn print_violations_unreadable_file_does_not_propagate_err() {
    // Non-existent path → check_file returns Err → eprintln in run(), not propagated
    let path = std::path::PathBuf::from("/tmp/linecheck-display-test-nonexistent.txt");
    let mut has_error = false;
    print_violations(&[path], &mut resolver(), &with_limits(10, 20), &mut has_error).unwrap();
}

// --- print_status ---

#[test]
fn print_status_covers_ok_warn_error_rows() {
    let dir = TempDir::new().unwrap();
    let ok   = make_file(&dir, "ok.txt",   2);
    let warn = make_file(&dir, "warn.txt", 8);
    let err  = make_file(&dir, "err.txt",  20);
    let mut has_error = false;
    print_status(&[ok, warn, err], &mut resolver(), &with_limits(5, 15), &mut has_error).unwrap();
    assert!(has_error);
}

#[test]
fn print_status_empty_list() {
    let mut has_error = false;
    print_status(&[], &mut resolver(), &with_limits(100, 200), &mut has_error).unwrap();
    assert!(!has_error);
}

#[test]
fn print_status_no_limits_skips_row() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "file.txt", 5);
    let mut has_error = false;
    print_status(&[path], &mut resolver(), &unlimited(), &mut has_error).unwrap();
    assert!(!has_error);
}

#[test]
fn print_status_zero_limit_shows_zero_percent() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "empty.txt", 0);
    let mut has_error = false;
    let opts = CheckOptions { max_lines: Some(0), fallback_warn: None, fallback_error: None };
    print_status(&[path], &mut resolver(), &opts, &mut has_error).unwrap();
}

#[test]
fn print_violations_message_is_printed() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("linecheck.yml"),
        "rules:\n  - pattern: '*.txt'\n    error: 5\n    error_message: 'too long!'\n").unwrap();
    let path = make_file(&dir, "big.txt", 10);
    let mut has_error = false;
    let mut res = ConfigResolver::new(None, "linecheck.yml");
    print_violations(&[path], &mut res, &unlimited(), &mut has_error).unwrap();
    assert!(has_error);
}

// --- print_json ---

#[test]
fn print_json_violations_empty_prints_brackets() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "small.txt", 2);
    let mut has_error = false;
    print_json(&[path], &mut resolver(), &with_limits(100, 200), false, &mut has_error).unwrap();
    assert!(!has_error);
}

#[test]
fn print_json_error_violation() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "big.txt", 20);
    let mut has_error = false;
    print_json(&[path], &mut resolver(), &with_limits(5, 10), false, &mut has_error).unwrap();
    assert!(has_error);
}

#[test]
fn print_json_warn_violation() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "medium.txt", 8);
    let mut has_error = false;
    print_json(&[path], &mut resolver(), &with_limits(5, 20), false, &mut has_error).unwrap();
    assert!(!has_error);
}

#[test]
fn print_json_status_mode_includes_ok_files() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "file.txt", 5);
    let mut has_error = false;
    print_json(&[path], &mut resolver(), &with_limits(10, 20), true, &mut has_error).unwrap();
    assert!(!has_error);
}

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
    fs::write(dir.path().join("linecheck.yml"),
        "rules:\n  - pattern: '*.txt'\n    error: 5\n    error_message: 'too long!'\n").unwrap();
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
    let opts = CheckOptions { max_lines: Some(0), fallback_warn: None, fallback_error: None };
    print_json(&[path], &mut resolver(), &opts, true, &mut has_error).unwrap();
}
