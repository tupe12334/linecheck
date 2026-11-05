//! Low-level line counting and inline-ignore detection.
mod count;
mod skip_comments;

pub use count::count_newlines;

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
        skip_comments::count_lines_skip_comments(path, data)
    } else {
        count_newlines(data)
    };
    (lines, ignored)
}

#[cfg(test)]
#[path = "../lines_tests.rs"]
mod tests;
