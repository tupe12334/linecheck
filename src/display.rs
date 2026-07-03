use anyhow::Result;
use linecheck::{checker::{CheckOptions, check_file}, config::ConfigResolver, result::{FileResult, Status}};
use std::path::PathBuf;

fn run<F: FnMut(&PathBuf, FileResult)>(f: &[PathBuf], rv: &mut ConfigResolver, o: &CheckOptions, mut e: F) {
    for p in f { match check_file(p, rv.resolve(p).as_ref(), o) { Ok(r) => e(p, r), Err(e) => eprintln!("Error: {e}") } }
}
/// Print only files that exceed warn or error thresholds.
pub fn print_violations(f: &[PathBuf], rv: &mut ConfigResolver, o: &CheckOptions, err: &mut bool) -> Result<()> {
    run(f, rv, o, |p, r| {
        if r.status < Status::Warn { return; }
        let (kind, lim) = if r.status == Status::Error { ("error", r.error_limit) } else { ("warn", r.warn_limit) };
        println!("{}: {} lines{}{}", p.display(), r.lines, lim.map_or(String::new(), |t| format!(" ({kind} threshold: {t})")), r.message.as_deref().map_or(String::new(), |m| format!(" — {m}")));
        if r.status == Status::Error { *err = true; }
    }); Ok(())
}
/// Print all files with a line-count / limit table (`--status` mode).
pub fn print_status(f: &[PathBuf], rv: &mut ConfigResolver, o: &CheckOptions, err: &mut bool) -> Result<()> {
    struct Row { path: String, lines: usize, limit: usize, status: Status }
    let mut rows: Vec<Row> = Vec::new();
    run(f, rv, o, |p, r| {
        let Some(lim) = (if r.status == Status::Warn { r.warn_limit.or(r.error_limit) } else { r.error_limit.or(r.warn_limit) }) else { return };
        if r.status == Status::Error { *err = true; }
        rows.push(Row { path: p.display().to_string(), lines: r.lines, limit: lim, status: r.status });
    });
    let (pw, lw, tw) = (rows.iter().map(|r| r.path.len()).max().unwrap_or(0), rows.iter().map(|r| digits(r.lines)).max().unwrap_or(0), rows.iter().map(|r| digits(r.limit)).max().unwrap_or(0));
    for r in &rows {
        let tag = match r.status { Status::Error => "[ERROR]".to_owned(), Status::Warn => "[WARN]".to_owned(), _ => format!("{}%", if r.limit > 0 { r.lines * 100 / r.limit } else { 0 }) };
        println!("{:<pw$}  {:>lw$} / {:<tw$}  {}", r.path, r.lines, r.limit, tag);
    }
    Ok(())
}
/// Print results as a JSON array (`--json`). Violations mode omits ok files; `--status` includes all.
pub fn print_json(f: &[PathBuf], rv: &mut ConfigResolver, o: &CheckOptions, sm: bool, err: &mut bool) -> Result<()> {
    let mut items: Vec<String> = Vec::new();
    run(f, rv, o, |p, r| {
        if !sm && r.status < Status::Warn { return; }
        let Some(lim) = (if r.status == Status::Warn { r.warn_limit.or(r.error_limit) } else { r.error_limit.or(r.warn_limit) }) else { return };
        if r.status == Status::Error { *err = true; }
        let (pct, st) = (if lim > 0 { r.lines * 100 / lim } else { 0 }, match r.status { Status::Error => "error", Status::Warn => "warn", Status::Ok => "ok" });
        let msg = r.message.as_deref().map_or(String::new(), |m| format!(r#","message":{}"#, serde_json::to_string(m).unwrap_or_else(|_| "null".into())));
        items.push(format!(r#"  {{"file":{f},"lines":{l},"limit":{lim},"percent":{pct},"status":"{st}"{msg}}}"#, f = serde_json::to_string(&p.display().to_string()).unwrap_or_else(|_| "null".into()), l = r.lines));
    }); println!("{}", if items.is_empty() { "[]".into() } else { format!("[\n{}\n]", items.join(",\n")) }); Ok(())
}
fn digits(n: usize) -> usize { n.checked_ilog10().unwrap_or(0) as usize + 1 }
#[cfg(test)]
#[path = "display_tests.rs"]
mod tests;
