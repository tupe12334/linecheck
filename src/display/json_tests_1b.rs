use super::json::print_json;
use super::test_helpers::{make_file, resolver, with_limits};
use tempfile::TempDir;
#[test]
fn print_json_warn_violation() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "medium.txt", 8);
    let mut has_error = false;
    print_json(
        &[path],
        &mut resolver(),
        &with_limits(5, 20),
        false,
        &mut has_error,
    );
    assert!(!has_error);
}
#[test]
fn print_json_status_mode_includes_ok_files() {
    let dir = TempDir::new().unwrap();
    let path = make_file(&dir, "file.txt", 5);
    let mut has_error = false;
    print_json(
        &[path],
        &mut resolver(),
        &with_limits(10, 20),
        true,
        &mut has_error,
    );
    assert!(!has_error);
}
