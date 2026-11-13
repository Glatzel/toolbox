mod key;
mod melakarta;
mod midi;
mod note;
mod shared;
mod thaat;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

pub use key::{Key, KeyMode, KeyQuality, KeyScale};
pub use melakarta::Melakarta;
pub use midi::Midi;
use mischief::IDiagnosis;
pub use note::Note;
use rax::error::VerbError;
pub use shared::{Accidental, Pitch};
pub use thaat::Thaat;

#[derive(Debug, PartialEq, Eq, IDiagnosis)]
pub enum NotationError {
    #[diagnosis(description = "notation parser error")]
    Verb(#[diagnosis(from)] VerbError),

    #[diagnosis(description = "notation integer parse error")]
    ParseInt(#[diagnosis(from)] ParseIntError),

    #[diagnosis(description = "notation enum parse error")]
    Strum(#[diagnosis(from)] strum::ParseError),

    #[diagnosis(description = "unknown pitch")]
    UnknownPitch(char),

    #[diagnosis(description = "unknown accidental")]
    UnknownAccidental(char),

    #[diagnosis(description = "invalid octave first character")]
    InvalidOctave(String),

    #[diagnosis(description = "invalid note")]
    InvalidNote(String),

    #[diagnosis(description = "invalid key")]
    InvalidKey(String),

    #[diagnosis(description = "invalid key kind")]
    InvalidKeyKind(String),
}

impl fmt::Display for NotationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Verb(source) => fmt::Display::fmt(source, f),
            Self::ParseInt(source) => fmt::Display::fmt(source, f),
            Self::Strum(source) => fmt::Display::fmt(source, f),
            Self::UnknownPitch(value) => write!(f, "Unknown pitch: {value}"),
            Self::UnknownAccidental(value) => write!(f, "Unknown accidental: {value}"),
            Self::InvalidOctave(value) => write!(f, "Invalid octave first char: {value}"),
            Self::InvalidNote(value) => write!(f, "Invalid note: {value}"),
            Self::InvalidKey(value) => write!(f, "Invalid key: {value}"),
            Self::InvalidKeyKind(value) => write!(f, "Invalid key kind: {value}"),
        }
    }
}

impl Error for NotationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Verb(source) => Error::source(source),
            Self::ParseInt(source) => Error::source(source),
            Self::Strum(source) => Error::source(source),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_is_unchanged() {
        assert_eq!(
            NotationError::UnknownPitch('H').to_string(),
            "Unknown pitch: H"
        );
        assert_eq!(
            NotationError::InvalidKeyKind(String::from("unknown")).to_string(),
            "Invalid key kind: unknown"
        );
    }

    #[test]
    fn transparent_variants_forward_display_and_source() {
        let parse = "not an integer".parse::<u8>().expect_err("parse must fail");
        let expected = parse.to_string();
        let error = NotationError::from(parse);

        assert_eq!(error.to_string(), expected);
        assert!(Error::source(&error).is_none());
    }
}
