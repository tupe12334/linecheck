//! Core file-checking logic: resolves limits and returns a [`FileResult`].
mod build;
mod options;
mod resolve;
use crate::config::Config;
use crate::lines::{content_info_with_options, file_info_with_options};
use crate::result::FileResult;
use anyhow::Result;
use build::build_result;
pub use options::CheckOptions;
use std::path::Path;

/// Check a single file; pass `None` for `config` to fall back to the thresholds in `opts`.
pub fn check_file(path: &Path, config: Option<&Config>, opts: &CheckOptions) -> Result<FileResult> {
    let (lines, ignored) = file_info_with_options(path, opts.skip_whitespace)?;
    Ok(build_result(path, lines, ignored, config, opts))
}

/// Check in-memory content against the rule matching `path`, with no filesystem access.
///
/// `path` is used only for glob-pattern matching — it does not need to exist on disk.
/// This is the entry point for hosts that supply content directly rather than a
/// readable file, such as the WASM bindings (and any future non-Rust bindings).
#[must_use]
pub fn check_content(
    path: &Path,
    content: &[u8],
    config: Option<&Config>,
    opts: &CheckOptions,
) -> FileResult {
    let (lines, ignored) = content_info_with_options(content, opts.skip_whitespace);
    build_result(path, lines, ignored, config, opts)
}

#[cfg(test)]
#[path = "../checker_tests.rs"]
mod tests;
