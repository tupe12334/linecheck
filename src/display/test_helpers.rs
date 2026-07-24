use linecheck::checker::CheckOptions;
use linecheck::config::ConfigResolver;
use std::fs;
use tempfile::TempDir;

pub(super) fn make_file(dir: &TempDir, name: &str, line_count: usize) -> std::path::PathBuf {
    let content = (0..line_count)
        .map(|i| format!("line{i}\n"))
        .collect::<String>();
    let path = dir.path().join(name);
    fs::write(&path, &content).unwrap();
    path
}
pub(super) fn unlimited() -> CheckOptions {
    CheckOptions {
        max_lines: None,
        fallback_warn: None,
        fallback_error: None,
        skip_whitespace: false,
    }
}
pub(super) fn with_limits(warn: usize, error: usize) -> CheckOptions {
    CheckOptions {
        max_lines: None,
        fallback_warn: Some(warn),
        fallback_error: Some(error),
        skip_whitespace: false,
    }
}
pub(super) fn resolver() -> ConfigResolver {
    ConfigResolver::new(None, "linecheck.yml")
}
