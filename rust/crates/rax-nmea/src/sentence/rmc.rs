use chrono::NaiveDate;
use derive_getters::Getters;
use rax::string::{Decoder, IDecode};

use crate::RaxNmeaError;
use crate::common::{FaaMode, Status};
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;
#[derive(Debug, PartialEq, Clone, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RmcNavigationStatus {
    #[strum(serialize = "Autonomous", serialize = "A")]
    Autonomous,
    #[strum(serialize = "Differential", serialize = "D")]
    Differential,
    #[strum(serialize = "Estimated", serialize = "E")]
    Estimated,
    #[strum(serialize = "Manual Input Mode", serialize = "M")]
    ManualInputMode,
    #[strum(serialize = "Not Valid", serialize = "N")]
    NotValid,
    #[strum(serialize = "Simulator", serialize = "S")]
    Simulator,
    #[strum(serialize = "Valid", serialize = "V")]
    Valid,
}
#[doc = "Recommended minimum data"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Rmc {
    /// UTC time of the position fix
    time: Option<chrono::NaiveTime>,

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
    date: Option<NaiveDate>,

    /// Magnetic variation
    mv: Option<f64>,

    /// FAA mode
    pos_mode: Option<FaaMode>,

    /// Navigation status
    nav_status: Option<RmcNavigationStatus>,
}

impl IDecode<RaxNmeaError> for Rmc {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
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

        Ok(Rmc {
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
    use clerk::{LevelFilter, init_log_with_level};

    extern crate std;
    use std::println;

    use super::*;
    #[rstest::rstest]
    #[case(
        "1",
        "$GPRMC,110125,A,5505.337580,N,03858.653666,E,148.8,84.6,310317,8.9,E,D*2E"
    )]
    #[case("2", "$GPRMC,,V,,,,,,,,,,N*53")]
    #[case("3", "$GPRMC,,V,,,,,,,,,,N,V*29")]
    fn test_rmc(#[case] index: &str, #[case] input: &str) -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = Decoder::new(input);
        let rmc = Rmc::decode(&mut decoder)?;
        println!("{rmc:?}");
        insta::assert_json_snapshot!(index, rmc);
        Ok(())
    }
}
