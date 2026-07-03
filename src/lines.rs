use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const IGNORE_MARKER: &[u8] = b"linecheck:ignore";

pub fn file_info(path: &Path) -> Result<(usize, bool)> {
    let data = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let ignored = data.windows(IGNORE_MARKER.len()).any(|w| w == IGNORE_MARKER);
    Ok((count_newlines(&data), ignored))
}

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
        assert!(count_newlines(b"// linecheck:ignore\nfn main() {}") > 0);
    }
}
