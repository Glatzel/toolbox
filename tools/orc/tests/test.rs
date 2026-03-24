use std::path::PathBuf;

use assert_cmd::Command;
use path_slash::PathExt;
use rstest::rstest;

fn test_file() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("temp")
        .join("raw_r.dll")
        .to_slash_lossy()
        .to_string()
}
fn path_env() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".pixi")
        .join("envs")
        .join("default")
        .join("Library")
        .join("bin")
        .to_slash_lossy()
        .to_string()
}
#[test]
fn test_simple() {
    let cmd = Command::new(assert_cmd::cargo_bin!("orc"))
        .env_clear()
        .env("PATH", path_env())
        .args([test_file()])
        .assert()
        .success();
    insta::assert_snapshot!(String::from_utf8_lossy(cmd.get_output().stdout.as_slice()));
}
#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
fn test_limit(#[case] limit: usize) {
    let cmd = Command::new(assert_cmd::cargo_bin!("orc"))
        .env_clear()
        .env("PATH", path_env())
        .args(["-l", &limit.to_string()])
        .args([test_file()])
        .assert()
        .success();
    insta::assert_snapshot!(
        format!("test_limit-{}", limit),
        String::from_utf8_lossy(cmd.get_output().stdout.as_slice())
    );
}
#[test]
fn test_missing() {
    let cmd = Command::new(assert_cmd::cargo_bin!("orc"))
        .env_clear()
        .env("PATH", path_env())
        .args(["-l", "0"])
        .args(["-s", "missing"])
        .args([test_file()])
        .assert()
        .success();
    insta::assert_snapshot!(String::from_utf8_lossy(cmd.get_output().stdout.as_slice()));
}
