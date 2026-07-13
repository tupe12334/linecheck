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

#[test]
fn binary_content_is_ignored() {
    // PNG signature includes a NUL byte; body full of \n like a real compressed image would produce
    let mut png_like = b"\x89PNG\r\n\x1a\x00".to_vec();
    png_like.extend(std::iter::repeat_n(b'\n', 1000));
    let (_, ignored) = content_info(&png_like);
    assert!(ignored);
}

#[test]
fn text_content_is_not_ignored() {
    let (_, ignored) = content_info(b"hello\nworld\n");
    assert!(!ignored);
}
