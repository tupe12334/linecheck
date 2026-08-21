//! Raw and blank-line-aware newline counting.

/// Count logical lines in raw file bytes.
///
/// A file with no trailing newline has its last line counted anyway, so
/// `"hello\nworld"` returns 2 just like `"hello\nworld\n"`.
#[must_use]
pub fn count_newlines(data: &[u8]) -> usize {
    if data.is_empty() {
        return 0;
    }
    let newlines = data.iter().filter(|&&b| b == b'\n').count();
    if data.last() == Some(&b'\n') {
        newlines
    } else {
        newlines + 1
    }
}

/// Count lines, optionally excluding blank/whitespace-only lines.
///
/// A line is blank when it is empty or contains only whitespace once split
/// on `\n` (a trailing `\r` on CRLF files is whitespace too, so it doesn't
/// make a line non-blank).
#[must_use]
pub fn count_lines(data: &[u8], skip_whitespace: bool) -> usize {
    if !skip_whitespace {
        return count_newlines(data);
    }
    if data.is_empty() {
        return 0;
    }
    let mut lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
    if data.last() == Some(&b'\n') {
        lines.pop();
    }
    lines
        .iter()
        .filter(|line| !line.iter().all(u8::is_ascii_whitespace))
        .count()
}

#[cfg(test)]
#[path = "count_tests.rs"]
mod tests;
