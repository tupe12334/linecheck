//! Low-level line counting and inline-ignore detection.
use crate::comments::line_comment_prefix;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

// Escape ':' as \x3a so this file does not contain the marker as a raw literal
// and accidentally ignore itself when linecheck scans its own source tree.
const IGNORE_MARKER: &[u8] = b"linecheck\x3aignore";

/// Read `path` and return `(line_count, is_ignored)`.
///
/// `is_ignored` is `true` when the file contains the ignore marker anywhere.
/// When `skip_comments` is `true`, full-line comments are excluded from the
/// count for languages `linecheck` recognizes by file extension.
pub fn file_info(path: &Path, skip_comments: bool) -> Result<(usize, bool)> {
    let data = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(content_info(path, &data, skip_comments))
}

/// Compute `(line_count, is_ignored)` directly from in-memory bytes (used by
/// WASM bindings). Binary content is treated as ignored — its raw newline
/// bytes aren't meaningful lines. `path` is used only to detect a
/// language's line-comment syntax when `skip_comments` is `true`; it does
/// not need to exist on disk.
#[must_use]
pub fn content_info(path: &Path, data: &[u8], skip_comments: bool) -> (usize, bool) {
    let ignored = content_inspector::inspect(data).is_binary()
        || data
            .windows(IGNORE_MARKER.len())
            .any(|w| w == IGNORE_MARKER);
    let lines = if skip_comments {
        count_lines_skip_comments(path, data)
    } else {
        count_newlines(data)
    };
    (lines, ignored)
}

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
    if data.last() != Some(&b'\n') {
        newlines + 1
    } else {
        newlines
    }
}

/// Count lines, excluding full-line comments when `path`'s extension maps to
/// a known line-comment prefix. Falls back to [`count_newlines`] for
/// unrecognized extensions.
fn count_lines_skip_comments(path: &Path, data: &[u8]) -> usize {
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

#[cfg(test)]
#[path = "lines_tests.rs"]
mod tests;
