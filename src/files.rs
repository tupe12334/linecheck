//! File collection: walks paths and applies exclude patterns.
use glob::Pattern;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Collect all files reachable from `paths`, skipping anything that matches
/// an `exclude` glob pattern or that lives inside a hidden directory.
pub fn collect_files(paths: &[PathBuf], exclude: &[String]) -> Vec<PathBuf> {
    let patterns: Vec<Pattern> = exclude.iter().filter_map(|p| Pattern::new(p).ok()).collect();
    let mut files = Vec::new();
    for path in paths {
        if path.is_file() {
            if !is_excluded(path, None, &patterns) { files.push(path.clone()); }
        } else if path.is_dir() {
            for entry in WalkDir::new(path).follow_links(false).into_iter()
                .filter_entry(|e| (e.depth() == 0 || !is_hidden(e.path())) && !is_excluded(e.path(), Some(path), &patterns))
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
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|s| s.starts_with('.') && s.len() > 1)
}

fn is_excluded(path: &Path, root: Option<&Path>, patterns: &[Pattern]) -> bool {
    // Match relative to root when available (the common case for CLI usage)
    if let Some(root) = root
        && let Ok(rel) = path.strip_prefix(root) {
            let s = rel.to_string_lossy();
            if patterns.iter().any(|p| p.matches(&s)) { return true; }
        }
    let s = path.to_string_lossy();
    let normalized = s.strip_prefix("./").unwrap_or(&s);
    patterns.iter().any(|p| p.matches(normalized) || p.matches(&s))
}
