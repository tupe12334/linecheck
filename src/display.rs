//! Formatted output: violations-only, `--status` table, and `--json`.
use std::path::PathBuf;
use anyhow::Result;

use crate::checker::{check_file, CheckOptions, FileResult, Status};
use crate::config::ConfigResolver;

fn run<F>(files: &[PathBuf], resolver: &mut ConfigResolver, opts: &CheckOptions, mut each: F) -> Result<()>
where F: FnMut(&PathBuf, FileResult)
{
    for file in files {
        let cfg = resolver.resolve(file);
        match check_file(file, cfg.as_ref(), opts) {
            Ok(r) => each(file, r),
            Err(e) => eprintln!("Error: {e}"),
        }
    }
    Ok(())
}

/// Print only files that exceed warn or error thresholds.
pub fn print_violations(files: &[PathBuf], resolver: &mut ConfigResolver, opts: &CheckOptions, has_error: &mut bool) -> Result<()> {
    run(files, resolver, opts, |file, r| {
        if r.status < Status::Warn { return; }
        let (kind, lim) = if r.status == Status::Error { ("error", r.error_limit) } else { ("warn", r.warn_limit) };
        let tag = lim.map_or(String::new(), |t| format!(" ({kind} threshold: {t})"));
        println!("{}: {} lines{tag}", file.display(), r.lines);
        if r.status == Status::Error { *has_error = true; }
    })
}

/// Print all files with a line-count / limit table (`--status` mode).
pub fn print_status(files: &[PathBuf], resolver: &mut ConfigResolver, opts: &CheckOptions, has_error: &mut bool) -> Result<()> {
    struct Row { path: String, lines: usize, limit: usize, status: Status }
    let mut rows: Vec<Row> = Vec::new();
    run(files, resolver, opts, |file, r| {
        let Some(limit) = r.error_limit.or(r.warn_limit) else { return };
        if r.status == Status::Error { *has_error = true; }
        rows.push(Row { path: file.display().to_string(), lines: r.lines, limit, status: r.status });
    })?;
    let pw = rows.iter().map(|r| r.path.len()).max().unwrap_or(0);
    let lw = rows.iter().map(|r| digits(r.lines)).max().unwrap_or(0);
    let tw = rows.iter().map(|r| digits(r.limit)).max().unwrap_or(0);
    for row in &rows {
        let tag = match row.status {
            Status::Error => "[ERROR]".to_string(),
            Status::Warn  => "[WARN]".to_string(),
            Status::Ok    => format!("{}%", row.lines * 100 / row.limit),
        };
        println!("{:<pw$}  {:>lw$} / {:<tw$}  {}", row.path, row.lines, row.limit, tag);
    }
    Ok(())
}

/// Print results as a JSON array (`--json` flag). In violations mode only
/// warn/error files are included; in status mode all files are included.
pub fn print_json(files: &[PathBuf], resolver: &mut ConfigResolver, opts: &CheckOptions, status_mode: bool, has_error: &mut bool) -> Result<()> {
    let mut items: Vec<String> = Vec::new();
    run(files, resolver, opts, |file, r| {
        if !status_mode && r.status < Status::Warn { return; }
        let limit = r.error_limit.or(r.warn_limit);
        let Some(lim) = limit else { return };
        if r.status == Status::Error { *has_error = true; }
        let pct = r.lines * 100 / lim;
        let st = match r.status { Status::Error => "error", Status::Warn => "warn", Status::Ok => "ok" };
        items.push(format!(
            r#"  {{"file":{f},"lines":{l},"limit":{lim},"percent":{pct},"status":"{st}"}}"#,
            f = json_str(&file.display().to_string()), l = r.lines,
        ));
    })?;
    println!("[{}{}]", if items.is_empty() { "" } else { "\n" }, items.join(",\n"));
    Ok(())
}

fn digits(n: usize) -> usize {
    if n == 0 { return 1; }
    let mut d = 0; let mut x = n; while x > 0 { d += 1; x /= 10; } d
}

fn json_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
