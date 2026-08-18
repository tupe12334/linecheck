use glob::Pattern;
use std::path::Path;

// Exclude patterns are always written with `/`, so Windows paths are
// normalized before matching or a pattern like `src/**` never matches.
#[cfg(windows)]
fn as_glob_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(not(windows))]
fn as_glob_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

pub(super) fn excluded(path: &Path, root: Option<&Path>, pats: &[Pattern]) -> bool {
    if let Some(root) = root
        && let Ok(rel) = path.strip_prefix(root)
    {
        let s = as_glob_path(rel);
        if pats.iter().any(|p| p.matches(&s)) {
            return true;
        }
    }
    let s = as_glob_path(path);
    let n = s.strip_prefix("./").unwrap_or(&s);
    pats.iter().any(|p| p.matches(n) || p.matches(&s))
}
