use glob::Pattern;
use std::path::Path;

pub(super) fn excluded(path: &Path, root: Option<&Path>, pats: &[Pattern]) -> bool {
    if let Some(root) = root
        && let Ok(rel) = path.strip_prefix(root)
    {
        let s = rel.to_string_lossy();
        if pats.iter().any(|p| p.matches(&s)) {
            return true;
        }
    }
    let s = path.to_string_lossy();
    let n = s.strip_prefix("./").unwrap_or(&s);
    pats.iter().any(|p| p.matches(n) || p.matches(&s))
}
