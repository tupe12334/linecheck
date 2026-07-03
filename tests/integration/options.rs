use tempfile::TempDir;

use linecheck::{CheckOptions, Preset, Status, check_file};

use super::helpers::write;

#[test]
fn max_lines_override() {
    let dir = TempDir::new().unwrap();
    let content = (0..5).map(|i| format!("line{i}\n")).collect::<String>();
    let path = write(dir.path(), "file.txt", &content);
    let opts = CheckOptions {
        max_lines: Some(3),
        fallback_warn: None,
        fallback_error: None,
    };
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
