//! Formatted output: violations-only, `--status` table, and `--json`.
use std::path::PathBuf;
use anyhow::Result;

use linecheck::checker::{check_file, CheckOptions};
use linecheck::result::{FileResult, Status};
use linecheck::config::ConfigResolver;
fn run<F: FnMut(&PathBuf, FileResult)>(files: &[PathBuf], resolver: &mut ConfigResolver, opts: &CheckOptions, mut each: F) -> Result<()> {
    for file in files {
        match check_file(file, resolver.resolve(file).as_ref(), opts) {
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
        let hint = r.message.as_deref().map_or(String::new(), |m| format!(" — {m}"));
        println!("{}: {} lines{tag}{hint}", file.display(), r.lines);
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
        let tag = if row.status == Status::Error { "[ERROR]".to_owned() }
            else if row.status == Status::Warn { "[WARN]".to_owned() }
            else { format!("{}%", if row.limit > 0 { row.lines * 100 / row.limit } else { 0 }) };
        println!("{:<pw$}  {:>lw$} / {:<tw$}  {}", row.path, row.lines, row.limit, tag);
    }
    Ok(())
}

/// Print results as a JSON array (`--json`). Violations mode omits ok files; `--status` includes all.
pub fn print_json(files: &[PathBuf], resolver: &mut ConfigResolver, opts: &CheckOptions, status_mode: bool, has_error: &mut bool) -> Result<()> {
    let mut items: Vec<String> = Vec::new();
    run(files, resolver, opts, |file, r| {
        if !status_mode && r.status < Status::Warn { return; }
        let Some(lim) = r.error_limit.or(r.warn_limit) else { return };
        if r.status == Status::Error { *has_error = true; }
        let pct = if lim > 0 { r.lines * 100 / lim } else { 0 };
        let st = match r.status { Status::Error => "error", Status::Warn => "warn", Status::Ok => "ok" };
        let json = |s: &str| serde_json::to_string(s).unwrap_or_else(|_| "null".into());
        let msg = r.message.as_deref().map_or(String::new(), |m| format!(r#","message":{}"#, json(m)));
        items.push(format!(r#"  {{"file":{f},"lines":{l},"limit":{lim},"percent":{pct},"status":"{st}"{msg}}}"#,
            f = json(&file.display().to_string()), l = r.lines));
    })?;
    println!("{}", if items.is_empty() { "[]".into() } else { format!("[\n{}\n]", items.join(",\n")) });
    Ok(())
}

fn digits(n: usize) -> usize { n.checked_ilog10().unwrap_or(0) as usize + 1 }

