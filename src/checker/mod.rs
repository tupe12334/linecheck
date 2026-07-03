//! Core file-checking logic: resolves limits and returns a [`FileResult`].
mod options;
mod resolve;
use crate::config::Config;
use crate::lines::file_info;
use crate::result::{FileResult, Status};
use anyhow::Result;
pub use options::CheckOptions;
use std::path::Path;

/// Check a single file; pass `None` for `config` to fall back to the thresholds in `opts`.
pub fn check_file(path: &Path, config: Option<&Config>, opts: &CheckOptions) -> Result<FileResult> {
    let (lines, ignored) = file_info(path)?;
    if ignored {
        return Ok(FileResult {
            status: Status::Ok,
            lines,
            warn_limit: None,
            error_limit: None,
            message: None,
        });
    }
    let (warn_limit, error_limit, warn_message, error_message) =
        resolve::resolve_limits(path, config, opts);
    let status = if error_limit.is_some_and(|l| lines > l) {
        Status::Error
    } else if warn_limit.is_some_and(|l| lines > l) {
        Status::Warn
    } else {
        Status::Ok
    };
    let message = match status {
        Status::Error => error_message,
        Status::Warn => warn_message,
        Status::Ok => None,
    };
    Ok(FileResult {
        status,
        lines,
        warn_limit,
        error_limit,
        message,
    })
}

#[cfg(test)]
#[path = "../checker_tests.rs"]
mod tests;
