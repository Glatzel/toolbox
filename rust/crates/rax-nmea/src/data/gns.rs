use core::fmt::Debug;
use core::str::FromStr;
extern crate alloc;
use alloc::string::ToString;
use alloc::vec::Vec;

use derive_getters::Getters;
use rax::str_parser::{ParseOptExt, StrParserContext};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::data::{INmeaData, PosMode, Talker};
use crate::rules::*;
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NavigationStatus {
    Safe,
    Caution,
    Unsafe,
    Invalid,
}
impl FromStr for NavigationStatus {
    type Err = RaxNmeaError;

    fn from_str(s: &str) -> Result<Self, RaxNmeaError> {
        match s {
            "S" => Ok(Self::Safe),
            "C" => Ok(Self::Caution),
            "U" => Ok(Self::Unsafe),
            "V" => Ok(Self::Invalid),
            _ => Err(RaxNmeaError::UnknownNavigationStatus(s.to_string())),
        }
    }
}
///GNSS fix data
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Gns {
    talker: Talker,
    /// UTC time of the position fix
    time: Option<chrono::NaiveTime>,
    /// Latitude, ddmm.mmmm, where dd is degrees and mm.mmmm is minutes.
    /// Positive values indicate North, negative values indicate South.
    lat: Option<f64>,
    ///Longitude, dddmm.mmmm, where ddd is degrees and mm.mmmm is minutes.
    /// Positive values indicate East, negative values indicate West.
    lon: Option<f64>,
    /// FAA mode
    pos_mode: Vec<PosMode>,
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

impl INmeaData for Gns {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> Result<Self, RaxNmeaError> {
        clerk::trace!("Gga::new: sentence='{}'", ctx.full_str());

        ctx.global(&NmeaValidate)?;

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
            .filter_map(|(_, c)| PosMode::try_from(&c).ok())
            .collect::<Vec<PosMode>>();
        clerk::debug!("mode: {:?}", pos_mode);

        clerk::debug!("Parsing satellites...");
        let num_sv = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("satellites: {:?}", num_sv);

        clerk::debug!("Parsing hdop...");
        let hdop = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("hdop: {:?}", hdop);

        clerk::debug!("Parsing altitude...");
        let alt = ctx.take(&UNTIL_COMMA_OR_STAR_DISCARD).parse_opt();
        clerk::debug!("altitude: {:?}", alt);

        clerk::debug!("Parsing goeidal_separation...");
        let sep = ctx.take(&UNTIL_COMMA_OR_STAR_DISCARD).parse_opt();
        clerk::debug!("goeidal_separation: {:?}", sep);

        clerk::debug!("Parsing differential_data_age...");
        let diff_age = ctx.take(&UNTIL_COMMA_OR_STAR_DISCARD).parse_opt();
        clerk::debug!("differential_data_age: {:?}", diff_age);

        clerk::debug!("Parsing differential_reference_station_id...");

        let diff_station = ctx.take(&UNTIL_COMMA_OR_STAR_DISCARD).parse_opt();

        clerk::debug!("differential_reference_station_id: {:?}", diff_station);

        clerk::debug!("Parsing navigational_status...");
        let nav_status = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();
        clerk::debug!("navigational_status: {:?}", nav_status);

        Ok(Gns {
            talker,
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
impl Debug for Gns {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("GNS");
        ds.field("talker", &self.talker);
        if let Some(ref time) = self.time {
            ds.field("time", time);
        }
        if let Some(lat) = self.lat {
            ds.field("lat", &lat);
        }
        if let Some(lon) = self.lon {
            ds.field("lon", &lon);
        }
        ds.field("pos_mode", &self.pos_mode);
        if let Some(num_sv) = self.num_sv {
            ds.field("num_sv", &num_sv);
        }
        if let Some(hdop) = self.hdop {
            ds.field("hdop", &hdop);
        }
        if let Some(alt) = self.alt {
            ds.field("alt", &alt);
        }
        if let Some(sep) = self.sep {
            ds.field("sep", &sep);
        }
        if let Some(diff_age) = self.diff_age {
            ds.field("diff_age", &diff_age);
        }
        if let Some(diff_station) = self.diff_station {
            ds.field("diff_station", &diff_station);
        }
        if let Some(nav_status) = &self.nav_status {
            ds.field("nav_status", nav_status);
        }
        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use clerk::{LogLevel, init_log_with_level};
    extern crate std;
    use std::println;
    use std::string::ToString;

    use super::*;
    use crate::data::Talker;
    #[test]
    fn test_gns_parsing1() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGNS,112257.00,3844.24011,N,00908.43828,W,AN,03,10.5,,*57";
        let mut ctx = StrParserContext::new();
        let gns = Gns::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{gns:?}");
        insta::assert_debug_snapshot!(gns);

        Ok(())
    }
    #[test]
    fn test_gns_parsing2() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GNGNS,181604.00,,,,,NN,00,99.99,,,,*59";
        let mut ctx = StrParserContext::new();
        let gns = Gns::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{gns:?}");
        insta::assert_debug_snapshot!(gns);
        Ok(())
    }
}
