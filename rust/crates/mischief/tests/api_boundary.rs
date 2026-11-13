use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn check_fixture(name: &str, manifest: &str, source: &str) {
    let temp = tempfile::tempdir().expect("temporary directory should be created");
    let crate_dir = temp.path().join(name);
    fs::create_dir_all(crate_dir.join("src")).expect("fixture source directory should exist");
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
        "{name} fixture failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn fixture_manifest(
    package: &str,
    dependency_alias: &str,
    package_name: &str,
    path: &Path,
) -> String {
    format!(
        r#"
[workspace]

[package]
name = "{package}"
version = "0.0.0"
edition = "2024"

[dependencies]
{dependency_alias} = {{ package = "{package_name}", path = "{}", default-features = false }}
"#,
        path.display().to_string().replace('\\', "/")
    )
}

fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("mischief crate should be inside the crates directory")
        .to_path_buf()
}

#[test]
fn derive_compiles_against_mischief_api_only() {
    let source = r#"
use diagnostics_api::IDiagnosis;

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "inner")]
struct InnerError {}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error)]
enum ApiError {
    #[diagnosis(description = "Unknown pitch: {0}")]
    Unknown(char),
    #[diagnosis(transparent)]
    Inner(InnerError),
    #[diagnosis(transparent(foreign))]
    Io(#[diagnosis(from)] std::io::Error),
    #[diagnosis(description = "wrapped")]
    Wrapped {
        #[diagnosis(source)]
        source: Option<InnerError>,
    },
}

fn main() {
    let error = ApiError::Unknown('H');
    assert_eq!(error.description(), "Unknown pitch: {0}");
    assert_eq!(error.to_string(), "Unknown pitch: H");
    assert_eq!(ApiError::Io(std::io::Error::other("io")).to_string(), "io");
}
"#;

    check_fixture(
        "api-only",
        &fixture_manifest(
            "api-only",
            "diagnostics_api",
            "mischief-api",
            &crates_dir().join("mischief-api"),
        ),
        source,
    );
}

#[test]
fn derive_compiles_against_mischief_facade_only() {
    let source = r#"
use diagnostics::IDiagnosis;

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error, description = "inner")]
struct InnerError {}

#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error)]
enum FacadeError {
    #[diagnosis(description = "Unknown pitch: {0}")]
    Unknown(char),
    #[diagnosis(transparent(foreign))]
    Io(#[diagnosis(from)] std::io::Error),
    #[diagnosis(description = "wrapped")]
    Wrapped {
        #[diagnosis(source)]
        source: Option<InnerError>,
    },
}

fn main() {
    let report = FacadeError::Io(std::io::Error::other("io")).into_report();
    assert_eq!(report.error().description(), "io");
    assert!(report.downcast_ref::<std::io::Error>().is_some());

    let error = FacadeError::Unknown('H');
    assert_eq!(error.description(), "Unknown pitch: {0}");
    assert_eq!(error.to_string(), "Unknown pitch: H");
}
"#;

    let manifest = format!(
        r#"
[workspace]

[package]
name = "facade-only"
version = "0.0.0"
edition = "2024"

[dependencies]
diagnostics = {{ package = "mischief", path = "{}", default-features = false, features = ["macros"] }}
"#,
        crates_dir().join("mischief").display()
    );

    check_fixture("facade-only", &manifest, source);
}
