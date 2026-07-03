use anyhow::Result;
use glob::Pattern;
use std::path::Path;

use crate::config::Config;
use crate::lines::file_info;
use crate::preset::{DEFAULT_ERROR, DEFAULT_WARN};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Status { Ok, Warn, Error }

pub struct FileResult {
    pub status: Status,
    pub lines: usize,
    pub warn_limit: Option<usize>,
    pub error_limit: Option<usize>,
}

/// Options controlling how a single file is checked.
pub struct CheckOptions {
    /// `--max-lines` — overrides all rules and presets.
    pub max_lines: Option<usize>,
    /// Warn fallback when no rule matches (from preset or built-in default).
    pub fallback_warn: Option<usize>,
    /// Error fallback when no rule matches (from preset or built-in default).
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

pub fn check_file(path: &Path, config: Option<&Config>, opts: &CheckOptions) -> Result<FileResult> {
    let (lines, ignored) = file_info(path)?;
    if ignored {
        return Ok(FileResult { status: Status::Ok, lines, warn_limit: None, error_limit: None });
    }
    let (warn_limit, error_limit) = resolve_limits(path, config, opts);
    let status = if error_limit.is_some_and(|l| lines > l) { Status::Error }
        else if warn_limit.is_some_and(|l| lines > l) { Status::Warn }
        else { Status::Ok };
    Ok(FileResult { status, lines, warn_limit, error_limit })
}

fn resolve_limits(path: &Path, config: Option<&Config>, opts: &CheckOptions) -> (Option<usize>, Option<usize>) {
    if let Some(max) = opts.max_lines { return (Some(max), Some(max)); }
    if let Some(cfg) = config {
        let s = path.to_string_lossy();
        let path_str = s.strip_prefix("./").unwrap_or(&s);
        for rule in &cfg.rules {
            let Ok(pat) = Pattern::new(&rule.pattern) else { continue };
            let fname = path.file_name().and_then(|f| f.to_str()).is_some_and(|f| pat.matches(f));
            if pat.matches(path_str) || fname { return (rule.warn, rule.error); }
        }
    }
    (opts.fallback_warn, opts.fallback_error)
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
