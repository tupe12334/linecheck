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
        let fname_matches = |pat: &Pattern| {
            path.file_name()
                .and_then(|f| f.to_str())
                .is_some_and(|f| pat.matches(f))
        };
        for rule in cfg
            .rules
            .iter()
            .filter_map(|r| Pattern::new(&r.pattern).ok().map(|p| (r, p)))
        {
            if rule.1.matches(path_str) || fname_matches(&rule.1) {
                return (
                    rule.0.warn,
                    rule.0.error,
                    rule.0.warn_message.clone(),
                    rule.0.error_message.clone(),
                );
            }
        }
    }
    (opts.fallback_warn, opts.fallback_error, None, None)
}
