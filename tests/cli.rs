use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_linecheck"))
}

#[test]
fn version_flag() {
    let out = bin().arg("--version").output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("linecheck"),
        "expected 'linecheck' in: {stdout}"
    );
    assert!(stdout.contains("0.3"), "expected version in: {stdout}");
}

#[test]
fn missing_explicit_config_exits_nonzero() {
    let out = bin()
        .args(["--config", "/tmp/linecheck-test-nonexistent.yml", "."])
        .output()
        .unwrap();
    assert!(!out.status.success(), "expected exit 1 for missing config");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("not found"),
        "expected 'not found' in stderr: {stderr}"
    );
}

#[test]
fn default_config_missing_does_not_error() {
    // When the default 'linecheck.yml' is absent, hierarchical lookup runs silently.
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(tmp.path().join("a.rs"), "fn main() {}\n").unwrap();
    let out = bin().arg(tmp.path()).output().unwrap();
    // No linecheck.yml in tmp → falls back to built-in defaults → file is fine
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn dotslash_default_config_is_not_explicit() {
    // "--config ./linecheck.yml" should behave like the default (no error when absent),
    // not like an explicit path (which errors when absent).
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(tmp.path().join("a.rs"), "fn main() {}\n").unwrap();
    let out = bin()
        .args(["--config", "./linecheck.yml"])
        .arg(tmp.path())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn nonexistent_path_warns_on_stderr() {
    let out = bin()
        .arg("/tmp/linecheck-nonexistent-dir-xyz")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "should exit 0 (no files checked = no errors)"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("not found"),
        "expected 'not found' warning in: {stderr}"
    );
}

#[test]
fn exits_one_on_error_threshold_breach() {
    let tmp = tempfile::TempDir::new().unwrap();
    let content = (0..10).map(|i| format!("line{i}\n")).collect::<String>();
    std::fs::write(tmp.path().join("big.rs"), &content).unwrap();
    let out = bin()
        .args(["--max-lines", "3"])
        .arg(tmp.path())
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
}
