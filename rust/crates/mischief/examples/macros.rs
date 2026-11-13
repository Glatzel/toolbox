use mischief::{IDiagnosis, Result, ensure, mischief};

fn validate(value: bool) -> Result<()> {
    ensure!(value, "invalid value", code = "E_INPUT");
    Ok(())
}

fn main() {
    let report = mischief!("connect to {}", "db", code = "E_CONNECT", help = "retry");
    assert_eq!(report.error().code(), Some("E_CONNECT"));
    assert!(validate(false).is_err());
    eprintln!("{report}");
    eprintln!(
        "{}",
        validate(false).expect_err("validation should produce a report")
    );
}

#[test]
fn test() { main(); }
