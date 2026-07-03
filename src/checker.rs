//! Core file-checking logic: resolves limits and returns a [`FileResult`].
use anyhow::Result;
use glob::Pattern;
use std::path::Path;

use crate::config::Config;
use crate::lines::file_info;
use crate::preset::{DEFAULT_ERROR, DEFAULT_WARN};

/// The outcome of checking a single file against its line limits.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Status {
    /// Line count is within all limits.
    Ok,
    /// Line count exceeds the warn threshold but not the error threshold.
    Warn,
    /// Line count exceeds the error threshold.
    Error,
}

/// The result of checking a single file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileResult {
    /// Whether the file is within limits, at warn level, or at error level.
    pub status: Status,
    /// Number of lines in the file.
    pub lines: usize,
    /// The warn threshold that applied, if any.
    pub warn_limit: Option<usize>,
    /// The error threshold that applied, if any.
    pub error_limit: Option<usize>,
    /// Human-readable hint from the matched rule's `message` field, if set.
    pub message: Option<String>,
}

/// Options controlling how a single file is checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckOptions {
    /// When set, overrides every rule and preset — every file uses this as both its warn and error threshold.
    pub max_lines: Option<usize>,
    /// Warn threshold used when no config rule matches the file. `None` means no warn limit.
    pub fallback_warn: Option<usize>,
    /// Error threshold used when no config rule matches the file. `None` means no error limit.
    pub fallback_error: Option<usize>,
}

impl Default for CheckOptions {
    fn default() -> Self {
        Self {
            max_lines: None,
            fallback_warn: Some(DEFAULT_WARN),
            fallback_error: Some(DEFAULT_ERROR),
        }
    }
}

/// Check a single file and return its [`FileResult`].
///
/// Pass `config` when you have already resolved the applicable [`Config`] for
/// this file. Pass `None` to fall back to `opts.fallback_warn` /
/// `opts.fallback_error` only.
pub fn check_file(path: &Path, config: Option<&Config>, opts: &CheckOptions) -> Result<FileResult> {
    let (lines, ignored) = file_info(path)?;
    if ignored {
        return Ok(FileResult { status: Status::Ok, lines, warn_limit: None, error_limit: None, message: None });
    }
    let (warn_limit, error_limit, warn_message, error_message) = resolve_limits(path, config, opts);
    let status = if error_limit.is_some_and(|l| lines > l) { Status::Error }
        else if warn_limit.is_some_and(|l| lines > l) { Status::Warn }
        else { Status::Ok };
    let message = match status { Status::Error => error_message, Status::Warn => warn_message, Status::Ok => None };
    Ok(FileResult { status, lines, warn_limit, error_limit, message })
}

fn resolve_limits(path: &Path, config: Option<&Config>, opts: &CheckOptions) -> (Option<usize>, Option<usize>, Option<String>, Option<String>) {
    if let Some(max) = opts.max_lines { return (Some(max), Some(max), None, None); }
    if let Some(cfg) = config {
        let s = path.to_string_lossy();
        let path_str = s.strip_prefix("./").unwrap_or(&s);
        for rule in &cfg.rules {
            let Ok(pat) = Pattern::new(&rule.pattern) else { continue };
            let fname = path.file_name().and_then(|f| f.to_str()).is_some_and(|f| pat.matches(f));
            if pat.matches(path_str) || fname {
                return (rule.warn, rule.error, rule.warn_message.clone(), rule.error_message.clone());
            }
        }
    }
    (opts.fallback_warn, opts.fallback_error, None, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn status_ordering() {
        assert!(Status::Error > Status::Warn);
        assert!(Status::Warn > Status::Ok);
    }
}
