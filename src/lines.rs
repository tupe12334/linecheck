//! Low-level line counting and inline-ignore detection.
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

// Escape ':' as \x3a so this file does not contain the marker as a raw literal
// and accidentally ignore itself when linecheck scans its own source tree.
const IGNORE_MARKER: &[u8] = b"linecheck\x3aignore";

/// Read `path` and return `(line_count, is_ignored)`.
///
/// `is_ignored` is `true` when the file contains the ignore marker anywhere.
pub fn file_info(path: &Path) -> Result<(usize, bool)> {
    let data = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(content_info(&data))
}

/// Compute `(line_count, is_ignored)` directly from in-memory bytes.
///
/// Used by hosts that supply file content directly instead of a filesystem
/// path, such as the WASM bindings (and any future non-Rust bindings).
///
/// Binary files (e.g. images) are treated as ignored: raw newline bytes in
/// compressed/binary data are not meaningful "lines", so counting them
/// against a line-length threshold produces false positives.
#[must_use]
pub fn content_info(data: &[u8]) -> (usize, bool) {
    let ignored = is_binary(data)
        || data
            .windows(IGNORE_MARKER.len())
            .any(|w| w == IGNORE_MARKER);
    (count_newlines(data), ignored)
}

/// Heuristic: a NUL byte anywhere in the first 8000 bytes marks the content
/// as binary. Same heuristic used by git and GNU grep.
#[must_use]
fn is_binary(data: &[u8]) -> bool {
    let sample_len = data.len().min(8000);
    data[..sample_len].contains(&0)
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

#[cfg(test)]
#[path = "lines_tests.rs"]
mod tests;
