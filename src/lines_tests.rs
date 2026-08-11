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
    let result = file_info(Path::new("/tmp/linecheck-test-nonexistent-xyz.txt"), false);
    assert!(result.is_err());
}

#[test]
fn binary_content_is_ignored() {
    // PNG signature includes a NUL byte; body full of \n like a real compressed image would produce
    let mut png_like = b"\x89PNG\r\n\x1a\x00".to_vec();
    png_like.extend(std::iter::repeat_n(b'\n', 1000));
    let (_, ignored) = content_info(Path::new("img.png"), &png_like, false);
    assert!(ignored);
}

#[test]
fn text_content_is_not_ignored() {
    let (_, ignored) = content_info(Path::new("hello.txt"), b"hello\nworld\n", false);
    assert!(!ignored);
}

#[test]
fn skip_comments_unset_counts_comment_lines() {
    let (lines, _) = content_info(Path::new("main.rs"), b"// a\nfn x() {}\n", false);
    assert_eq!(lines, 2);
}

#[test]
fn skip_comments_excludes_full_line_comments() {
    let (lines, _) = content_info(Path::new("main.rs"), b"// a\nfn x() {}\n// b\n", true);
    assert_eq!(lines, 1);
}

#[test]
fn skip_comments_keeps_inline_trailing_comments() {
    // Only *full-line* comments are stripped — a trailing `// x` after real
    // code is not a comment-only line, so it's still counted.
    let (lines, _) = content_info(Path::new("main.rs"), b"let x = 1; // one\n", true);
    assert_eq!(lines, 1);
}

#[test]
fn skip_comments_ignores_leading_indentation() {
    let (lines, _) = content_info(Path::new("main.rs"), b"fn x() {\n    // note\n}\n", true);
    assert_eq!(lines, 2);
}

#[test]
fn skip_comments_uses_hash_for_python() {
    let (lines, _) = content_info(Path::new("build.py"), b"# note\nprint(1)\n", true);
    assert_eq!(lines, 1);
}

#[test]
fn skip_comments_unknown_extension_counts_all_lines() {
    let (lines, _) = content_info(Path::new("data.bin"), b"# note\nx\n", true);
    assert_eq!(lines, 2);
}

#[test]
fn skip_comments_on_empty_data() {
    let (lines, _) = content_info(Path::new("main.rs"), b"", true);
    assert_eq!(lines, 0);
}
