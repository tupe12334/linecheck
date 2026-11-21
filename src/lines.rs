//! Low-level line counting and inline-ignore detection.
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

mod count;
use count::count_lines;
pub use count::count_newlines;

// Escape ':' as \x3a so this file does not contain the marker as a raw literal
// and accidentally ignore itself when linecheck scans its own source tree.
const IGNORE_MARKER: &[u8] = b"linecheck\x3aignore";

/// Read `path` and return `(line_count, is_ignored)`.
///
/// `is_ignored` is `true` when the file contains the ignore marker anywhere.
/// When `skip_whitespace` is `true`, blank/whitespace-only lines are excluded
/// from the count.
pub fn file_info(path: &Path, skip_whitespace: bool) -> Result<(usize, bool)> {
    let data = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(content_info(&data, skip_whitespace))
}

/// Compute `(line_count, is_ignored)` directly from in-memory bytes (used by
/// WASM bindings). Binary content is treated as ignored — its raw newline
/// bytes aren't meaningful lines.
#[must_use]
pub fn content_info(data: &[u8], skip_whitespace: bool) -> (usize, bool) {
    let ignored = content_inspector::inspect(data).is_binary()
        || data
            .windows(IGNORE_MARKER.len())
            .any(|w| w == IGNORE_MARKER);
    (count_lines(data, skip_whitespace), ignored)
}

#[cfg(test)]
#[path = "lines_tests.rs"]
mod tests;
