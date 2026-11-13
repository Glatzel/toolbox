use std::fs;
use std::process::Command;

#[test]
fn bail_and_ensure_work_with_a_renamed_facade_dependency() {
    let temp = tempfile::tempdir().expect("temporary directory should be created");
    let crate_dir = temp.path().join("renamed-consumer");
    fs::create_dir_all(crate_dir.join("src")).expect("fixture source directory should exist");

    let facade_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .display()
        .to_string()
        .replace('\\', "/");
    let manifest = format!(
        r#"
[package]
name = "renamed-consumer"
version = "0.0.0"
edition = "2024"

[dependencies]
diagnostics = {{ package = "mischief", path = "{facade_path}", default-features = false, features = ["std"] }}
"#
    );
    let source = r#"
use core::fmt;
use diagnostics::{bail, ensure};

#[derive(Debug)]
struct SourceError;

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("source error")
    }
}

impl core::error::Error for SourceError {}

fn fail() -> diagnostics::Result<()> {
    bail!("renamed bail", source = SourceError, code = "E_BAIL");
}

fn check(value: bool) -> diagnostics::Result<()> {
    ensure!(value, "renamed ensure", help = "retry");
    Ok(())
}

fn main() {
    let _ = fail();
    let _ = check(true);
}
"#;

    fs::write(crate_dir.join("Cargo.toml"), manifest).expect("fixture manifest should be written");
    fs::write(crate_dir.join("src/main.rs"), source).expect("fixture source should be written");

    let output = Command::new(env!("CARGO"))
        .args([
            "check",
            "--quiet",
            "--manifest-path",
            crate_dir
                .join("Cargo.toml")
                .to_str()
                .expect("fixture manifest path should be UTF-8"),
        ])
        .env("CARGO_TARGET_DIR", temp.path().join("target"))
        .output()
        .expect("fixture cargo check should run");

    assert!(
        output.status.success(),
        "renamed dependency fixture failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
