//! File collection: walks paths and applies exclude patterns.
mod filters;
use filters::excluded;
use glob::Pattern;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Collect all files reachable from `paths`, skipping excluded or hidden paths.
pub fn collect_files(paths: &[PathBuf], exclude: &[String]) -> Vec<PathBuf> {
    let pats: Vec<Pattern> = exclude
        .iter()
        .filter_map(|p| Pattern::new(p).ok())
        .collect();
    let mut files = Vec::new();
    for path in paths {
        if path.is_file() {
            if !excluded(path, None, &pats) {
                files.push(path.clone());
            }
        } else if path.is_dir() {
            for e in WalkDir::new(path)
                .follow_links(false)
                .into_iter()
                .filter_entry(|e| {
                    (e.depth() == 0 || !hidden(e.path())) && !excluded(e.path(), Some(path), &pats)
                })
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                files.push(e.into_path());
            }
        } else {
            eprintln!("Warning: path not found: {}", path.display());
        }
    }
    files
}

fn hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|s| s.starts_with('.') && s.len() > 1)
}

#[cfg(test)]
#[path = "../files_tests.rs"]
mod tests;
