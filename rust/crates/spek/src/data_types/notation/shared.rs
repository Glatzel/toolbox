use strum::{AsRefStr, EnumString};

use crate::data_types::notation::NotationError;

#[derive(Debug, Copy, Clone, AsRefStr, EnumString, PartialEq, Eq)]
pub enum Pitch {
    #[strum(serialize = "C", serialize = "c")]
    C,
    #[strum(serialize = "D", serialize = "d")]
    D,
    #[strum(serialize = "E", serialize = "e")]
    E,
    #[strum(serialize = "F", serialize = "f")]
    F,
    #[strum(serialize = "G", serialize = "g")]
    G,
    #[strum(serialize = "A", serialize = "a")]
    A,
    #[strum(serialize = "B", serialize = "b")]
    B,
}
impl From<Pitch> for u8 {
    fn from(value: Pitch) -> Self {
        match value {
            Pitch::C => 0,
            Pitch::D => 2,
            Pitch::E => 4,
            Pitch::F => 5,
            Pitch::G => 7,
            Pitch::A => 9,
            Pitch::B => 11,
        }
    }
}
impl TryFrom<char> for Pitch {
    type Error = NotationError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let result = match value {
            'C' | 'c' => Self::C,
            'D' | 'd' => Self::D,
            'E' | 'e' => Self::E,
            'F' | 'f' => Self::F,
            'G' | 'g' => Self::G,
            'A' | 'a' => Self::A,
            'B' | 'b' => Self::B,
            c => return Err(NotationError::UnknownPitch(c)),
        };
        Ok(result)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr, EnumString)]
#[repr(i8)]
pub enum Accidental {
    #[strum(serialize = "♯", serialize = "#")]
    Sharp = 1,

    #[strum(serialize = "♭", serialize = "b", serialize = "!")]
    Flat = -1,

    #[strum(serialize = "𝄪")]
    DoubleSharp = 2,

    #[strum(serialize = "𝄫")]
    DoubleFlat = -2,

    #[strum(serialize = "♮", serialize = "n")]
    Natural = 0,
}
impl TryFrom<char> for Accidental {
    type Error = NotationError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let result = match value {
            '♯' | '#' => Self::Sharp,
            '♭' | 'b' | '!' => Self::Flat,
            '𝄪' => Self::DoubleSharp,
            '𝄫' => Self::DoubleFlat,
            '♮' | 'n' => Self::Natural,
            c => return Err(NotationError::UnknownAccidental(c)),
        };
        Ok(result)
    }
}
