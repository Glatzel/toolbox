use core::fmt::{self, Display};
use core::str::FromStr;

use rax::str_parser::{ParseOptExt, StrParserContext};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GgaQualityIndicator {
    Invalid = 0,
    GpsFix = 1,
    DifferentialGpsFix = 2,
    PpsFix = 3,
    RealTimeKinematic = 4,
    FloatRTK = 5,
    DeadReckoning = 6,
    ManualInputMode = 7,
    SimulationMode = 8,
}
impl FromStr for GgaQualityIndicator {
    type Err = mischief::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Invalid),
            "1" => Ok(Self::GpsFix),
            "2" => Ok(Self::DifferentialGpsFix),
            "3" => Ok(Self::PpsFix),
            "4" => Ok(Self::RealTimeKinematic),
            "5" => Ok(Self::FloatRTK),
            "6" => Ok(Self::DeadReckoning),
            "7" => Ok(Self::ManualInputMode),
            "8" => Ok(Self::SimulationMode),
            other => mischief::bail!("Unknown GgaQualityIndicator {}", other),
        }
    }
}
impl Display for GgaQualityIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GgaQualityIndicator::Invalid => "Invalid",
            GgaQualityIndicator::GpsFix => "Gps Fix",
            GgaQualityIndicator::DifferentialGpsFix => "Differential Gps Fix",
            GgaQualityIndicator::PpsFix => "Pps Fix",
            GgaQualityIndicator::RealTimeKinematic => "RealTimeKinematic",
            GgaQualityIndicator::FloatRTK => "Float RTK",
            GgaQualityIndicator::DeadReckoning => "Dead Reckoning",
            GgaQualityIndicator::ManualInputMode => "Manual Input Mode",
            GgaQualityIndicator::SimulationMode => "Simulation Mode",
        };
        write!(f, "{s}")
    }
}
readonly_struct!(
    Gga ,
    "Global Positioning System Fix Data."
    "This is one of the sentences commonly emitted by GPS units. Time, Position and fix related data for a GPS receiver."
    "# References"
    "* <https://gpsd.gitlab.io/gpsd/NMEA.html#_gga_global_positioning_system_fix_data>",

    {talker: Talker},

    {
        time: Option<chrono::NaiveTime>,
        "UTC of this position report, hh is hours, mm is minutes, ss.ss is seconds."
    },
    {
        lat: Option<f64>,
        "Latitude, dd is degrees, mm.mm is minutes"
    },
    {
        lon: Option<f64>,
        "Longitude, dd is degrees, mm.mm is minutes"
    },
    {
        quality: Option<GgaQualityIndicator>,
        " Quality indicator for position fix"
    },
    {
        num_sv: Option<u8>,
        "Number of satellites used (range: 0-12)"
    },
    {
        hdop: Option<f64>,
        "Horizontal Dilution of precision (meters)"
    },
    {
        alt: Option<f64>,
        "Antenna Altitude above/below mean-sea-level (geoid) (in meters)"
    },
    {
        sep: Option<f64>,
        "Geoidal separation, the difference between the WGS-84 earth ellipsoid and mean-sea-level (geoid), `-` means mean-sea-level below ellipsoid"
    },
    {
        diff_age: Option<f64>,
        "Age of differential GPS data, time in seconds since last SC104 type 1 or 9 update, null field when DGPS is not used"
    },
    {
        diff_station: Option<u16>,
        "Differential reference station ID, 0000-1023"
    }
);
impl INmeaData for Gga {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> mischief::Result<Self> {
        clerk::trace!("Gga::new: sentence='{}'", ctx.full_str());

        ctx.global(&NMEA_VALIDATE)?;

        clerk::debug!("Parsing utc_time...");
        let time = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NMEA_TIME);
        clerk::debug!("utc_time: {:?}", time);

        clerk::debug!("Parsing lat...");
        let lat = ctx.take(&NMEA_COORD);
        clerk::debug!("lat: {:?}", lat);

        clerk::debug!("Parsing lon...");
        let lon = ctx.take(&NMEA_COORD);
        clerk::debug!("lon: {:?}", lon);

        clerk::debug!("Parsing quality...");
        let quality = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("quality: {:?}", quality);

        clerk::debug!("Parsing satellite_count...");
        let num_sv = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("satellite_count: {:?}", num_sv);

        clerk::debug!("Parsing hdop...");
        let hdop = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("hdop: {:?}", hdop);

        clerk::debug!("Parsing altitude...");
        let alt = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("altitude: {:?}", alt);

        clerk::debug!("Skipping char_comma and char_m for altitude units...");
        ctx.skip_strict(&UNTIL_COMMA_DISCARD)?;

        clerk::debug!("Parsing geoid_separation...");
        let sep = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("geoid_separation: {:?}", sep);

        clerk::debug!("Skipping char_m for geoid units...");
        ctx.skip_strict(&UNTIL_COMMA_DISCARD)?;

        clerk::debug!("Parsing age_of_differential_gps_data...");
        let diff_age = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("age_of_differential_gps_data: {:?}", diff_age);

        clerk::debug!("Parsing differential_reference_station_id...");
        let diff_station = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();
        clerk::debug!("differential_reference_station_id: {:?}", diff_station);

        Ok(Gga {
            talker,
            time,
            lat,
            lon,
            quality,
            num_sv,
            hdop,
            alt,
            sep,
            diff_age,
            diff_station,
        })
    }
}

impl fmt::Debug for Gga {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GGA");
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
        if let Some(ref quality) = self.quality {
            ds.field("quality", quality);
        }
        if let Some(num_sv) = self.num_sv {
            ds.field("num_sv", &num_sv);
        }
        if let Some(hdop) = self.hdop {
            ds.field("hdop", &hdop);
        }
        if let Some(alt) = self.alt {
            ds.field("alt", &format!("{alt} M"));
        }
        if let Some(sep) = self.sep {
            ds.field("sep", &format!("{sep} M"));
        }
        if let Some(diff_age) = self.diff_age {
            ds.field("diff_age", &diff_age);
        }
        if let Some(diff_station) = self.diff_station {
            ds.field("diff_station", &diff_station);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use clerk::{LogLevel, init_log_with_level};
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn test_new_gga1() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGGA,110256,5505.676996,N,03856.028884,E,2,08,0.7,2135.0,M,14.0,M,,*7D";
        let mut ctx = StrParserContext::new();
        let gga = Gga::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{gga:?}");
        assert_eq!(gga.talker, Talker::GN);
        assert!(gga.time.unwrap().to_string().contains("11:02:56"));
        assert_approx_eq!(f64, gga.lat.unwrap(), 55.0946166);
        assert_approx_eq!(f64, gga.lon.unwrap(), 38.93381473333333);
        assert_eq!(
            gga.quality.unwrap(),
            GgaQualityIndicator::DifferentialGpsFix
        );
        assert_eq!(gga.num_sv.unwrap(), 8);
        assert_approx_eq!(f64, gga.hdop.unwrap(), 0.7);
        assert_approx_eq!(f64, gga.alt.unwrap(), 2135.0);
        assert_approx_eq!(f64, gga.sep.unwrap(), 14.0);
        assert!(gga.diff_age.is_none());
        assert!(gga.diff_station.is_none());
        Ok(())
    }
}
