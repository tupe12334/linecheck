//! Full-line-comment exclusion behind the `--skip-comments` flag.
use super::count::count_newlines;
use crate::comments::line_comment_prefix;
use std::path::Path;

/// Count lines, excluding full-line comments when `path`'s extension maps to
/// a known line-comment prefix. Falls back to [`count_newlines`] for
/// unrecognized extensions.
pub fn count_lines_skip_comments(path: &Path, data: &[u8]) -> usize {
    let Some(prefix) = line_comment_prefix(path) else {
        return count_newlines(data);
    };
    if data.is_empty() {
        return 0;
    }
    let mut lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
    if data.last() == Some(&b'\n') {
        lines.pop();
    }
    lines
        .iter()
        .filter(|line| {
            let trimmed = trim_start(line);
            !trimmed.starts_with(prefix.as_bytes())
        })
        .count()
}

fn trim_start(line: &[u8]) -> &[u8] {
    let start = line
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(line.len());
    &line[start..]
}
