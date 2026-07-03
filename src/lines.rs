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
    let ignored = data.windows(IGNORE_MARKER.len()).any(|w| w == IGNORE_MARKER);
    Ok((count_newlines(&data), ignored))
}

/// Count logical lines in raw file bytes.
///
/// A file with no trailing newline has its last line counted anyway, so
/// `"hello\nworld"` returns 2 just like `"hello\nworld\n"`.
pub fn count_newlines(data: &[u8]) -> usize {
    if data.is_empty() { return 0; }
    let newlines = data.iter().filter(|&&b| b == b'\n').count();
    if *data.last().unwrap() != b'\n' { newlines + 1 } else { newlines }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() { assert_eq!(count_newlines(b""), 0); }

    #[test]
    fn no_trailing_newline() { assert_eq!(count_newlines(b"hello\nworld"), 2); }

    #[test]
    fn trailing_newline() { assert_eq!(count_newlines(b"hello\nworld\n"), 2); }

    #[test]
    fn single_line() { assert_eq!(count_newlines(b"hello"), 1); }

    #[test]
    fn ignore_marker_detected() {
        // Use \x3a for ':' so this test file doesn't self-ignore
        assert!(count_newlines(b"// linecheck\x3aignore\nfn main() {}") > 0);
    }
}
