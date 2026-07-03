use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_linecheck"))
}

#[test]
fn preset_loose_flag_runs() {
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(tmp.path().join("a.rs"), "fn main() {}\n").unwrap();
    let out = bin()
        .current_dir(tmp.path())
        .arg("--loose")
        .arg(".")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn preset_free_never_errors() {
    let tmp = tempfile::TempDir::new().unwrap();
    let content = (0..1000).map(|i| format!("line{i}\n")).collect::<String>();
    std::fs::write(tmp.path().join("huge.rs"), &content).unwrap();
    let out = bin()
        .current_dir(tmp.path())
        .arg("--free")
        .arg(".")
        .output()
        .unwrap();
    assert!(out.status.success());
}

#[test]
fn default_config_file_present_in_cwd_is_used() {
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(tmp.path().join("linecheck.yml"), "rules: []\n").unwrap();
    std::fs::write(tmp.path().join("a.rs"), "fn main() {}\n").unwrap();
    let out = bin().current_dir(tmp.path()).arg(".").output().unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn explicit_config_that_exists_is_loaded() {
    let tmp = tempfile::TempDir::new().unwrap();
    let cfg = tmp.path().join("my.yml");
    std::fs::write(&cfg, "rules: []\n").unwrap();
    std::fs::write(tmp.path().join("a.rs"), "fn main() {}\n").unwrap();
    let out = bin()
        .args(["--config", cfg.to_str().unwrap()])
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
fn direct_file_arg_is_checked() {
    let tmp = tempfile::TempDir::new().unwrap();
    let file = tmp.path().join("a.rs");
    std::fs::write(&file, "fn main() {}\n").unwrap();
    let out = bin().current_dir(tmp.path()).arg(&file).output().unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}
