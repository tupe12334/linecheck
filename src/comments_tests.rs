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
