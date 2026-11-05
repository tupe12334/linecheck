//! Line-counting variants that exclude blank/whitespace-only lines.
use super::{count_newlines, is_ignored};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Read `path` and return `(line_count, is_ignored)`, optionally excluding
/// blank and whitespace-only lines from the count. See
/// [`content_info_with_options`].
pub fn file_info_with_options(path: &Path, skip_whitespace: bool) -> Result<(usize, bool)> {
    let data = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(content_info_with_options(&data, skip_whitespace))
}

/// Compute `(line_count, is_ignored)`, optionally excluding blank and
/// whitespace-only lines from the count. Pass `skip_whitespace: false` for
/// the same behavior as [`super::content_info`].
#[must_use]
pub fn content_info_with_options(data: &[u8], skip_whitespace: bool) -> (usize, bool) {
    let lines = if skip_whitespace {
        count_non_blank_lines(data)
    } else {
        count_newlines(data)
    };
    (lines, is_ignored(data))
}

/// Count logical lines in raw file bytes, excluding blank and
/// whitespace-only lines.
///
/// Follows the same line-splitting rules as [`count_newlines`] — a trailing
/// newline does not introduce an extra (empty) line — but a line consisting
/// only of ASCII whitespace (spaces, tabs, `\r`, ...) is not counted.
#[must_use]
pub fn count_non_blank_lines(data: &[u8]) -> usize {
    data.split(|&b| b == b'\n')
        .filter(|line| !line.iter().all(u8::is_ascii_whitespace))
        .count()
}
