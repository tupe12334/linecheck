use super::run;
use super::table::{Row, print_table};
use anyhow::Result;
use linecheck::{checker::CheckOptions, config::ConfigResolver, result::Status};
use std::path::PathBuf;

/// Print all files with a line-count / limit table (`--status` mode).
pub fn print_status(
    files: &[PathBuf],
    resolver: &mut ConfigResolver,
    opts: &CheckOptions,
    has_error: &mut bool,
) -> Result<()> {
    let mut rows: Vec<Row> = Vec::new();
    run(files, resolver, opts, |file, r| {
        let Some(limit) = (if r.status == Status::Warn {
            r.warn_limit.or(r.error_limit)
        } else {
            r.error_limit.or(r.warn_limit)
        }) else {
            return;
        };
        if r.status == Status::Error {
            *has_error = true;
        }
        rows.push(Row {
            path: file.display().to_string(),
            lines: r.lines,
            limit,
            status: r.status,
        });
    })?;
    print_table(&rows);
    Ok(())
}
