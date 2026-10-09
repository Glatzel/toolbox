use std::fmt::Display;
use std::iter::Sum;
use std::num::ParseIntError;
use std::str::FromStr;

use generic_num::num;
use num_traits::Float;
use rax::error::VerbError;
use rax::text::filters::{AsciiCharSetFilter, CHAR_SET_DIGITS, CharSetFilter};
use rax::text::{OneOfCharSet, StrParser, UntilMode, UntilNotInCharSet};
use strum::{AsRefStr, EnumString};
use thiserror::Error;

use crate::convert::frequency_unit::midi_to_hz;
#[derive(Debug, Error, PartialEq, Eq)]
pub enum NoteError {
    #[error(transparent)]
    Verb(#[from] VerbError),

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error("Unknown pitch: {0}")]
    UnknownPitch(char),

    #[error("Unknown accidental: {0}")]
    UnknownAccidental(char),

    #[error("Invalid octave first char: {0}")]
    InvalidOctave(String),

    #[error("Invalid note: {0}")]
    InvalidNote(String),
}

#[allow(non_camel_case_types, reason = "pitch has lower case signal.")]
#[derive(Debug, Copy, Clone, AsRefStr, EnumString, PartialEq, Eq)]
pub enum Pitch {
    #[strum(serialize = "c")]
    c,
    #[strum(serialize = "d")]
    d,
    #[strum(serialize = "e")]
    e,
    #[strum(serialize = "f")]
    f,
    #[strum(serialize = "g")]
    g,
    #[strum(serialize = "a")]
    a,
    #[strum(serialize = "b")]
    b,
    #[strum(serialize = "C")]
    C,
    #[strum(serialize = "D")]
    D,
    #[strum(serialize = "E")]
    E,
    #[strum(serialize = "F")]
    F,
    #[strum(serialize = "G")]
    G,
    #[strum(serialize = "A")]
    A,
    #[strum(serialize = "B")]
    B,
}
impl From<Pitch> for u8 {
    fn from(value: Pitch) -> Self {
        match value {
            Pitch::c | Pitch::C => 0,
            Pitch::d | Pitch::D => 2,
            Pitch::e | Pitch::E => 4,
            Pitch::f | Pitch::F => 5,
            Pitch::g | Pitch::G => 7,
            Pitch::a | Pitch::A => 9,
            Pitch::b | Pitch::B => 11,
        }
    }
}
impl TryFrom<char> for Pitch {
    type Error = NoteError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let result = match value {
            'c' => Self::c,
            'd' => Self::d,
            'e' => Self::e,
            'f' => Self::f,
            'g' => Self::g,
            'a' => Self::a,
            'b' => Self::b,
            'C' => Self::C,
            'D' => Self::D,
            'E' => Self::E,
            'F' => Self::F,
            'G' => Self::G,
            'A' => Self::A,
            'B' => Self::B,
            c => return Err(NoteError::UnknownPitch(c)),
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
    type Error = NoteError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let result = match value {
            '♯' | '#' => Self::Sharp,
            '♭' | 'b' | '!' => Self::Flat,
            '𝄪' => Self::DoubleSharp,
            '𝄫' => Self::DoubleFlat,
            '♮' | 'n' => Self::Natural,
            c => return Err(NoteError::UnknownAccidental(c)),
        };
        Ok(result)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
        let cents = self.cents.map_or_else(T::zero, |c| num!(c) / num!(100));
        let offset: T = self.accs.iter().map(|a| num!(*a as u8)).sum();
        num!(12) * (num!(self.octave.unwrap_or_default()) + T::one())
            + num!(u8::from(self.pitch))
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
impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.pitch.as_ref())?;
        for a in &self.accs {
            f.write_str(a.as_ref())?;
        }
        if let Some(octave) = self.octave {
            f.write_str(&octave.to_string())?;
        }
        if let Some(cents) = self.cents {
            f.write_str(&format!("{cents:+}"))?;
        }
        Ok(())
    }
}
impl FromStr for Note {
    type Err = NoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const PITCH_FILTER: AsciiCharSetFilter<14> = AsciiCharSetFilter::new([
            'c', 'd', 'e', 'f', 'g', 'a', 'b', 'C', 'D', 'E', 'F', 'G', 'A', 'B',
        ]);
        const PITCH_RULE: OneOfCharSet<'_, true, 14, AsciiCharSetFilter<14>> =
            OneOfCharSet(&PITCH_FILTER);
        const ACCIDENTAL_FILTER: CharSetFilter<9> =
            CharSetFilter::new(['♯', '#', '♭', 'b', '!', '𝄪', '𝄫', '♮', 'n']);
        const ACCIDENTAL_RULE: UntilNotInCharSet<'_, false, 9, CharSetFilter<9>> =
            UntilNotInCharSet {
                filter: &ACCIDENTAL_FILTER,
                mode: UntilMode::KeepInRest,
            };
        pub const CHAR_SET_NUMBER: AsciiCharSetFilter<12> =
            AsciiCharSetFilter::new(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '+']);
        const OCTAVE_FIRST_RULE: OneOfCharSet<'_, true, 12, AsciiCharSetFilter<12>> =
            OneOfCharSet(&CHAR_SET_NUMBER);
        const NUMBER_RULE: UntilNotInCharSet<'_, true, 10, AsciiCharSetFilter<10>> =
            UntilNotInCharSet {
                filter: &CHAR_SET_DIGITS,
                mode: UntilMode::KeepInRest,
            };
        pub const CENTS_SIGNAL: AsciiCharSetFilter<2> = AsciiCharSetFilter::new(['-', '+']);
        const CENTS_SIGNAL_RULE: OneOfCharSet<'_, true, 2, AsciiCharSetFilter<2>> =
            OneOfCharSet(&CENTS_SIGNAL);

        let mut parser = StrParser::new(s);
        let pitch = parser.take(&PITCH_RULE).map(Pitch::try_from)??;
        let accs = parser.take(&ACCIDENTAL_RULE).map(|a| {
            a.chars()
                .map(Accidental::try_from)
                .collect::<Result<Vec<_>, NoteError>>()
        })??;
        if parser.rest_str().is_empty() {
            return Ok(Self {
                pitch,
                accs,
                octave: None,
                cents: None,
            });
        }
        let octave = match parser.take(&OCTAVE_FIRST_RULE) {
            Ok(c) => parser
                .take(&NUMBER_RULE)
                .map(|d| format!("{c}{d}").parse())??,
            Err(_) => return Err(NoteError::InvalidOctave(parser.full_str().into())),
        };
        if parser.rest_str().is_empty() {
            return Ok(Self {
                pitch,
                accs,
                octave: Some(octave),
                cents: None,
            });
        }
        let cents = match parser.take(&CENTS_SIGNAL_RULE) {
            Ok(c) => parser
                .take(&NUMBER_RULE)
                .map(|d| format!("{c}{d}").parse())??,
            Err(_) => return Err(NoteError::InvalidOctave(parser.full_str().into())),
        };
        if !parser.rest_str().is_empty() {
            return Err(NoteError::InvalidNote(parser.full_str().into()));
        }
        Ok(Self {
            pitch,
            accs,
            octave: Some(octave),
            cents: Some(cents),
        })
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
    // Valid: note
    #[case("note_c", "C", true)]
    #[case("note_lowercase", "c", true)]
    #[case("note_g", "G", true)]
    #[case("note_lowercase_g", "g", true)]
    // Valid: accidentals
    #[case("sharp", "C#", true)]
    #[case("sharp_unicode", "C♯", true)]
    #[case("double_sharp", "C𝄪", true)]
    #[case("flat", "Cb", true)]
    #[case("flat_unicode", "C♭", true)]
    #[case("double_flat", "C𝄫", true)]
    #[case("natural", "C♮", true)]
    #[case("natural_n", "Cn", true)]
    #[case("exclamation", "C!", true)]
    // Valid: repeated accidentals
    #[case("multiple_accidentals", "C##", true)]
    #[case("sharp_flat", "C#b", true)]
    #[case("multiple_naturals", "C♮n", true)]
    #[case("mixed_accidentals", "C#♭n", true)]
    // Valid: octave
    #[case("octave_zero", "C0", true)]
    #[case("octave_positive", "C4", true)]
    #[case("octave_negative", "C-1", true)]
    #[case("octave_explicit_positive", "C+4", true)]
    #[case("octave_two_digit", "C10", true)]
    #[case("octave_negative_two_digit", "C-10", true)]
    // Valid: accidental + octave
    #[case("sharp_octave", "C#4", true)]
    #[case("flat_octave", "Cb4", true)]
    #[case("double_sharp_octave", "C𝄪4", true)]
    #[case("double_flat_octave", "C𝄫4", true)]
    #[case("sharp_negative_octave", "C♯-1", true)]
    #[case("flat_positive_octave", "C♭+4", true)]
    // Valid: combined
    #[case("sharp_octave_cents", "C#4+25", true)]
    #[case("sharp_octave_negative_cents", "C#-4-25", true)]
    #[case("flat_octave_cents", "Cb+4+25", true)]
    #[case("double_sharp_octave_cents", "C𝄪4-50", true)]
    // Invalid: empty
    #[case("empty", "", false)]
    #[case("whitespace", " ", false)]
    #[case("leading_whitespace", " C", false)]
    #[case("trailing_whitespace", "C ", false)]
    #[case("both_whitespace", " C ", false)]
    // Invalid: note
    #[case("invalid_note", "H", false)]
    #[case("invalid_note_lowercase", "h", false)]
    #[case("number_as_note", "4", false)]
    #[case("missing_note", "#4", false)]
    #[case("unicode_note", "Ç", false)]
    // Invalid: accidental
    #[case("invalid_accidental", "Cx", false)]
    #[case("invalid_accidental_number", "C2#", false)]
    #[case("invalid_accidental_symbol", "C@", false)]
    #[case("leading_accidental", "#C", false)]
    #[case("leading_flat", "♭C", false)]
    // Invalid: octave
    #[case("octave_without_note", "4", false)]
    #[case("octave_decimal", "C4.5", false)]
    #[case("octave_space", "C 4", false)]
    #[case("octave_double_sign", "C++4", false)]
    #[case("octave_double_negative", "C--4", false)]
    #[case("octave_sign_only", "C+", false)]
    #[case("octave_negative_sign_only", "C-", false)]
    // Invalid: cents
    #[case("cents_decimal", "C+2.5", false)]
    #[case("cents_sign_only", "C+", false)]
    #[case("cents_double_sign", "C++25", false)]
    #[case("cents_with_space", "C+ 25", false)]
    // Invalid: malformed combined forms
    #[case("octave_then_accidental", "C4#", false)]
    #[case("octave_then_flat", "C4b", false)]
    #[case("octave_then_natural", "C4n", false)]
    #[case("cents_then_accidental", "C+25#", false)]
    #[case("trailing_plus", "C4+", false)]
    #[case("trailing_minus", "C4-", false)]
    #[case("double_cents", "C4+25+10", false)]
    fn test_note_from_str(#[case] name: &str, #[case] input: &str, #[case] valid: bool) {
        let result: Result<Note, NoteError> = input.parse();
        if valid {
            let result = result.unwrap();
            insta::assert_snapshot!(
                format!("test_note_from_str{name}"),
                format!("input:{input}\n{result:?}\ndisplay:{result}")
            );
            let display = result.to_string();
            let reparse: Note = display.parse().unwrap();
            assert_eq!(reparse, result);
        } else {
            println!("{name}");
            assert!(result.is_err())
        }
    }

    #[rstest]
    #[case("C",Note{ pitch: Pitch::C, accs: vec![], octave: None, cents: None })]
    #[case("C_sharp_3",Note{ pitch: Pitch::C, accs: vec![Accidental::Sharp], octave: Some(3), cents: None })]
    fn test_to_midi(#[case] name: &str, #[case] note: Note) {
        let result: f32 = note.to_midi();
        insta::assert_snapshot!(
            format!("test_to_midi{name}"),
            format!("{note:?}{result:.0}")
        )
    }
}
