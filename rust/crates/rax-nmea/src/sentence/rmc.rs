use core::str::FromStr;

use derive_getters::Getters;
use jiff::civil::{Date, Time};
use rax::text::StrParser;

use crate::RaxNmeaError;
use crate::common::{FaaMode, Status};
use crate::rules::{
    NmeaCoord, NmeaDate, NmeaDegree, NmeaTime, UNTIL_COMMA_DISCARD, UNTIL_COMMA_OR_STAR_KEEP_RIGHT,
    UNTIL_STAR_DISCARD,
};
use crate::utils::ParseOptionPrimitive;
/// Navigational status reported in the RMC sentence.
#[derive(Debug, PartialEq, Eq, Clone, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RmcNavigationStatus {
    /// Autonomous position solution (`A`).
    #[strum(serialize = "Autonomous", serialize = "A")]
    Autonomous,
    /// Differential position solution (`D`).
    #[strum(serialize = "Differential", serialize = "D")]
    Differential,
    /// Estimated (dead reckoning) position (`E`).
    #[strum(serialize = "Estimated", serialize = "E")]
    Estimated,
    /// Manual input mode (`M`).
    #[strum(serialize = "Manual Input Mode", serialize = "M")]
    ManualInputMode,
    /// Position is not valid (`N`).
    #[strum(serialize = "Not Valid", serialize = "N")]
    NotValid,
    /// Simulated position (`S`).
    #[strum(serialize = "Simulator", serialize = "S")]
    Simulator,
    /// Position is valid (`V`).
    #[strum(serialize = "Valid", serialize = "V")]
    Valid,
}
/// Recommended minimum data.
///
/// Time, date, position, course, and speed reported by a GNSS receiver.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Rmc {
    /// UTC time of the position fix
    time: Option<Time>,

    /// Status
    status: Option<Status>,

    /// Latitude
    lat: Option<f64>,

    /// Longitude
    lon: Option<f64>,

    /// Speed over ground
    spd: Option<f64>,

    /// Track made good
    cog: Option<f64>,

    /// Date
    date: Option<Date>,

    /// Magnetic variation
    mv: Option<f64>,

    /// FAA mode
    pos_mode: Option<FaaMode>,

    /// Navigation status
    nav_status: Option<RmcNavigationStatus>,
}

impl FromStr for Rmc {
    type Err = RaxNmeaError;
    fn from_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut parser = StrParser::new(input);
        let time = parser.skip(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime)?;
        let status = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let lat = parser.take(&NmeaCoord)?;
        let lon = parser.take(&NmeaCoord)?;
        let spd = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let cog = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let date = parser.take(&NmeaDate)?;
        let mv = parser.take(&NmeaDegree)?;
        let pos_mode = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        let _ = parser.take(&UNTIL_COMMA_DISCARD);
        let nav_status = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Self {
            time,
            status,
            lat,
            lon,
            spd,
            cog,
            date,
            mv,
            pos_mode,
            nav_status,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_sentence;

    test_sentence!(
        test_rmc1,
        1,
        Rmc,
        "$GPRMC,110125,A,5505.337580,N,03858.653666,E,148.8,84.6,310317,8.9,E,D*2E"
    );
    test_sentence!(test_rmc2, 2, Rmc, "$GPRMC,,V,,,,,,,,,,N*53");
    test_sentence!(test_rmc3, 3, Rmc, "$GPRMC,,V,,,,,,,,,,N,V*29");
}
