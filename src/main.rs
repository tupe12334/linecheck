use anyhow::{Context, Result};
use clap::Parser;
use glob::Pattern;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "linecheck", about = "Warn or error when files exceed a set line count")]
struct Args {
    /// Files or directories to check
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,

    /// Override the maximum line limit
    #[arg(long)]
    max_lines: Option<usize>,

    /// Config file path (default: .linecheckrc)
    #[arg(long, default_value = ".linecheckrc")]
    config: PathBuf,
}

#[derive(Debug, Deserialize, Default)]
struct Config {
    #[serde(default)]
    rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
struct Rule {
    pattern: String,
    warn: Option<usize>,
    error: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Status {
    Ok,
    Warn,
    Error,
}

fn load_config(path: &Path) -> Config {
    let Ok(content) = fs::read_to_string(path) else {
        return Config::default();
    };
    serde_yaml::from_str(&content).unwrap_or_else(|e| {
        eprintln!("Warning: failed to parse config {}: {}", path.display(), e);
        Config::default()
    })
}

fn count_lines(path: &Path) -> Result<usize> {
    let content = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(bytecount_lines(&content))
}

fn bytecount_lines(data: &[u8]) -> usize {
    if data.is_empty() {
        return 0;
    }
    let newlines = data.iter().filter(|&&b| b == b'\n').count();
    if *data.last().unwrap() != b'\n' {
        newlines + 1
    } else {
        newlines
    }
}

fn match_rule<'a>(config: &'a Config, path: &Path) -> Option<&'a Rule> {
    let path_str = path.to_string_lossy();
    for rule in &config.rules {
        if let Ok(pat) = Pattern::new(&rule.pattern) {
            if pat.matches(&path_str) {
                return Some(rule);
            }
            if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
                if pat.matches(fname) {
                    return Some(rule);
                }
            }
        }
    }
    None
}

fn check_file(
    path: &Path,
    config: &Config,
    max_lines_override: Option<usize>,
) -> Result<(Status, usize)> {
    let lines = count_lines(path)?;

    let rule = match_rule(config, path);

    let (warn_limit, error_limit) = if let Some(max) = max_lines_override {
        (Some(max), Some(max))
    } else if let Some(r) = rule {
        (r.warn, r.error)
    } else {
        (None, None)
    };

    let status = if error_limit.map_or(false, |limit| lines > limit) {
        Status::Error
    } else if warn_limit.map_or(false, |limit| lines > limit) {
        Status::Warn
    } else {
        Status::Ok
    };

    Ok((status, lines))
}

fn collect_files(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for path in paths {
        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() {
            for entry in WalkDir::new(path)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                files.push(entry.into_path());
            }
        }
    }
    files
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = load_config(&args.config);

    let files = collect_files(&args.paths);

    let mut worst = Status::Ok;

    for file in &files {
        match check_file(file, &config, args.max_lines) {
            Ok((status, lines)) => {
                if status >= Status::Warn {
                    let label = if status == Status::Error { "ERROR" } else { "WARN" };
                    println!("{} {} ({} lines)", label, file.display(), lines);
                    if status > worst {
                        worst = status;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    let exit_code = match worst {
        Status::Ok => 0,
        Status::Warn => 1,
        Status::Error => 2,
    };

    std::process::exit(exit_code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytecount_lines_empty() {
        assert_eq!(bytecount_lines(b""), 0);
    }

    #[test]
    fn test_bytecount_lines_no_trailing_newline() {
        assert_eq!(bytecount_lines(b"hello\nworld"), 2);
    }

    #[test]
    fn test_bytecount_lines_trailing_newline() {
        assert_eq!(bytecount_lines(b"hello\nworld\n"), 2);
    }

    #[test]
    fn test_bytecount_lines_single() {
        assert_eq!(bytecount_lines(b"hello"), 1);
    }

    #[test]
    fn test_status_ordering() {
        assert!(Status::Error > Status::Warn);
        assert!(Status::Warn > Status::Ok);
    }
}
