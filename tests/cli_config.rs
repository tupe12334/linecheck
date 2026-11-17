use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_linecheck"))
}

#[test]
fn malformed_config_warns_on_stderr() {
    // A broken linecheck.yml must not be dropped silently: the user's rules
    // would stop being enforced while CI keeps passing.
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(tmp.path().join("linecheck.yml"), "rules: [unclosed\n").unwrap();
    std::fs::write(tmp.path().join("a.txt"), "x\n").unwrap();
    let out = bin().current_dir(tmp.path()).arg(".").output().unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("failed to parse config"),
        "expected parse warning in stderr: {stderr}"
    );
}
