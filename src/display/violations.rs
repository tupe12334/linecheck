use super::run;
use anyhow::Result;
use linecheck::{checker::CheckOptions, config::ConfigResolver, result::Status};
use std::path::PathBuf;

/// Print only files that exceed warn or error thresholds.
pub fn print_violations(
    files: &[PathBuf],
    resolver: &mut ConfigResolver,
    opts: &CheckOptions,
    has_error: &mut bool,
) -> Result<()> {
    run(files, resolver, opts, |file, r| {
        if r.status < Status::Warn {
            return;
        }
        let (kind, lim) = if r.status == Status::Error {
            ("error", r.error_limit)
        } else {
            ("warn", r.warn_limit)
        };
        println!(
            "{}: {} lines{}{}",
            file.display(),
            r.lines,
            lim.map_or(String::new(), |t| format!(" ({kind} threshold: {t})")),
            r.message
                .as_deref()
                .map_or(String::new(), |m| format!(" — {m}"))
        );
        if r.status == Status::Error {
            *has_error = true;
        }
    })
}
