use glob::Pattern;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn collect_files(paths: &[PathBuf], exclude: &[String]) -> Vec<PathBuf> {
    let patterns: Vec<Pattern> = exclude.iter().filter_map(|p| Pattern::new(p).ok()).collect();
    let mut files = Vec::new();
    for path in paths {
        if path.is_file() {
            if !is_excluded(path, &patterns) { files.push(path.clone()); }
        } else if path.is_dir() {
            for entry in WalkDir::new(path).follow_links(false).into_iter()
                .filter_entry(|e| !is_hidden(e.path()) && !is_excluded(e.path(), &patterns))
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                files.push(entry.into_path());
            }
        }
    }
    files
}

fn is_hidden(path: &Path) -> bool {
    path.components().any(|c| {
        c.as_os_str().to_str().map_or(false, |s| s.starts_with('.') && s.len() > 1)
    })
}

fn is_excluded(path: &Path, patterns: &[Pattern]) -> bool {
    let s = path.to_string_lossy();
    let normalized = s.strip_prefix("./").unwrap_or(&s);
    patterns.iter().any(|p| p.matches(normalized) || p.matches(&s))
}
