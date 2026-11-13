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

    #[diagnosis(description = "Unknown pitch: {0}")]
    UnknownPitch(char),

    #[diagnosis(description = "Unknown accidental: {0}")]
    UnknownAccidental(char),

    #[diagnosis(description = "Invalid octave first char: {0}")]
    InvalidOctave(String),

    #[diagnosis(description = "Invalid note: {0}")]
    InvalidNote(String),

    #[diagnosis(description = "Invalid key: {0}")]
    InvalidKey(String),

    #[diagnosis(description = "Invalid key kind: {0}")]
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
    }
}
