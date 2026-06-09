extern crate alloc;

use alloc::vec::Vec;
use core::fmt::Debug;

use derive_getters::Getters;
use rax::string::{Decoder, IDecode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::common::FaaMode;
use crate::rules::*;

#[derive(Debug, PartialEq, Clone, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NavigationStatus {
    #[strum(serialize = "Safe", serialize = "S")]
    Safe,
    #[strum(serialize = "Caution", serialize = "C")]
    Caution,
    #[strum(serialize = "Unsafe", serialize = "U")]
    Unsafe,
    #[strum(serialize = "Invalid", serialize = "V")]
    Invalid,
}

///GNSS fix data
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gns {
    /// UTC time of the position fix
    time: Option<chrono::NaiveTime>,
    /// Latitude, ddmm.mmmm, where dd is degrees and mm.mmmm is minutes.
    /// Positive values indicate North, negative values indicate South.
    lat: Option<f64>,
    ///Longitude, dddmm.mmmm, where ddd is degrees and mm.mmmm is minutes.
    /// Positive values indicate East, negative values indicate West.
    lon: Option<f64>,
    /// FAA mode
    pos_mode: Vec<FaaMode>,
    /// Number of satellites in use
    num_sv: Option<u8>,
    /// Horizontal dilution of precision
    hdop: Option<f64>,
    /// Altitude
    alt: Option<f64>,
    /// Geoidal separation
    sep: Option<f64>,
    /// Differential data age
    diff_age: Option<f64>,
    /// Differential reference station ID
    diff_station: Option<u16>,
    /// Navigational status
    nav_status: Option<NavigationStatus>,
}

impl IDecode<RaxNmeaError> for Gns {
    fn decode(ctx: &mut Decoder) -> Result<Self, RaxNmeaError> {
        clerk::trace!("Gga::decode: sentence='{}'", ctx.full_str());

        clerk::debug!("Parsing utc_time...");
        let time = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime);
        clerk::debug!("utc_time: {:?}", time);

        clerk::debug!("Parsing lat...");
        let lat = ctx.take(&NmeaCoord);
        clerk::debug!("lat: {:?}", lat);

        clerk::debug!("Parsing lon...");
        let lon = ctx.take(&NmeaCoord);
        clerk::debug!("lon: {:?}", lon);

        clerk::debug!("Parsing mode...");
        let mode_str = ctx
            .take(&UNTIL_COMMA_DISCARD)
            .expect("Mode string should not be empty.");
        let pos_mode = mode_str
            .char_indices()
            .filter_map(|(_, c)| FaaMode::try_from(&c).ok())
            .collect::<Vec<FaaMode>>();
        clerk::debug!("mode: {:?}", pos_mode);

        clerk::debug!("Parsing satellites...");
        let num_sv = ctx.take(&UNTIL_COMMA_DISCARD).and_then(|s| s.parse().ok());
        clerk::debug!("satellites: {:?}", num_sv);

        clerk::debug!("Parsing hdop...");
        let hdop = ctx.take(&UNTIL_COMMA_DISCARD).and_then(|s| s.parse().ok());
        clerk::debug!("hdop: {:?}", hdop);

        clerk::debug!("Parsing altitude...");
        let alt = ctx
            .take(&UNTIL_COMMA_OR_STAR_DISCARD)
            .and_then(|s| s.parse().ok());
        clerk::debug!("altitude: {:?}", alt);

        clerk::debug!("Parsing goeidal_separation...");
        let sep = ctx
            .take(&UNTIL_COMMA_OR_STAR_DISCARD)
            .and_then(|s| s.parse().ok());
        clerk::debug!("goeidal_separation: {:?}", sep);

        clerk::debug!("Parsing differential_data_age...");
        let diff_age = ctx
            .take(&UNTIL_COMMA_OR_STAR_DISCARD)
            .and_then(|s| s.parse().ok());
        clerk::debug!("differential_data_age: {:?}", diff_age);

        clerk::debug!("Parsing differential_reference_station_id...");

        let diff_station = ctx
            .take(&UNTIL_COMMA_OR_STAR_DISCARD)
            .and_then(|s| s.parse().ok());

        clerk::debug!("differential_reference_station_id: {:?}", diff_station);

        clerk::debug!("Parsing navigational_status...");
        let nav_status = ctx.take(&UNTIL_STAR_DISCARD).and_then(|s| s.parse().ok());
        clerk::debug!("navigational_status: {:?}", nav_status);

        Ok(Gns {
            time,
            lat,
            lon,
            pos_mode,
            num_sv,
            hdop,
            alt,
            sep,
            diff_age,
            diff_station,
            nav_status,
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
    fn test_gns_parsing1() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGNS,112257.00,3844.24011,N,00908.43828,W,AN,03,10.5,,*57";
        let mut ctx = Decoder::new();
        let gns = Gns::decode(ctx.init(s.to_string()))?;
        println!("{gns:?}");
        insta::assert_json_snapshot!(gns);

        Ok(())
    }
    #[test]
    fn test_gns_parsing2() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GNGNS,181604.00,,,,,NN,00,99.99,,,,*59";
        let mut parser = Decoder::new();
        let gns = Gns::decode(parser.init(s.to_string()))?;
        println!("{gns:?}");
        insta::assert_json_snapshot!(gns);
        Ok(())
    }
}
