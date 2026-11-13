use mischief::{IDiagnosis, MischiefError, Report, WrapErr};

fn main() {
    let diagnosis = MischiefError::builder("parse failed")
        .code("E_PARSE")
        .severity(mischief::Severity::Warning)
        .help("check the input")
        .url("https://example.com/errors/E_PARSE")
        .build();
    let result = Err::<(), _>(Report::new(diagnosis)).wrap_err("load config");
    let report = result.expect_err("the result should be an error");

    assert_eq!(report.error().description(), "load config");
    assert_eq!(
        report.error().source().and_then(|source| source.code()),
        Some("E_PARSE")
    );
    eprintln!("{report}");
}

#[test]
fn test() { main(); }
