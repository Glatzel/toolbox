use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::{AsRefStr, EnumString};

use crate::data_types::notation::NotationError;

#[derive(
    Debug, Copy, Clone, AsRefStr, EnumString, PartialEq, Eq, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u8)]
pub enum Pitch {
    #[strum(serialize = "C", serialize = "c")]
    C = 0,
    #[strum(serialize = "D", serialize = "d")]
    D = 2,
    #[strum(serialize = "E", serialize = "e")]
    E = 4,
    #[strum(serialize = "F", serialize = "f")]
    F = 5,
    #[strum(serialize = "G", serialize = "g")]
    G = 7,
    #[strum(serialize = "A", serialize = "a")]
    A = 9,
    #[strum(serialize = "B", serialize = "b")]
    B = 11,
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
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, AsRefStr, EnumString, TryFromPrimitive, IntoPrimitive,
)]
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
