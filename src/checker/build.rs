use super::{CheckOptions, resolve};
use crate::config::Config;
use crate::result::{FileResult, Status};
use std::path::Path;

pub(super) fn build_result(
    path: &Path,
    lines: usize,
    ignored: bool,
    config: Option<&Config>,
    opts: &CheckOptions,
) -> FileResult {
    if ignored {
        return FileResult {
            status: Status::Ok,
            lines,
            warn_limit: None,
            error_limit: None,
            message: None,
        };
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
    FileResult {
        status,
        lines,
        warn_limit,
        error_limit,
        message,
    }
}
