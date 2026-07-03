use std::fs;
use std::path::Path;

use linecheck::{CheckOptions, Config, Rule};

pub fn write(dir: &Path, name: &str, content: &str) -> std::path::PathBuf {
    let p = dir.join(name);
    fs::write(&p, content).unwrap();
    p
}

pub fn cfg(warn: usize, error: usize) -> Config {
    Config {
        rules: vec![Rule { pattern: "**/*.txt".into(), warn: Some(warn), warn_message: None, error: Some(error), error_message: None }],
        exclude: vec![],
    }
}

pub fn opts_unlimited() -> CheckOptions {
    CheckOptions { max_lines: None, fallback_warn: None, fallback_error: None }
}
