use core::fmt::Debug;
use core::str::FromStr;
extern crate alloc;
use alloc::string::ToString;
use alloc::vec::Vec;

use rax::str_parser::{ParseOptExt, StrParserContext};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::data::{INmeaData, PosMode, Talker};
use crate::macros::readonly_struct;
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
readonly_struct!(
    Gns,
    "GNSS fix data",
    {talker: Talker},

    {
        time:Option<chrono::NaiveTime>,
        "UTC time of the position fix"
    },
    {
        lat: Option<f64>,
        "Latitude, ddmm.mmmm, where dd is degrees and mm.mmmm is minutes. Positive values indicate North, negative values indicate South."
    },
    {
        lon: Option<f64>,
        "Longitude, dddmm.mmmm, where ddd is degrees and mm.mmmm is minutes. Positive values indicate East, negative values indicate West."
    },
    {
        pos_mode: Vec<PosMode>,
        "FAA mode"
    },
    {
        num_sv :Option<u8>,
        "Number of satellites in use"
    },
    {
        hdop:Option<f64>,
        "Horizontal dilution of precision"
    },
    {
        alt:Option<f64>,
        "Altitude"
    },
    {
        sep:Option<f64>,
        "Geoidal separation"
    },
    {
        diff_age:Option<f64>,
        "Differential data age"
    },
    {
        diff_station:Option<u16>,
        "Differential reference station ID"
    },
    {
        nav_status:Option<NavigationStatus>,
        "Navigational status"
    }
);

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
    use float_cmp::assert_approx_eq;
    extern crate std;
    use std::println;
    use std::string::ToString;

    use super::*;
    use crate::data::{PosMode, Talker};
    #[test]
    fn test_gns_parsing1() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGNS,112257.00,3844.24011,N,00908.43828,W,AN,03,10.5,,*57";
        let mut ctx = StrParserContext::new();
        let gns = Gns::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{gns:?}");
        assert_eq!(gns.talker, Talker::GN);
        assert!(gns.time.unwrap().to_string().contains("11:22:57"));
        assert_eq!(gns.lat.unwrap(), 38.73733516666667);
        assert_eq!(gns.lon.unwrap(), -9.140638);
        assert_eq!(gns.pos_mode, [PosMode::Autonomous, PosMode::NotValid]);
        assert_eq!(gns.num_sv.unwrap(), 3);
        assert_eq!(gns.hdop.unwrap(), 10.5);
        assert!(gns.alt.is_none());
        assert!(gns.sep.is_none());
        assert!(gns.diff_age.is_none());
        assert!(gns.diff_station.is_none());

        Ok(())
    }
    #[test]
    fn test_gns_parsing2() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GNGNS,181604.00,,,,,NN,00,99.99,,,,*59";
        let mut ctx = StrParserContext::new();
        let gns = Gns::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{gns:?}");
        assert_eq!(gns.talker, Talker::GN);
        assert!(gns.time.unwrap().to_string().contains("18:16:04"));
        assert!(gns.lat.is_none());
        assert!(gns.lon.is_none());
        assert_eq!(gns.pos_mode, [PosMode::NotValid, PosMode::NotValid]);
        assert_eq!(gns.num_sv.unwrap(), 0);
        assert_approx_eq!(f64, gns.hdop.unwrap(), 99.99);
        assert!(gns.alt.is_none());
        assert!(gns.sep.is_none());
        assert!(gns.diff_age.is_none());
        assert!(gns.diff_station.is_none());
        assert!(gns.nav_status.is_none());
        Ok(())
    }
}
