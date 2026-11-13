use mischief::{IDiagnosis, MischiefError, Severity};

mod support;

#[derive(IDiagnosis)]
#[diagnosis(
    description = "configuration failed",
    code = "E_CONFIG",
    severity = Severity::Warning
)]
pub enum ConfigError {
    Missing,
    #[diagnosis(
        description = "invalid port",
        code = "E_CONFIG_PORT",
        severity = Severity::Error,
        help = "provide a port from 0 through 65535"
    )]
    InvalidPort {
        #[diagnosis(source)]
        source: MischiefError,
    },
}

#[allow(clippy::result_large_err)]
pub fn parse_port(input: &str) -> Result<u16, ConfigError> {
    if input.is_empty() {
        return Err(ConfigError::Missing);
    }
    input.parse().map_err(|source| ConfigError::InvalidPort {
        source: MischiefError::from_error(source),
    })
}

fn main() {
    let missing = parse_port("").expect_err("empty input should fail");
    assert_eq!(missing.description(), "configuration failed");
    assert_eq!(missing.code(), Some("E_CONFIG"));

    let invalid = parse_port("70000").expect_err("out-of-range input should fail");
    assert_eq!(invalid.description(), "invalid port");
    assert_eq!(invalid.code(), Some("E_CONFIG_PORT"));
    assert_eq!(invalid.severity(), Some(Severity::Error));
    assert!(invalid.source().is_some());
    support::print_diagnosis("library error", &invalid);
}

#[test]
fn test() { main(); }
