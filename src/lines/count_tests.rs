use super::*;

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
fn skip_whitespace_false_counts_blank_lines() {
    // Default behavior is unchanged when the flag is unset.
    assert_eq!(count_lines(b"a\n\nb\n", false), 3);
}

#[test]
fn skip_whitespace_excludes_blank_lines() {
    assert_eq!(count_lines(b"a\n\nb\n", true), 2);
}

#[test]
fn skip_whitespace_excludes_whitespace_only_lines() {
    assert_eq!(count_lines(b"a\n   \n\t\nb\n", true), 2);
}

#[test]
fn skip_whitespace_on_empty_data() {
    assert_eq!(count_lines(b"", true), 0);
}

#[test]
fn skip_whitespace_no_trailing_newline() {
    assert_eq!(count_lines(b"a\n\nb", true), 2);
}

#[test]
fn skip_whitespace_matches_issue_example() {
    // 2 real lines + 8 blank padding lines = 10 raw lines, 2 counted lines.
    let mut data = String::from("real1\nreal2\n");
    for _ in 0..8 {
        data.push('\n');
    }
    assert_eq!(count_newlines(data.as_bytes()), 10);
    assert_eq!(count_lines(data.as_bytes(), true), 2);
}
