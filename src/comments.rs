//! Per-language line-comment prefix lookup, used by `--skip-comments`.
use std::path::Path;

/// Returns the single-line comment prefix for `path`'s extension, if recognized.
///
/// Only languages with a `//`- or `#`-style line comment are covered; block
/// comments (`/* */`) are out of scope since a full-line-comment check can't
/// tell where a block comment ends without parsing the file.
#[must_use]
pub fn line_comment_prefix(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?;
    match ext {
        "rs" | "js" | "jsx" | "mjs" | "cjs" | "ts" | "tsx" | "go" | "java" | "c" | "h" | "cpp"
        | "cc" | "cxx" | "hpp" | "hh" | "cs" | "kt" | "kts" | "swift" | "scala" | "dart" => {
            Some("//")
        }
        "py" | "rb" | "sh" | "bash" | "zsh" | "yml" | "yaml" | "toml" | "pl" | "r" | "ex"
        | "exs" => Some("#"),
        _ => None,
    }
}

#[cfg(test)]
#[path = "comments_tests.rs"]
mod tests;
