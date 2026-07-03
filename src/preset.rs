//! Built-in strictness presets.

/// Built-in strictness presets applied when no config rule matches a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Preset {
    /// Warn and error when a file exceeds 100 lines.
    Strict,
    /// Warn when a file exceeds 200 lines, error when it exceeds 400 (the built-in default).
    Default,
    /// Warn and error when a file exceeds 400 lines.
    Loose,
    /// Unlimited — all limits disabled.
    Free,
}

/// Default warn limit applied when no config is found anywhere.
pub const DEFAULT_WARN: usize = 200;
/// Default error limit applied when no config is found anywhere.
pub const DEFAULT_ERROR: usize = 400;

impl Preset {
    /// Returns `(warn_limit, error_limit)`; `None` means unlimited.
    pub fn limits(self) -> (Option<usize>, Option<usize>) {
        match self {
            Preset::Strict => (Some(100), Some(100)),
            Preset::Default => (Some(DEFAULT_WARN), Some(DEFAULT_ERROR)),
            Preset::Loose => (Some(400), Some(400)),
            Preset::Free => (None, None),
        }
    }
}

#[cfg(test)]
#[path = "preset_tests.rs"]
mod tests;
