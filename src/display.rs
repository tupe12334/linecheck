use anyhow::Result;
use linecheck::{
    checker::{CheckOptions, check_file},
    config::ConfigResolver,
    result::{FileResult, Status},
};
use std::path::PathBuf;

fn run<F: FnMut(&PathBuf, FileResult)>(
    files: &[PathBuf],
    resolver: &mut ConfigResolver,
    opts: &CheckOptions,
    mut each: F,
) -> Result<()> {
    for f in files {
        match check_file(f, resolver.resolve(f).as_ref(), opts) {
            Ok(r) => each(f, r),
            Err(e) => eprintln!("Error: {e}"),
        }
    }
    Ok(())
}
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
/// Print all files with a line-count / limit table (`--status` mode).
pub fn print_status(
    files: &[PathBuf],
    resolver: &mut ConfigResolver,
    opts: &CheckOptions,
    has_error: &mut bool,
) -> Result<()> {
    struct Row {
        path: String,
        lines: usize,
        limit: usize,
        status: Status,
    }
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
    let pw = rows.iter().map(|r| r.path.len()).max().unwrap_or(0);
    let (lw, tw) = (
        rows.iter().map(|r| digits(r.lines)).max().unwrap_or(0),
        rows.iter().map(|r| digits(r.limit)).max().unwrap_or(0),
    );
    for row in &rows {
        let pct = if row.limit > 0 {
            row.lines * 100 / row.limit
        } else {
            0
        };
        let tag = match row.status {
            Status::Error => "[ERROR]".to_owned(),
            Status::Warn => "[WARN]".to_owned(),
            _ => format!("{pct}%"),
        };
        println!(
            "{:<pw$}  {:>lw$} / {:<tw$}  {}",
            row.path, row.lines, row.limit, tag
        );
    }
    Ok(())
}
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
        let (pct, st) = (
            if lim > 0 { r.lines * 100 / lim } else { 0 },
            match r.status {
                Status::Error => "error",
                Status::Warn => "warn",
                Status::Ok => "ok",
            },
        );
        let msg = r.message.as_deref().map_or(String::new(), |m| {
            format!(
                r#","message":{}"#,
                serde_json::to_string(m).unwrap_or_else(|_| "null".into())
            )
        });
        items.push(format!(
            r#"  {{"file":{f},"lines":{l},"limit":{lim},"percent":{pct},"status":"{st}"{msg}}}"#,
            f = serde_json::to_string(&file.display().to_string())
                .unwrap_or_else(|_| "null".into()),
            l = r.lines
        ));
    })?;
    println!(
        "{}",
        if items.is_empty() {
            "[]".into()
        } else {
            format!("[\n{}\n]", items.join(",\n"))
        }
    );
    Ok(())
}
fn digits(n: usize) -> usize {
    n.checked_ilog10().unwrap_or(0) as usize + 1
}
