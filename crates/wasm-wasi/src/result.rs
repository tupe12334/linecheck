use linecheck::{FileResult, Status};

#[derive(serde::Serialize)]
pub(crate) struct CheckResult {
    status: &'static str,
    lines: usize,
    warn_limit: Option<usize>,
    error_limit: Option<usize>,
    message: Option<String>,
}

impl CheckResult {
    /// Build a result for a malformed `config_yaml` input, since the WASI ABI has
    /// no separate error channel — the host distinguishes it by `status == "error"`.
    pub(crate) fn config_error(message: String) -> Self {
        Self {
            status: "error",
            lines: 0,
            warn_limit: None,
            error_limit: None,
            message: Some(message),
        }
    }
}

impl From<FileResult> for CheckResult {
    fn from(result: FileResult) -> Self {
        Self {
            status: match result.status {
                Status::Ok => "ok",
                Status::Warn => "warn",
                Status::Error => "error",
            },
            lines: result.lines,
            warn_limit: result.warn_limit,
            error_limit: result.error_limit,
            message: result.message,
        }
    }
}
