use mischief::{IDiagnosis, IntoMischief, Report, WrapErr};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("invalid setting `{key}`")]
struct SettingsError {
    key: &'static str,
}

#[derive(Debug, Error)]
#[error("configuration failed")]
struct UpstreamError {
    #[source]
    source: SettingsError,
}

#[derive(Debug, Error)]
#[error(transparent)]
struct TransparentError {
    #[from]
    source: SettingsError,
}

fn load() -> Result<(), UpstreamError> {
    Err(UpstreamError {
        source: SettingsError { key: "port" },
    })
}

fn main() {
    let result = load().into_mischief().wrap_err("load settings");
    let report = result.expect_err("loading settings should fail");

    assert_eq!(report.error().description(), "load settings");
    assert!(report.downcast_ref::<UpstreamError>().is_some());
    let upstream = report.error().source().expect("upstream source");
    assert_eq!(upstream.description(), "configuration failed");
    assert_eq!(
        upstream.source().map(IDiagnosis::description),
        Some("invalid setting `port`")
    );

    let transparent = Report::from(TransparentError::from(SettingsError { key: "mode" }));
    assert_eq!(transparent.error().description(), "invalid setting `mode`");
    assert!(transparent.downcast_ref::<TransparentError>().is_some());
    assert!(transparent.downcast_ref::<SettingsError>().is_none());

    let directly_converted = Report::from(SettingsError { key: "mode" });
    assert!(directly_converted.downcast_ref::<SettingsError>().is_some());
}

#[test]
fn test() { main(); }
