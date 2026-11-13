use mischief::{IDiagnosis, MischiefError, Severity};

#[derive(IDiagnosis)]
enum ApplicationError {
    #[diagnosis(forward)]
    Diagnostic(MischiefError),
}

fn main() {
    let diagnosis = MischiefError::builder("database unavailable")
        .code("E_DATABASE")
        .severity(Severity::Error)
        .help("start the database")
        .build();
    let error = ApplicationError::Diagnostic(diagnosis);

    assert_eq!(error.description(), "database unavailable");
    assert_eq!(error.code(), Some("E_DATABASE"));
    assert_eq!(error.severity(), Some(Severity::Error));
    assert_eq!(error.help(), Some("start the database"));
    assert!(error.source().is_none());
}

#[test]
fn test() { main(); }
