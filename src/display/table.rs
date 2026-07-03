use linecheck::result::Status;

pub(super) struct Row {
    pub path: String,
    pub lines: usize,
    pub limit: usize,
    pub status: Status,
}

pub(super) fn digits(n: usize) -> usize {
    n.checked_ilog10().unwrap_or(0) as usize + 1
}

pub(super) fn print_table(rows: &[Row]) {
    let pw = rows.iter().map(|r| r.path.len()).max().unwrap_or(0);
    let (lw, tw) = (
        rows.iter().map(|r| digits(r.lines)).max().unwrap_or(0),
        rows.iter().map(|r| digits(r.limit)).max().unwrap_or(0),
    );
    for row in rows {
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
}
