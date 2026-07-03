//! The [`Rule`] type — a single pattern/limit pair from a config file.
use serde::Deserialize;

/// A single pattern/limit pair inside a [`crate::Config`].
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Rule {
    /// Glob pattern matched against file paths (e.g. `"**/*.rs"`).
    pub pattern: String,
    /// Line count at which a warning is emitted. `None` means no warn limit.
    pub warn: Option<usize>,
    /// Message printed alongside a warn violation. `None` means no message.
    #[serde(default)]
    pub warn_message: Option<String>,
    /// Line count at which an error is emitted. `None` means no error limit.
    pub error: Option<usize>,
    /// Message printed alongside an error violation. `None` means no message.
    #[serde(default)]
    pub error_message: Option<String>,
}
