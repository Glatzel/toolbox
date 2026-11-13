#![cfg(feature = "macros")]

use mischief::{IDiagnosis, MischiefError, Severity};

#[derive(IDiagnosis)]
#[diagnosis(
    description = "static description",
    code = "E_STATIC",
    severity = Severity::Warning,
    help = "static help",
    url = "https://example.com/errors/E_STATIC",
)]
struct StaticDiagnosis {
    #[diagnosis(source)]
    source: Option<MischiefError>,
}

#[derive(IDiagnosis)]
#[diagnosis(description = message)]
struct DelegatedDiagnosis {
    message: String,
    #[diagnosis(source)]
    source: MischiefError,
}

#[derive(IDiagnosis)]
#[diagnosis(description = "description only")]
struct DescriptionOnly {
    value: u8,
}

#[test]
fn derives_static_description_and_metadata() {
    let source = MischiefError::builder("source").build();
    let diagnosis = StaticDiagnosis {
        source: Some(source),
    };

    assert_eq!(diagnosis.description(), "static description");
    assert_eq!(diagnosis.code(), Some("E_STATIC"));
    assert_eq!(diagnosis.severity(), Some(Severity::Warning));
    assert_eq!(diagnosis.help(), Some("static help"));
    assert_eq!(diagnosis.url(), Some("https://example.com/errors/E_STATIC"));
    assert_eq!(
        diagnosis.source().map(IDiagnosis::description),
        Some("source")
    );
}

#[test]
fn derives_delegated_description_and_source() {
    let diagnosis = DelegatedDiagnosis {
        message: String::from("delegated description"),
        source: MischiefError::builder("source").build(),
    };

    assert_eq!(diagnosis.description(), "delegated description");
    assert_eq!(diagnosis.code(), None);
    assert_eq!(diagnosis.severity(), None);
    assert_eq!(diagnosis.help(), None);
    assert_eq!(diagnosis.url(), None);
    assert_eq!(
        diagnosis.source().map(IDiagnosis::description),
        Some("source")
    );
}

#[test]
fn derives_no_source_when_no_field_is_marked() {
    let diagnosis = DescriptionOnly { value: 1 };

    assert_eq!(diagnosis.value, 1);
    assert_eq!(diagnosis.description(), "description only");
    assert!(diagnosis.source().is_none());
}
