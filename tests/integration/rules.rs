use tempfile::TempDir;

use linecheck::{check_file, Config, Rule, Status};

use super::helpers::{cfg, opts_unlimited, write};

#[test]
fn first_matching_rule_wins() {
    // Rules are evaluated in order; the first match is used, not the most specific.
    // Put more specific patterns before broader ones.
    let dir = TempDir::new().unwrap();
    let path = write(dir.path(), "file.txt", &(0..5).map(|i| format!("line{i}\n")).collect::<String>());

    // Broad rule first (warn=10) — should win over the specific rule below (warn=2)
    let broad_first = Config {
        rules: vec![
            Rule { pattern: "**/*.txt".into(), warn: Some(10), warn_message: None, error: Some(20), error_message: None },
            Rule { pattern: "file.txt".into(), warn: Some(2),  warn_message: None, error: Some(5),  error_message: None },
        ],
        exclude: vec![],
    };
    let r = check_file(&path, Some(&broad_first), &opts_unlimited()).unwrap();
    assert_eq!(r.status, Status::Ok);      // 5 lines < warn=10 from first rule
    assert_eq!(r.warn_limit, Some(10));

    // Specific rule first (warn=2) — should win now
    let specific_first = Config {
        rules: vec![
            Rule { pattern: "file.txt".into(), warn: Some(2),  warn_message: None, error: Some(5),  error_message: None },
            Rule { pattern: "**/*.txt".into(), warn: Some(10), warn_message: None, error: Some(20), error_message: None },
        ],
        exclude: vec![],
    };
    let r2 = check_file(&path, Some(&specific_first), &opts_unlimited()).unwrap();
    assert_eq!(r2.status, Status::Warn);   // 5 lines > warn=2 from first rule
    assert_eq!(r2.warn_limit, Some(2));
}

#[test]
fn rule_message_propagates_to_result() {
    let dir = TempDir::new().unwrap();
    let warn_cfg = Config {
        rules: vec![Rule {
            pattern: "**/*.txt".into(),
            warn: Some(2), warn_message: Some("getting long".into()),
            error: Some(10), error_message: Some("too large".into()),
        }],
        exclude: vec![],
    };

    // warn_message used when status is Warn
    let path = write(dir.path(), "a.txt", "1\n2\n3\n"); // 3 lines > warn=2
    let r = check_file(&path, Some(&warn_cfg), &opts_unlimited()).unwrap();
    assert_eq!(r.status, Status::Warn);
    assert_eq!(r.message.as_deref(), Some("getting long"));

    // error_message used when status is Error
    let path2 = write(dir.path(), "b.txt", &"x\n".repeat(11)); // 11 lines > error=10
    let r2 = check_file(&path2, Some(&warn_cfg), &opts_unlimited()).unwrap();
    assert_eq!(r2.status, Status::Error);
    assert_eq!(r2.message.as_deref(), Some("too large"));

    // no message when status is Ok
    let path3 = write(dir.path(), "c.txt", "x\n"); // 1 line <= warn=2
    let r3 = check_file(&path3, Some(&cfg(2, 5)), &opts_unlimited()).unwrap();
    assert_eq!(r3.status, Status::Ok);
    assert_eq!(r3.message, None);
}
