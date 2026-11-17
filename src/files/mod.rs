//! File collection: walks paths and applies exclude patterns.
mod filters;
use filters::excluded;
use glob::Pattern;
use ignore::WalkBuilder;
use std::path::PathBuf;

/// Collect all files reachable from `paths`, skipping excluded, hidden, or
/// `.gitignore`d paths.
#[must_use]
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
            let root = path.clone();
            let pats = pats.clone();
            for e in WalkBuilder::new(path)
                .follow_links(false)
                .filter_entry(move |e| !excluded(e.path(), Some(&root), &pats))
                .build()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_some_and(|t| t.is_file()))
            {
                files.push(e.into_path());
            }
        } else {
            eprintln!("Warning: path not found: {}", path.display());
        }
    }
    files
}

#[cfg(test)]
#[path = "../files_tests.rs"]
mod tests;
