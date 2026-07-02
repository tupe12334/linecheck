use anyhow::Result;
use glob::Pattern;
use std::path::Path;

use crate::config::Config;
use crate::lines::count_lines;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Status {
    Ok,
    Warn,
    Error,
}

pub fn check_file(path: &Path, config: &Config, max_lines_override: Option<usize>) -> Result<(Status, usize)> {
    let lines = count_lines(path)?;
    let (warn_limit, error_limit) = resolve_limits(path, config, max_lines_override);
    let status = if error_limit.map_or(false, |l| lines > l) { Status::Error }
        else if warn_limit.map_or(false, |l| lines > l) { Status::Warn }
        else { Status::Ok };
    Ok((status, lines))
}

fn resolve_limits(path: &Path, config: &Config, override_: Option<usize>) -> (Option<usize>, Option<usize>) {
    if let Some(max) = override_ { return (Some(max), Some(max)); }
    let path_str = path.to_string_lossy();
    for rule in &config.rules {
        let Ok(pat) = Pattern::new(&rule.pattern) else { continue };
        let fname_match = path.file_name().and_then(|f| f.to_str()).map_or(false, |f| pat.matches(f));
        if pat.matches(&path_str) || fname_match { return (rule.warn, rule.error); }
    }
    (None, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_ordering() {
        assert!(Status::Error > Status::Warn);
        assert!(Status::Warn > Status::Ok);
    }
}
