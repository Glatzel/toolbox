mod key;
mod note;
mod shared;

use std::num::ParseIntError;

pub use key::{Key, KeyKind, KeyKindMisc, KeyKindMode, KeyKindScale};
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

fn _mela_to_svara() { todo!() }
fn _mela_to_degrees() { todo!() }
fn _thaat_to_degrees() { todo!() }
fn _list_mela() { todo!() }
fn _list_thaat() { todo!() }
fn _fifths_to_note() { todo!() }
fn _interval_to_fjs() { todo!() }
fn _interval_frequencies() { todo!() }
