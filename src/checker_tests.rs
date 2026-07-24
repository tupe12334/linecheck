use super::*;
use crate::result::Status;

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

    assert!(
        Pattern::new("[invalid-pattern").is_err(),
        "pattern should be invalid"
    );
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
        skip_whitespace: false,
    };
    let result = check_file(f.path(), Some(&cfg), &opts).unwrap();
    assert_eq!(result.status, Status::Ok);
}

#[test]
fn non_matching_rule_is_skipped_and_next_rule_wins() {
    use crate::config::Config;
    use crate::rule::Rule;
    use std::io::Write;
    use tempfile::Builder;

    let mut f = Builder::new().suffix(".rs").tempfile().unwrap();
    writeln!(f, "line1").unwrap();
    let cfg = Config {
        rules: vec![
            Rule {
                pattern: "**/*.ts".into(),
                warn: None,
                error: Some(1),
                warn_message: None,
                error_message: None,
            },
            Rule {
                pattern: "**/*.rs".into(),
                warn: None,
                error: Some(99),
                warn_message: None,
                error_message: None,
            },
        ],
        exclude: vec![],
    };
    let opts = CheckOptions {
        max_lines: None,
        fallback_warn: None,
        fallback_error: None,
        skip_whitespace: false,
    };
    let result = check_file(f.path(), Some(&cfg), &opts).unwrap();
    assert_eq!(result.error_limit, Some(99));
}

#[test]
fn check_content_matches_rule_by_virtual_path() {
    use crate::config::Config;
    use crate::rule::Rule;
    use std::path::Path;

    let cfg = Config {
        rules: vec![Rule {
            pattern: "**/*.rs".into(),
            warn: None,
            error: Some(1),
            warn_message: None,
            error_message: Some("too long".into()),
        }],
        exclude: vec![],
    };
    let opts = CheckOptions {
        max_lines: None,
        fallback_warn: None,
        fallback_error: None,
        skip_whitespace: false,
    };
    let result = check_content(
        Path::new("src/main.rs"),
        b"line1\nline2\n",
        Some(&cfg),
        &opts,
    );
    assert_eq!(result.status, Status::Error);
    assert_eq!(result.lines, 2);
    assert_eq!(result.message.as_deref(), Some("too long"));
}

#[test]
fn check_content_respects_ignore_marker() {
    use std::path::Path;

    let opts = CheckOptions {
        max_lines: Some(0),
        fallback_warn: None,
        fallback_error: None,
        skip_whitespace: false,
    };
    // Escape ':' as \x3a so this test file doesn't self-ignore.
    let result = check_content(
        Path::new("src/generated.rs"),
        b"// linecheck\x3aignore\nline1\n",
        None,
        &opts,
    );
    assert_eq!(result.status, Status::Ok);
}
