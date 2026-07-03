use super::test_helpers::{make_file, resolver, with_limits};
use super::violations::print_violations;
use tempfile::TempDir;
#[test]
fn print_violations_ok_file_is_silent() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "small.txt", 2);
    let mut has_error = false;
    print_violations(
        &[path],
        &mut resolver(),
        &with_limits(10, 20),
        &mut has_error,
    );
    assert!(!has_error);
}
#[test]
fn print_violations_error_sets_flag() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "big.txt", 20);
    let mut has_error = false;
    print_violations(
        &[path],
        &mut resolver(),
        &with_limits(5, 10),
        &mut has_error,
    );
    assert!(has_error);
}
