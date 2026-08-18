use crate::glob_path::glob_path;
use glob::Pattern;
use std::path::Path;

pub(super) fn excluded(path: &Path, root: Option<&Path>, pats: &[Pattern]) -> bool {
    if let Some(root) = root
        && let Ok(rel) = path.strip_prefix(root)
    {
        let s = glob_path(rel);
        if pats.iter().any(|p| p.matches(&s)) {
            return true;
        }
    }
    let s = glob_path(path);
    let n = s.strip_prefix("./").unwrap_or(&s);
    pats.iter().any(|p| p.matches(n) || p.matches(&s))
}
