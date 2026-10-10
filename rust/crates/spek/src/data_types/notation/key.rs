use std::fmt::Display;
use std::str::FromStr;

use strum::{AsRefStr, EnumString};

use crate::data_types::notation::{Accidental, NotationError, Pitch};
#[derive(Debug, Copy, Clone, AsRefStr, EnumString, PartialEq, Eq)]
pub enum KeyScale {
    #[strum(serialize = "major", serialize = "maj")]
    Major,
    #[strum(serialize = "minor", serialize = "min")]
    Minor,
}
#[derive(Debug, Copy, Clone, AsRefStr, EnumString, PartialEq, Eq)]
pub enum KeyMode {
    #[strum(serialize = "ionian", serialize = "ion")]
    Ionian,
    #[strum(serialize = "dorian", serialize = "dor")]
    Dorian,
    #[strum(serialize = "phrygian", serialize = "phryg", serialize = "phr")]
    Phrygian,
    #[strum(serialize = "lydian", serialize = "lyd")]
    Lydian,
    #[strum(serialize = "mixolydian", serialize = "mixolyd", serialize = "mix")]
    Mixolydian,
    #[strum(serialize = "aeolian", serialize = "aeol", serialize = "aeo")]
    Aeolian,
    #[strum(serialize = "locrian", serialize = "locr", serialize = "loc")]
    Locrian,
}

impl KeyMode {
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
#[derive(Debug, Clone, AsRefStr, PartialEq, Eq)]
pub enum KeyQuality {
    Scale(KeyScale),
    Mode(KeyMode),
}
impl FromStr for KeyQuality {
    type Err = NotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        KeyScale::from_str(s).map_or_else(
            |_| {
                KeyMode::from_str(s).map_or_else(
                    |_| Err(NotationError::InvalidKeyKind(s.into())),
                    |key| Ok(Self::Mode(key)),
                )
            },
            |key| Ok(Self::Scale(key)),
        )
    }
}
pub struct Key {
    pitch: Pitch,
    accs: Vec<Accidental>,
    key: KeyQuality,
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
