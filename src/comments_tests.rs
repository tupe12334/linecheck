use super::*;

#[test]
fn rust_uses_slash_slash() {
    assert_eq!(line_comment_prefix(Path::new("src/main.rs")), Some("//"));
}

#[test]
fn typescript_uses_slash_slash() {
    assert_eq!(line_comment_prefix(Path::new("src/app.tsx")), Some("//"));
}

#[test]
fn python_uses_hash() {
    assert_eq!(
        line_comment_prefix(Path::new("scripts/build.py")),
        Some("#")
    );
}

#[test]
fn yaml_uses_hash() {
    assert_eq!(line_comment_prefix(Path::new("linecheck.yml")), Some("#"));
}

#[test]
fn unknown_extension_returns_none() {
    assert_eq!(line_comment_prefix(Path::new("data.bin")), None);
}

#[test]
fn no_extension_returns_none() {
    assert_eq!(line_comment_prefix(Path::new("Makefile")), None);
}

#[test]
fn php_uses_slash_slash() {
    assert_eq!(line_comment_prefix(Path::new("src/index.php")), Some("//"));
}

#[test]
fn terraform_uses_hash() {
    assert_eq!(line_comment_prefix(Path::new("infra/main.tf")), Some("#"));
}

#[test]
fn sql_uses_double_dash() {
    assert_eq!(
        line_comment_prefix(Path::new("migrations/001.sql")),
        Some("--")
    );
}

#[test]
fn latex_uses_percent() {
    assert_eq!(line_comment_prefix(Path::new("paper.tex")), Some("%"));
}

#[test]
fn clojure_uses_semicolon() {
    assert_eq!(line_comment_prefix(Path::new("src/core.clj")), Some(";"));
}

#[test]
fn vimscript_uses_double_quote() {
    assert_eq!(
        line_comment_prefix(Path::new("plugin/init.vim")),
        Some("\"")
    );
}

#[test]
fn ambiguous_matlab_objc_extension_stays_unrecognized() {
    assert_eq!(line_comment_prefix(Path::new("solver.m")), None);
}
