//! [`FileResult`] and [`Status`] — the output types of a single file check.

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
