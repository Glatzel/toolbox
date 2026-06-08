use chrono::NaiveDate;
use derive_getters::Getters;
use rax::string::{DecodeOptExt, Decoder, IDecode};

use crate::RaxNmeaError;
use crate::data::{PosMode, Status};
use crate::rules::*;

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
    pos_mode: Option<PosMode>,
}

impl IDecode<RaxNmeaError> for Rmc {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let time = parser.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime);
        let status = parser.take(&UNTIL_COMMA_DISCARD).decode_opt();
        let lat = parser.take(&NmeaCoord);
        let lon = parser.take(&NmeaCoord);
        let spd = parser.take(&UNTIL_COMMA_DISCARD).decode_opt();
        let cog = parser.take(&UNTIL_COMMA_DISCARD).decode_opt();
        let date = parser.take(&NmeaDate);
        let mv = parser.take(&NmeaDegree);
        let pos_mode = parser.take(&UNTIL_STAR_DISCARD).decode_opt();
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
        })
    }
}

#[cfg(test)]
mod test {
    use clerk::{LevelFilter, init_log_with_level};

    extern crate std;
    use std::println;
    use std::string::ToString;

    use super::*;
    #[test]
    fn test_new_rmc1() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPRMC,110125,A,5505.337580,N,03858.653666,E,148.8,84.6,310317,8.9,E,D*2E";
        let mut ctx = Decoder::new();
        let rmc = Rmc::decode(ctx.init(s.to_string()))?;
        println!("{rmc:?}");
        insta::assert_json_snapshot!(rmc);
        Ok(())
    }
    #[test]
    fn test_new_rmc2() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPRMC,,V,,,,,,,,,,N*53";
        let mut ctx = Decoder::new();
        let rmc = Rmc::decode(ctx.init(s.to_string()))?;
        println!("{rmc:?}");
        insta::assert_json_snapshot!(rmc);
        Ok(())
    }
}
