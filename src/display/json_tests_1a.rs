use super::json::print_json;
use super::test_helpers::{make_file, resolver, with_limits};
use tempfile::TempDir;
#[test]
fn print_json_violations_empty_prints_brackets() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "small.txt", 2);
    let mut has_error = false;
    print_json(
        &[path],
        &mut resolver(),
        &with_limits(100, 200),
        false,
        &mut has_error,
    )
    .unwrap();
    assert!(!has_error);
}
#[test]
fn print_json_error_violation() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "big.txt", 20);
    let mut has_error = false;
    print_json(
        &[path],
        &mut resolver(),
        &with_limits(5, 10),
        false,
        &mut has_error,
    )
    .unwrap();
    assert!(has_error);
}
