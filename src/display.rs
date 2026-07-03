use std::path::PathBuf;
use anyhow::Result;

use crate::checker::{FileResult, Status, check_file};
use crate::config::Config;

pub fn print_violations(files: &[PathBuf], config: &Config, max_lines: Option<usize>, has_error: &mut bool) -> Result<()> {
    for file in files {
        match check_file(file, config, max_lines) {
            Ok(FileResult { status, lines, error_limit, warn_limit }) if status >= Status::Warn => {
                let (kind, threshold) = if status == Status::Error { ("error", error_limit) } else { ("warn", warn_limit) };
                let limit = threshold.map_or(String::new(), |t| format!(" ({kind} threshold: {t})"));
                println!("{}: {lines} lines{limit}", file.display());
                if status == Status::Error { *has_error = true; }
            }
            Err(e) => eprintln!("Error: {}", e),
            _ => {}
        }
    }
    Ok(())
}

pub fn print_status(files: &[PathBuf], config: &Config, max_lines: Option<usize>, has_error: &mut bool) -> Result<()> {
    struct Row { path: String, lines: usize, limit: usize, status: Status }
    let rows: Vec<Row> = files.iter().filter_map(|f| {
        let r = check_file(f, config, max_lines).ok()?;
        let limit = r.error_limit.or(r.warn_limit)?;
        if r.status == Status::Error { *has_error = true; }
        Some(Row { path: f.display().to_string(), lines: r.lines, limit, status: r.status })
    }).collect();
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

fn digits(n: usize) -> usize {
    if n == 0 { return 1; }
    let mut d = 0; let mut x = n; while x > 0 { d += 1; x /= 10; } d
}
