mod key;
mod note;
mod shared;
mod midi;

use std::num::ParseIntError;

pub use key::{Key, KeyKind, KeyKindMisc, KeyKindMode, KeyKindScale};
pub use midi::Midi;
pub use note::Note;
use rax::error::VerbError;
pub use shared::{Accidental, Pitch};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum NotationError {
    #[error(transparent)]
    Verb(#[from] VerbError),

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    Strum(#[from] strum::ParseError),

    #[error("Unknown pitch: {0}")]
    UnknownPitch(char),

    #[error("Unknown accidental: {0}")]
    UnknownAccidental(char),

    #[error("Invalid octave first char: {0}")]
    InvalidOctave(String),

    #[error("Invalid note: {0}")]
    InvalidNote(String),

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Invalid key kind: {0}")]
    InvalidKeyKind(String),
}
