use crate::preset::{DEFAULT_ERROR, DEFAULT_WARN};

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
