use std::iter::Sum;

use generic_num::num;
use num_traits::Float;
use strum::EnumString;

use crate::convert::frequency_unit::midi_to_hz;

#[derive(Debug, Copy, Clone, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum Pitch {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
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

    #[strum(serialize = "♮", serialize = "")]
    Natural = 0,
}
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: Pitch,
    pub accs: Vec<Accidental>,
    pub octave: Option<i8>,
    pub cents: Option<i8>,
}
impl Note {
    pub fn to_midi<T>(&self) -> T
    where
        T: Float + Sum,
    {
        let cents = match self.cents {
            Some(c) => num!(c) / num!(100),
            None => T::zero(),
        };
        let offset: T = self.accs.iter().map(|a| num!(*a as u8)).sum();
        num!(12) * (num!(self.octave.unwrap_or_default()) + T::one())
            + num!(self.pitch as u8)
            + offset
            + cents
    }
    pub fn to_hz<T>(&self) -> T
    where
        T: Float + Sum,
    {
        midi_to_hz(self.to_midi())
    }
}

fn _key_to_notes() { todo!() }
fn _key_to_degrees() { todo!() }
fn _mela_to_svara() { todo!() }
fn _mela_to_degrees() { todo!() }
fn _thaat_to_degrees() { todo!() }
fn _list_mela() { todo!() }
fn _list_thaat() { todo!() }
fn _fifths_to_note() { todo!() }
fn _interval_to_fjs() { todo!() }
fn _interval_frequencies() { todo!() }
#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case("C",Note{ pitch: Pitch::C, accs: vec![], octave: None, cents: None })]
    #[case("C_sharp_3",Note{ pitch: Pitch::C, accs: vec![Accidental::Sharp], octave: Some(3), cents: None })]
    fn test_to_midi(#[case] name: &str, #[case] note: Note) {
        let result: f32 = note.to_midi();
        insta::assert_snapshot!(name, format!("{note:?}{result:.0}"))
    }
}
