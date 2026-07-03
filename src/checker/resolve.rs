use super::CheckOptions;
use crate::config::Config;
use glob::Pattern;
use std::path::Path;

pub(super) fn resolve_limits(
    path: &Path,
    config: Option<&Config>,
    opts: &CheckOptions,
) -> (Option<usize>, Option<usize>, Option<String>, Option<String>) {
    if let Some(max) = opts.max_lines {
        return (Some(max), Some(max), None, None);
    }
    if let Some(cfg) = config {
        let s = path.to_string_lossy();
        let path_str = s.strip_prefix("./").unwrap_or(&s);
        for rule in &cfg.rules {
            let Ok(pat) = Pattern::new(&rule.pattern) else {
                continue;
            };
            let fname = path
                .file_name()
                .and_then(|f| f.to_str())
                .is_some_and(|f| pat.matches(f));
            if pat.matches(path_str) || fname {
                return (
                    rule.warn,
                    rule.error,
                    rule.warn_message.clone(),
                    rule.error_message.clone(),
                );
            }
        }
    }
    (opts.fallback_warn, opts.fallback_error, None, None)
}
