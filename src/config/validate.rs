use super::Config;
use glob::Pattern;
use std::path::Path;

pub(crate) fn warn_invalid_patterns(cfg: &Config, source: &Path) {
    let pairs = cfg
        .rules
        .iter()
        .map(|r| (r.pattern.as_str(), "glob rule"))
        .chain(cfg.exclude.iter().map(|p| (p.as_str(), "exclude")));
    for (pat, kind) in pairs {
        if Pattern::new(pat).is_err() {
            eprintln!(
                "Warning: invalid {kind} pattern {pat:?} in {} — will be skipped",
                source.display()
            );
        }
    }
}
