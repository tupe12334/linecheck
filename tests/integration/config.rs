use std::fs;
use std::path::Path;

use tempfile::TempDir;

use linecheck::{collect_files, load_config};

use super::helpers::write;

#[test]
fn collect_files_excludes() {
    let dir = TempDir::new().unwrap();
    write(dir.path(), "keep.txt", "hello\n");
    let sub = dir.path().join("skip");
    fs::create_dir(&sub).unwrap();
    write(&sub, "drop.txt", "hello\n");
    let files = collect_files(&[dir.path().to_path_buf()], &["skip/**".into()]);
    assert!(files.iter().all(|f| !f.to_string_lossy().contains("drop")));
    assert!(files.iter().any(|f| f.to_string_lossy().contains("keep")));
}

#[test]
fn load_config_missing_file() {
    let cfg = load_config(Path::new("nonexistent.yml"));
    assert!(cfg.rules.is_empty());
}
