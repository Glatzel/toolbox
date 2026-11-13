use mischief::{IDiagnosis, Severity};

#[derive(Debug, IDiagnosis)]
#[diagnosis(
    display,
    error,
    description = "inner failure",
    severity = Severity::Warning
)]
enum InnerDiagnostic {
    Failure,
}

#[derive(Debug, IDiagnosis)]
#[diagnosis(
    display,
    error,
    description = "application failure",
    code = "E_APPLICATION"
)]
enum ApplicationError {
    #[diagnosis(transparent)]
    Diagnostic(#[diagnosis(from)] InnerDiagnostic),
    #[diagnosis(transparent(foreign))]
    Io(#[diagnosis(from)] std::io::Error),
    #[diagnosis(description = "invalid input")]
    Invalid,
}

fn main() {
    let io = ApplicationError::from(std::io::Error::other("read failed"));
    assert_eq!(io.to_string(), "read failed");
    assert!(core::error::Error::source(&io).is_none());

    let report = io.into_report();
    assert_eq!(report.error().description(), "read failed");
    assert!(report.downcast_ref::<std::io::Error>().is_some());
    eprintln!("{report}");

    let inner = ApplicationError::from(InnerDiagnostic::Failure);
    assert_eq!(inner.to_string(), "inner failure");
    let report = inner.into_report();
    assert_eq!(report.error().description(), "inner failure");
    assert_eq!(report.error().severity(), Some(Severity::Warning));
    eprintln!("{report}");

    let invalid = ApplicationError::Invalid;
    assert_eq!(invalid.to_string(), "invalid input");
    assert_eq!(invalid.into_report().error().description(), "invalid input");
}

#[test]
fn test() { main(); }
