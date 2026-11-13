use mischief::{IDiagnosis, MischiefError, Severity};

#[derive(IDiagnosis)]
#[diagnosis(
    description = "command failed",
    code = "E_COMMAND",
    severity = Severity::Warning,
    help = "retry the command",
    url = "https://example.com/errors/E_COMMAND"
)]
enum CommandError {
    Missing,
    #[diagnosis(
        description = "invalid configuration",
        code = "E_CONFIG",
        severity = Severity::Error
    )]
    InvalidConfig,
    #[diagnosis(description_from = path)]
    Load {
        path: String,
        #[diagnosis(source)]
        source: MischiefError,
    },
    #[diagnosis(description_from = 0)]
    Parse(String),
}

fn main() {
    let missing = CommandError::Missing;
    assert_eq!(missing.description(), "command failed");
    assert_eq!(missing.severity(), Some(Severity::Warning));
    assert_eq!(missing.help(), Some("retry the command"));

    let invalid = CommandError::InvalidConfig;
    assert_eq!(invalid.description(), "invalid configuration");
    assert_eq!(invalid.code(), Some("E_CONFIG"));
    assert_eq!(invalid.severity(), Some(Severity::Error));
    assert_eq!(invalid.url(), Some("https://example.com/errors/E_COMMAND"));

    let load = CommandError::Load {
        path: String::from("config.toml"),
        source: MischiefError::builder("permission denied").build(),
    };
    assert_eq!(load.description(), "config.toml");
    assert_eq!(load.code(), Some("E_COMMAND"));
    assert_eq!(
        load.source().map(IDiagnosis::description),
        Some("permission denied")
    );

    let parse = CommandError::Parse(String::from("unexpected token"));
    assert_eq!(parse.description(), "unexpected token");
}

#[test]
fn test() { main(); }
