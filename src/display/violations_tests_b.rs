use super::test_helpers::{make_file, resolver, with_limits};
use super::violations::print_violations;
use tempfile::TempDir;
#[test]
fn print_violations_warn_does_not_set_error_flag() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "medium.txt", 8);
    let mut has_error = false;
    print_violations(
        &[path],
        &mut resolver(),
        &with_limits(5, 20),
        &mut has_error,
    );
    assert!(!has_error);
}
#[test]
fn print_violations_unreadable_file_does_not_propagate_err() {
    let path = std::path::PathBuf::from("/tmp/linecheck-display-test-nonexistent.txt");
    let mut has_error = false;
    print_violations(
        &[path],
        &mut resolver(),
        &with_limits(10, 20),
        &mut has_error,
    );
}
