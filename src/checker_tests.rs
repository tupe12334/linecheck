use super::*;

#[test]
fn status_ordering() {
    assert!(Status::Error > Status::Warn);
    assert!(Status::Warn > Status::Ok);
}

#[test]
fn check_file_with_invalid_glob_rule_falls_back_to_defaults() {
    use crate::config::Config;
    use crate::rule::Rule;
    use glob::Pattern;
    use std::io::Write;
    use tempfile::NamedTempFile;

    assert!(Pattern::new("[invalid-pattern").is_err(), "pattern should be invalid");
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "line1\nline2\nline3").unwrap();
    let cfg = Config {
        rules: vec![Rule {
            pattern: "[invalid-pattern".into(),
            warn: Some(1),
            error: Some(1),
            warn_message: None,
            error_message: None,
        }],
        exclude: vec![],
    };
    let opts = CheckOptions {
        max_lines: None,
        fallback_warn: None,
        fallback_error: None,
    };
    let result = check_file(f.path(), Some(&cfg), &opts).unwrap();
    assert_eq!(result.status, Status::Ok);
}
