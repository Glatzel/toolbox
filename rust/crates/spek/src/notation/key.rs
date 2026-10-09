use std::fmt::Display;
use std::str::FromStr;

use strum::{AsRefStr, EnumString};

use crate::notation::{Accidental, NotationError, Pitch};
#[derive(Debug, Copy, Clone, AsRefStr, EnumString, PartialEq, Eq)]
pub enum KeyKindScale {
    #[strum(serialize = "major", serialize = "maj")]
    Major,
    #[strum(serialize = "minor", serialize = "min")]
    Minor,
}
#[derive(Debug, Copy, Clone, AsRefStr, EnumString, PartialEq, Eq)]
pub enum KeyKindMode {
    #[strum(serialize = "ionian")]
    Ionian,
    #[strum(serialize = "dorian")]
    Dorian,
    #[strum(serialize = "phrygian")]
    Phrygian,
    #[strum(serialize = "lydian")]
    Lydian,
    #[strum(serialize = "mixolydian")]
    Mixolydian,
    #[strum(serialize = "aeolian")]
    Aeolian,
    #[strum(serialize = "locrian")]
    Locrian,
}

impl KeyKindMode {
    pub const fn offset(&self) -> u8 {
        match self {
            Self::Ionian => 0,
            Self::Dorian => 1,
            Self::Phrygian => 2,
            Self::Lydian => 3,
            Self::Mixolydian => 4,
            Self::Aeolian => 5,
            Self::Locrian => 6,
        }
    }
}
#[derive(Debug, Copy, Clone, AsRefStr, EnumString, PartialEq, Eq)]
pub enum KeyKindMisc {
    #[strum(serialize = "phr")]
    Phr,
    #[strum(serialize = "mix")]
    Mix,
    #[strum(serialize = "aeo")]
    Aeo,
    #[strum(serialize = "loc")]
    Loc,
}
#[derive(Debug, Clone, AsRefStr, PartialEq, Eq)]
pub enum KeyKind {
    Scale(KeyKindScale),
    Mode(KeyKindMode),
    Misc(KeyKindMisc),
}
impl FromStr for KeyKind {
    type Err = NotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(key) = KeyKindScale::from_str(s) {
            Ok(Self::Scale(key))
        } else if let Ok(key) = KeyKindMode::from_str(s) {
            Ok(Self::Mode(key))
        } else if let Ok(key) = KeyKindMisc::from_str(s) {
            Ok(Self::Misc(key))
        } else {
            Err(NotationError::InvalidKeyKind(s.into()))
        }
    }
}
pub struct Key {
    pitch: Pitch,
    accs: Vec<Accidental>,
    key: KeyKind,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.pitch.as_ref())?;
        for a in &self.accs {
            f.write_str(a.as_ref())?;
        }
        f.write_str(":")?;
        f.write_str(self.key.as_ref())?;
        Ok(())
    }
}
impl FromStr for Key {
    type Err = NotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((first, s_key)) = s.split_once(':') {
            match first.split_at_checked(1) {
                Some((s_pitch, s_accs)) => {
                    let pitch = s_pitch.parse()?;
                    let accs = s_accs
                        .chars()
                        .map(Accidental::try_from)
                        .collect::<Result<Vec<_>, NotationError>>()?;
                    let key = s_key.parse()?;
                    Ok(Self { pitch, accs, key })
                }
                None => Err(NotationError::InvalidKey(s.into())),
            }
        } else {
            Err(NotationError::InvalidKey(s.into()))
        }
    }
}
impl Key {
    fn _to_notes(&self) { todo!() }
    fn _to_degrees(&self) { todo!() }
}
