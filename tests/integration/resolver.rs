use std::fs;

use tempfile::TempDir;

use linecheck::{check_file, CheckOptions, ConfigResolver, Status};

use super::helpers::write;

#[test]
fn config_resolver_walks_up_to_parent() {
    // Structure: root/linecheck.yml (warn=3) + root/sub/file.txt
    // The resolver should find root's config when given sub/file.txt.
    let dir = TempDir::new().unwrap();
    let config_content = "rules:\n  - pattern: \"**/*.txt\"\n    warn: 3\n    error: 10\n";
    fs::write(dir.path().join("linecheck.yml"), config_content).unwrap();
    let sub = dir.path().join("sub");
    fs::create_dir(&sub).unwrap();
    let file = write(&sub, "file.txt", "a\nb\nc\nd\n"); // 4 lines > warn=3

    let mut resolver = ConfigResolver::new(None, "linecheck.yml");
    let cfg = resolver.resolve(&file);
    let r = check_file(&file, cfg.as_ref(), &CheckOptions { max_lines: None, fallback_warn: None, fallback_error: None }).unwrap();
    assert_eq!(r.status, Status::Warn);
}

#[test]
fn nested_config_overrides_parent() {
    // root/linecheck.yml (warn=100) + root/sub/linecheck.yml (warn=2)
    // The sub config should take precedence for files inside sub/.
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("linecheck.yml"),
        "rules:\n  - pattern: \"**/*.txt\"\n    warn: 100\n    error: 200\n").unwrap();
    let sub = dir.path().join("sub");
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("linecheck.yml"),
        "rules:\n  - pattern: \"**/*.txt\"\n    warn: 2\n    error: 5\n").unwrap();
    let file = write(&sub, "file.txt", "a\nb\nc\n"); // 3 lines > sub warn=2

    let mut resolver = ConfigResolver::new(None, "linecheck.yml");
    let cfg = resolver.resolve(&file);
    let r = check_file(&file, cfg.as_ref(), &CheckOptions { max_lines: None, fallback_warn: None, fallback_error: None }).unwrap();
    assert_eq!(r.status, Status::Warn);
    assert_eq!(r.warn_limit, Some(2)); // sub config, not parent's 100
}
