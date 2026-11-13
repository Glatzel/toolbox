use mischief::{IDiagnosis, IntoMischief, Result, WrapErr, bail, ensure};

fn parse_port(input: &str) -> Result<u16> {
    ensure!(
        !input.is_empty(),
        "port is missing",
        code = "E_PORT_MISSING",
        help = "set a port in the configuration"
    );
    input
        .parse()
        .into_mischief()
        .wrap_err_with(|| format!("invalid port {input:?}"))
}

fn load_mode(mode: &str) -> Result<&'static str> {
    if mode == "development" {
        Ok("development")
    } else if mode == "production" {
        Ok("production")
    } else {
        bail!(
            "unknown mode {:?}",
            mode,
            code = "E_MODE",
            help = "use `development` or `production`"
        );
    }
}

fn main() -> Result<()> {
    let missing = parse_port("").expect_err("empty input should fail");
    assert_eq!(missing.error().code(), Some("E_PORT_MISSING"));

    let invalid = parse_port("not-a-port").expect_err("invalid input should fail");
    assert_eq!(invalid.error().description(), "invalid port \"not-a-port\"");
    assert!(invalid.downcast_ref::<std::num::ParseIntError>().is_some());
    eprintln!("{invalid}");

    let mode = load_mode("staging").expect_err("unknown mode should fail");
    assert_eq!(mode.error().code(), Some("E_MODE"));
    eprintln!("{mode}");
    Ok(())
}

#[test]
fn test() { main().expect("example assertions should pass"); }
