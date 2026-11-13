mod key;
mod melakarta;
mod midi;
mod note;
mod shared;
mod thaat;
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
#[diagnosis(error, display)]
pub enum NotationError {
    #[diagnosis(transparent)]
    Verb(#[diagnosis(from)] VerbError),

    #[diagnosis(transparent(foreign))]
    ParseInt(#[diagnosis(from)] ParseIntError),

    #[diagnosis(transparent(foreign))]
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
