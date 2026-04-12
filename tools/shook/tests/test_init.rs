use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_init() {
    let pwd = tempdir().unwrap();
    Command::new(assert_cmd::cargo_bin!("shook"))
        .arg("init")
        .current_dir(&pwd)
        .assert()
        .success();
    assert!(pwd.path().join("shook.toml").exists());
    assert!(pwd.path().join("shook.schema.json").exists());
}
