//! Low-level line counting and inline-ignore detection.
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

#[path = "lines_skip_whitespace.rs"]
mod skip_whitespace;
#[cfg(test)]
#[path = "lines_tests.rs"]
mod tests;
pub use skip_whitespace::{
    content_info_with_options, count_non_blank_lines, file_info_with_options,
};

// Escape ':' as \x3a so this file doesn't contain the marker and self-ignore.
const IGNORE_MARKER: &[u8] = b"linecheck\x3aignore";

/// Read `path` and return `(line_count, is_ignored)`; ignored if the file contains the marker.
pub fn file_info(path: &Path) -> Result<(usize, bool)> {
    let data = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(content_info(&data))
}

/// Compute `(line_count, is_ignored)` from in-memory bytes; binary content is ignored.
#[must_use]
pub fn content_info(data: &[u8]) -> (usize, bool) {
    (count_newlines(data), is_ignored(data))
}

fn is_ignored(data: &[u8]) -> bool {
    content_inspector::inspect(data).is_binary()
        || data
            .windows(IGNORE_MARKER.len())
            .any(|w| w == IGNORE_MARKER)
}

/// Count logical lines; a missing trailing newline still counts the last line.
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
