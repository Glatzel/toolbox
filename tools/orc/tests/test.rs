use std::path::PathBuf;

use assert_cmd::Command;
use path_slash::PathExt;
use rstest::rstest;

fn env_path() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".pixi")
        .join("envs")
        .join("default")
        .to_slash_lossy()
        .to_string()
}

fn test_file() -> String {
    #[cfg(target_os = "windows")]
    {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".pixi")
            .join("envs")
            .join("default")
            .join("Library")
            .join("bin")
            .join("raw_r.dll")
            .to_slash_lossy()
            .to_string()
    }
    #[cfg(target_os = "linux")]
    {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".pixi")
            .join("envs")
            .join("default")
            .join("lib")
            .join("libraw.so")
            .to_slash_lossy()
            .to_string()
    }
}
fn os() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "win"
    }

    #[cfg(target_os = "linux")]
    {
        "linux"
    }
}
macro_rules! filter {
    () => {
        vec![
            (
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .to_slash_lossy()
                    .to_string()
                    .as_str(),
                "[CARGO_MANIFEST_DIR]",
            ),
            (r".\[31m", "[RED]"),
            (r".\[32m", "[GREEN]"),
            (r".\[34m", "[BLUE]"),
            (r".\[39m", ""),
        ]
    };
}

#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
fn test_limit(#[case] limit: usize) {
    let cmd = Command::new(assert_cmd::cargo_bin!("orc"))
        .env_clear()
        .env("PATH", env_path())
        .args(["-l", &limit.to_string()])
        .args([test_file()])
        .assert()
        .success();
    println!(
        "{}",
        String::from_utf8_lossy(cmd.get_output().stdout.as_slice())
    );
    insta::with_settings!({filters => filter!()}, {
        insta::assert_snapshot!(
            format!("test_limit-{}-{}", limit, os()),
            String::from_utf8_lossy(cmd.get_output().stdout.as_slice())
        );
    });
}

#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
fn test_limit_missing(#[case] limit: usize) {
    let cmd = Command::new(assert_cmd::cargo_bin!("orc"))
        .env_clear()
        .env("PATH", env_path())
        .args(["-l", &limit.to_string()])
        .args(["-s", "missing"])
        .args([test_file()])
        .assert()
        .success();
    println!(
        "{}",
        String::from_utf8_lossy(cmd.get_output().stdout.as_slice())
    );
    insta::with_settings!({filters => filter!()
    }, {
        insta::assert_snapshot!(
            format!("test_limit_missing-{}-{}", limit, os()),
            String::from_utf8_lossy(cmd.get_output().stdout.as_slice())
        );
    });
}
