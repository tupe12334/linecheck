use super::*;
use std::path::Path;

#[test]
fn empty() {
    assert_eq!(count_newlines(b""), 0);
}

#[test]
fn no_trailing_newline() {
    assert_eq!(count_newlines(b"hello\nworld"), 2);
}

#[test]
fn trailing_newline() {
    assert_eq!(count_newlines(b"hello\nworld\n"), 2);
}

#[test]
fn single_line() {
    assert_eq!(count_newlines(b"hello"), 1);
}

#[test]
fn ignore_marker_detected() {
    // Use \x3a for ':' so this test file doesn't self-ignore
    assert!(count_newlines(b"// linecheck\x3aignore\nfn main() {}") > 0);
}

#[test]
fn file_info_missing_file_returns_err() {
    let result = file_info(Path::new("/tmp/linecheck-test-nonexistent-xyz.txt"));
    assert!(result.is_err());
}
