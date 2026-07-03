use std::fs;
use std::path::Path;
use tempfile::TempDir;

use linecheck::{check_file, collect_files, load_config, CheckOptions, Config, Preset, Rule, Status};

fn write(dir: &Path, name: &str, content: &str) -> std::path::PathBuf {
    let p = dir.join(name);
    fs::write(&p, content).unwrap();
    p
}

fn cfg(warn: usize, error: usize) -> Config {
    Config {
        rules: vec![Rule { pattern: "**/*.txt".into(), warn: Some(warn), error: Some(error) }],
        exclude: vec![],
    }
}

fn opts_unlimited() -> CheckOptions {
    CheckOptions { max_lines: None, fallback_warn: None, fallback_error: None }
}

#[test]
fn ok_file() {
    let dir = TempDir::new().unwrap();
    let path = write(dir.path(), "small.txt", "line1\nline2\n");
    let r = check_file(&path, Some(&cfg(10, 20)), &opts_unlimited()).unwrap();
    assert_eq!(r.status, Status::Ok);
    assert_eq!(r.lines, 2);
}

#[test]
fn warn_file() {
    let dir = TempDir::new().unwrap();
    let content = (0..5).map(|i| format!("line{i}\n")).collect::<String>();
    let path = write(dir.path(), "medium.txt", &content);
    let r = check_file(&path, Some(&cfg(3, 10)), &opts_unlimited()).unwrap();
    assert_eq!(r.status, Status::Warn);
}

#[test]
fn error_file() {
    let dir = TempDir::new().unwrap();
    let content = (0..15).map(|i| format!("line{i}\n")).collect::<String>();
    let path = write(dir.path(), "big.txt", &content);
    let r = check_file(&path, Some(&cfg(5, 10)), &opts_unlimited()).unwrap();
    assert_eq!(r.status, Status::Error);
}

#[test]
fn ignored_file() {
    let dir = TempDir::new().unwrap();
    let content = (0..20).map(|i| format!("line{i}\n")).collect::<String>()
        + "# linecheck:ignore\n";
    let path = write(dir.path(), "ignored.txt", &content);
    let r = check_file(&path, Some(&cfg(5, 10)), &opts_unlimited()).unwrap();
    assert_eq!(r.status, Status::Ok);
}

#[test]
fn max_lines_override() {
    let dir = TempDir::new().unwrap();
    let content = (0..5).map(|i| format!("line{i}\n")).collect::<String>();
    let path = write(dir.path(), "file.txt", &content);
    let opts = CheckOptions { max_lines: Some(3), fallback_warn: None, fallback_error: None };
    let r = check_file(&path, None, &opts).unwrap();
    assert_eq!(r.status, Status::Error);
}

#[test]
fn fallback_defaults_apply_when_no_rule_matches() {
    let dir = TempDir::new().unwrap();
    let content = (0..210).map(|i| format!("line{i}\n")).collect::<String>();
    let path = write(dir.path(), "file.txt", &content);
    let r = check_file(&path, None, &CheckOptions::default()).unwrap();
    assert_eq!(r.status, Status::Warn); // 210 > 200 default warn
}

#[test]
fn preset_strict() {
    let (warn, error) = Preset::Strict.limits();
    assert_eq!(warn, Some(100));
    assert_eq!(error, Some(100));
}

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
