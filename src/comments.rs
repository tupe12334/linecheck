//! Per-language line-comment prefix lookup, used by `--skip-comments`.
use std::path::Path;

/// Returns the single-line comment prefix for `path`'s extension, if recognized.
///
/// Block comments (`/* */`) are out of scope since a full-line-comment check
/// can't tell where a block comment ends without parsing the file. Extensions
/// whose comment syntax depends on the language they belong to (`.m` is
/// Objective-C or MATLAB) are left unrecognized rather than guessed.
#[must_use]
pub fn line_comment_prefix(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?;
    match ext {
        "rs" | "js" | "jsx" | "mjs" | "cjs" | "ts" | "tsx" | "go" | "java" | "c" | "h" | "cpp"
        | "cc" | "cxx" | "hpp" | "hh" | "cs" | "kt" | "kts" | "swift" | "scala" | "dart"
        | "php" | "zig" | "groovy" | "gradle" | "sol" | "proto" => Some("//"),
        "py" | "rb" | "sh" | "bash" | "zsh" | "yml" | "yaml" | "toml" | "pl" | "r" | "ex"
        | "exs" | "nix" | "tf" | "tfvars" | "hcl" | "jl" | "cmake" | "graphql" | "gql" | "ps1"
        | "fish" | "mk" | "dockerfile" => Some("#"),
        "lua" | "sql" | "hs" | "elm" | "adb" | "ads" => Some("--"),
        "erl" | "hrl" | "tex" | "sty" => Some("%"),
        "lisp" | "el" | "scm" | "ss" | "clj" | "cljs" | "cljc" | "edn" | "ini" => Some(";"),
        "vim" => Some("\""),
        _ => None,
    }
}

#[cfg(test)]
#[path = "comments_tests.rs"]
mod tests;
