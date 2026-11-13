use mischief::{IDiagnosis, MischiefError};

#[derive(IDiagnosis)]
#[diagnosis(
    description = "invalid input",
    code = "E_INPUT",
    severity = mischief::Severity::Warning,
    help = "provide a value"
)]
struct InputError {
    #[diagnosis(source)]
    source: Option<MischiefError>,
}

fn main() {
    let error = InputError { source: None };
    assert_eq!(error.code(), Some("E_INPUT"));
    assert_eq!(error.severity(), Some(mischief::Severity::Warning));
    assert!(error.source().is_none());
}

#[test]
fn test() { main(); }
