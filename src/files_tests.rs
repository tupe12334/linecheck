use super::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn collect_files_direct_file_path_included() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("a.txt");
    fs::write(&path, "hello\n").unwrap();
    let files = collect_files(std::slice::from_ref(&path), &[]);
    assert_eq!(files, vec![path]);
}

#[test]
fn collect_files_direct_file_path_excluded_by_glob() {
    // A "**" pattern matches any path, covering the branch where is_excluded returns true
    // for a direct file argument (root=None path in collect_files).
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("b.txt");
    fs::write(&path, "hello\n").unwrap();
    let files = collect_files(std::slice::from_ref(&path), &["**".into()]);
    assert!(files.is_empty(), "file should be excluded by '**' pattern");
}

#[test]
fn collect_files_skips_hidden_entries() {
    // Having a hidden file forces is_hidden to evaluate `s.len() > 1` (the && rhs).
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("visible.txt"), "visible\n").unwrap();
    fs::write(dir.path().join(".hidden"), "hidden\n").unwrap();
    let files = collect_files(&[dir.path().to_path_buf()], &[]);
    assert!(files.iter().any(|f| f.ends_with("visible.txt")));
    assert!(files.iter().all(|f| {
        !f.file_name().and_then(|n| n.to_str()).is_some_and(|s| s.starts_with('.'))
    }));
}

