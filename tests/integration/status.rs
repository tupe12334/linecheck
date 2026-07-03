use tempfile::TempDir;

use linecheck::{check_file, Status};

use super::helpers::{cfg, opts_unlimited, write};

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
        + "# linecheck\x3aignore\n"; // \x3a = ':' — avoids this file self-ignoring
    let path = write(dir.path(), "ignored.txt", &content);
    let r = check_file(&path, Some(&cfg(5, 10)), &opts_unlimited()).unwrap();
    assert_eq!(r.status, Status::Ok);
}

#[test]
fn threshold_is_exclusive() {
    // The limit is a strict upper bound: exactly N lines is Ok; N+1 triggers warn.
    let dir = TempDir::new().unwrap();
    let exactly = (0..3).map(|i| format!("line{i}\n")).collect::<String>(); // 3 lines
    let path = write(dir.path(), "file.txt", &exactly);
    let at = check_file(&path, Some(&cfg(3, 10)), &opts_unlimited()).unwrap();
    assert_eq!(at.status, Status::Ok); // exactly at warn limit → still Ok

    let over = (0..4).map(|i| format!("line{i}\n")).collect::<String>(); // 4 lines
    let path2 = write(dir.path(), "file2.txt", &over);
    let above = check_file(&path2, Some(&cfg(3, 10)), &opts_unlimited()).unwrap();
    assert_eq!(above.status, Status::Warn); // one over → Warn
}
