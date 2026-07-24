use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_linecheck"))
}

fn padded_file(tmp: &tempfile::TempDir) {
    // 2 real lines padded with 8 blank/whitespace-only lines
    let content = "a\n".to_string() + &"   \n".repeat(8) + "b\n";
    std::fs::write(tmp.path().join("padded.txt"), &content).unwrap();
}

#[test]
fn skip_whitespace_flag_excludes_blank_lines_from_reported_count() {
    let tmp = tempfile::TempDir::new().unwrap();
    padded_file(&tmp);
    let out = bin()
        .args([
            "--json",
            "--status",
            "--max-lines",
            "2",
            "--skip-whitespace",
        ])
        .arg(tmp.path())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"lines\":2"),
        "expected blank lines excluded from count: {stdout}"
    );
}

#[test]
fn skip_whitespace_flag_off_by_default_counts_blank_lines() {
    let tmp = tempfile::TempDir::new().unwrap();
    padded_file(&tmp);
    let out = bin()
        .args(["--json", "--status"])
        .arg(tmp.path())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"lines\":10"),
        "expected blank lines counted without the flag: {stdout}"
    );
}
