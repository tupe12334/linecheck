use super::json_helpers::{emit_json, msg_json};
use super::run;
use anyhow::Result;
use linecheck::{checker::CheckOptions, config::ConfigResolver, result::Status};
use std::path::PathBuf;

/// Print results as a JSON array (`--json`). Violations mode omits ok files; `--status` includes all.
pub fn print_json(
    files: &[PathBuf],
    resolver: &mut ConfigResolver,
    opts: &CheckOptions,
    status_mode: bool,
    has_error: &mut bool,
) -> Result<()> {
    let mut items: Vec<String> = Vec::new();
    run(files, resolver, opts, |file, r| {
        if !status_mode && r.status < Status::Warn {
            return;
        }
        let Some(lim) = (if r.status == Status::Warn {
            r.warn_limit.or(r.error_limit)
        } else {
            r.error_limit.or(r.warn_limit)
        }) else {
            return;
        };
        if r.status == Status::Error {
            *has_error = true;
        }
        let pct = if lim > 0 { r.lines * 100 / lim } else { 0 };
        let st = match r.status {
            Status::Error => "error",
            Status::Warn => "warn",
            Status::Ok => "ok",
        };
        let f =
            serde_json::to_string(&file.display().to_string()).unwrap_or_else(|_| "null".into());
        items.push(format!(
            r#"  {{"file":{f},"lines":{},"limit":{lim},"percent":{pct},"status":"{st}"{}}}"#,
            r.lines,
            msg_json(r.message.as_deref())
        ));
    })?;
    emit_json(items);
    Ok(())
}
