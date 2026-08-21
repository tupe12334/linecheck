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
        skip_whitespace: false,
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
fn skip_whitespace_off_counts_blank_lines() {
    let dir = TempDir::new().unwrap();
    // 2 real lines + 3 blank/whitespace-only lines = 5 raw lines
    let path = write(dir.path(), "file.txt", "a\n\nb\n   \n\t\n");
    let opts = CheckOptions {
        max_lines: Some(2),
        fallback_warn: None,
        fallback_error: None,
        skip_whitespace: false,
    };
    let r = check_file(&path, None, &opts).unwrap();
    assert_eq!(r.lines, 5);
    assert_eq!(r.status, Status::Error); // 5 > 2
}

#[test]
fn skip_whitespace_on_excludes_blank_lines_from_count() {
    let dir = TempDir::new().unwrap();
    // Same content as above, but only 2 non-blank lines
    let path = write(dir.path(), "file.txt", "a\n\nb\n   \n\t\n");
    let opts = CheckOptions {
        max_lines: Some(2),
        fallback_warn: None,
        fallback_error: None,
        skip_whitespace: true,
    };
    let r = check_file(&path, None, &opts).unwrap();
    assert_eq!(r.lines, 2);
    assert_eq!(r.status, Status::Ok); // 2 is not > 2
}

#[test]
fn skip_whitespace_on_blank_only_file_counts_zero() {
    let dir = TempDir::new().unwrap();
    let path = write(dir.path(), "padded.txt", "\n\n\n\n\n");
    let opts = CheckOptions {
        max_lines: Some(0),
        fallback_warn: None,
        fallback_error: None,
        skip_whitespace: true,
    };
    let r = check_file(&path, None, &opts).unwrap();
    assert_eq!(r.lines, 0);
    assert_eq!(r.status, Status::Ok); // 0 is not > 0
}

#[test]
fn preset_strict() {
    let (warn, error) = Preset::Strict.limits();
    assert_eq!(warn, Some(100));
    assert_eq!(error, Some(100));
}
