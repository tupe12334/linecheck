use std::fs;
use std::path::Path;
use tempfile::TempDir;

use linecheck::{check_file, collect_files, load_config, Config, Rule, Status};

fn write(dir: &Path, name: &str, content: &str) -> std::path::PathBuf {
    let p = dir.join(name);
    fs::write(&p, content).unwrap();
    p
}

fn config_with_limit(warn: usize, error: usize) -> Config {
    Config {
        rules: vec![Rule { pattern: "**/*.txt".into(), warn: Some(warn), error: Some(error) }],
        exclude: vec![],
    }
}

#[test]
fn ok_file() {
    let dir = TempDir::new().unwrap();
    let path = write(dir.path(), "small.txt", "line1\nline2\n");
    let cfg = config_with_limit(10, 20);
    let r = check_file(&path, &cfg, None).unwrap();
    assert_eq!(r.status, Status::Ok);
    assert_eq!(r.lines, 2);
}

#[test]
fn warn_file() {
    let dir = TempDir::new().unwrap();
    let content = (0..5).map(|i| format!("line{i}\n")).collect::<String>();
    let path = write(dir.path(), "medium.txt", &content);
    let cfg = config_with_limit(3, 10);
    let r = check_file(&path, &cfg, None).unwrap();
    assert_eq!(r.status, Status::Warn);
}

#[test]
fn error_file() {
    let dir = TempDir::new().unwrap();
    let content = (0..15).map(|i| format!("line{i}\n")).collect::<String>();
    let path = write(dir.path(), "big.txt", &content);
    let cfg = config_with_limit(5, 10);
    let r = check_file(&path, &cfg, None).unwrap();
    assert_eq!(r.status, Status::Error);
}

#[test]
fn ignored_file() {
    let dir = TempDir::new().unwrap();
    let content = (0..20).map(|i| format!("line{i}\n")).collect::<String>()
        + "# linecheck:ignore\n";
    let path = write(dir.path(), "ignored.txt", &content);
    let cfg = config_with_limit(5, 10);
    let r = check_file(&path, &cfg, None).unwrap();
    assert_eq!(r.status, Status::Ok);
}

#[test]
fn max_lines_override() {
    let dir = TempDir::new().unwrap();
    let content = (0..5).map(|i| format!("line{i}\n")).collect::<String>();
    let path = write(dir.path(), "file.txt", &content);
    let cfg = Config::default();
    let r = check_file(&path, &cfg, Some(3)).unwrap();
    assert_eq!(r.status, Status::Error);
}

#[test]
fn collect_files_excludes() {
    let dir = TempDir::new().unwrap();
    write(dir.path(), "keep.txt", "hello\n");
    let sub = dir.path().join("skip");
    fs::create_dir(&sub).unwrap();
    write(&sub, "drop.txt", "hello\n");
    let files = collect_files(&[dir.path().to_path_buf()], &["skip/**".into()]);
    eprintln!("TempDir: {}", dir.path().display());
    for f in &files { eprintln!("  file: {}", f.display()); }
    assert!(files.iter().all(|f| !f.to_string_lossy().contains("drop")));
    assert!(files.iter().any(|f| f.to_string_lossy().contains("keep")));
}

#[test]
fn load_config_missing_file() {
    let cfg = load_config(Path::new("nonexistent.yml"));
    assert!(cfg.rules.is_empty());
}
