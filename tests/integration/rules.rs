use tempfile::TempDir;

use linecheck::{check_file, Config, Rule, Status};

use super::helpers::{opts_unlimited, write};

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
