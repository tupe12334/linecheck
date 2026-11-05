//! Path rendering for glob matching.
//!
//! Patterns in a config are always written with `/`, but Windows paths use
//! `\`, so a pattern like `tests/**` would never match without normalization.
use std::path::Path;

#[cfg(windows)]
pub(crate) fn glob_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(not(windows))]
pub(crate) fn glob_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
