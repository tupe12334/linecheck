use std::fs;
use std::path::Path;
use tempfile::TempDir;

use linecheck::{check_file, collect_files, load_config, CheckOptions, Config, ConfigResolver, Preset, Rule, Status};

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
        + "# linecheck\x3aignore\n"; // \x3a = ':' — avoids this file self-ignoring
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

#[test]
fn first_matching_rule_wins() {
    // Rules are evaluated in order; the first match is used, not the most specific.
    // Put more specific patterns before broader ones.
    let dir = TempDir::new().unwrap();
    let path = write(dir.path(), "file.txt", &(0..5).map(|i| format!("line{i}\n")).collect::<String>());

    // Broad rule first (warn=10) — should win over the specific rule below (warn=2)
    let broad_first = Config {
        rules: vec![
            Rule { pattern: "**/*.txt".into(), warn: Some(10), error: Some(20) },
            Rule { pattern: "file.txt".into(), warn: Some(2),  error: Some(5)  },
        ],
        exclude: vec![],
    };
    let r = check_file(&path, Some(&broad_first), &opts_unlimited()).unwrap();
    assert_eq!(r.status, Status::Ok);      // 5 lines < warn=10 from first rule
    assert_eq!(r.warn_limit, Some(10));

    // Specific rule first (warn=2) — should win now
    let specific_first = Config {
        rules: vec![
            Rule { pattern: "file.txt".into(), warn: Some(2),  error: Some(5)  },
            Rule { pattern: "**/*.txt".into(), warn: Some(10), error: Some(20) },
        ],
        exclude: vec![],
    };
    let r2 = check_file(&path, Some(&specific_first), &opts_unlimited()).unwrap();
    assert_eq!(r2.status, Status::Warn);   // 5 lines > warn=2 from first rule
    assert_eq!(r2.warn_limit, Some(2));
}

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
