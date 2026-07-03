use super::test_helpers::{make_file, unlimited};
use super::violations::print_violations;
use linecheck::config::ConfigResolver;
use std::fs;
use tempfile::TempDir;
#[test]
fn print_violations_message_is_printed() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("linecheck.yml"),
        "rules:\n  - pattern: '*.txt'\n    error: 5\n    error_message: 'too long!'\n",
    )
    .unwrap();
    let path = make_file(&dir, "big.txt", 10);
    let mut has_error = false;
    let mut res = ConfigResolver::new(None, "linecheck.yml");
    print_violations(&[path], &mut res, &unlimited(), &mut has_error);
    assert!(has_error);
}
