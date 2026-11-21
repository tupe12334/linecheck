use super::status::print_status;
use super::test_helpers::{make_file, resolver, unlimited, with_limits};
use linecheck::checker::CheckOptions;
use tempfile::TempDir;
#[test]
fn print_status_covers_ok_warn_error_rows() {
    let dir = TempDir::new().unwrap();
    let (ok, warn, err) = (
        make_file(&dir, "ok.txt", 2),
        make_file(&dir, "warn.txt", 8),
        make_file(&dir, "err.txt", 20),
    );
    let mut has_error = false;
    print_status(
        &[ok, warn, err],
        &mut resolver(),
        &with_limits(5, 15),
        &mut has_error,
    );
    assert!(has_error);
}
#[test]
fn print_status_empty_list() {
    let mut has_error = false;
    print_status(&[], &mut resolver(), &with_limits(100, 200), &mut has_error);
    assert!(!has_error);
}
#[test]
fn print_status_no_limits_skips_row() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "file.txt", 5);
    let mut has_error = false;
    print_status(&[path], &mut resolver(), &unlimited(), &mut has_error);
    assert!(!has_error);
}
#[test]
fn print_status_zero_limit_shows_zero_percent() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "empty.txt", 0);
    let mut has_error = false;
    let opts = CheckOptions {
        max_lines: Some(0),
        fallback_warn: None,
        fallback_error: None,
        skip_whitespace: false,
    };
    print_status(&[path], &mut resolver(), &opts, &mut has_error);
}
