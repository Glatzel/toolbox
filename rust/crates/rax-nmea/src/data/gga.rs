use core::fmt::{self, Display};
use core::str::FromStr;
extern crate alloc;
use alloc::string::{String, ToString};
use core::fmt::Write;

use derive_getters::Getters;
use rax::string::{IDecode, ParseOptExt, Parser};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
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
    type Err = RaxNmeaError;
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
            other => Err(RaxNmeaError::UnknownGgaQualityIndicator(other.to_string())),
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
/// Global Positioning System Fix Data.
///
/// This is one of the sentences commonly emitted by GPS units. Time, Position
/// and fix related data for a GPS receiver.
///
/// # References
///
/// * <https://gpsd.gitlab.io/gpsd/NMEA.html#_gga_global_positioning_system_fix_data>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Gga {
    time: Option<chrono::NaiveTime>,

    /// Latitude, dd is degrees, mm.mm is minutes
    lat: Option<f64>,

    /// Longitude, dd is degrees, mm.mm is minutes
    lon: Option<f64>,

    /// Quality indicator for position fix
    quality: Option<GgaQualityIndicator>,

    /// Number of satellites used (range: 0-12)
    num_sv: Option<u8>,

    /// Horizontal Dilution of precision (meters)
    hdop: Option<f64>,

    /// Antenna Altitude above/below mean-sea-level (geoid) (in meters)
    alt: Option<f64>,

    /// Geoidal separation, the difference between the WGS-84 earth ellipsoid
    /// and mean-sea-level (geoid), `-` means mean-sea-level below ellipsoid (in
    /// meters)
    sep: Option<f64>,

    /// Age of differential GPS data, time in seconds since last SC104 type 1 or
    /// 9 update, null field when DGPS is not used
    diff_age: Option<f64>,

    /// Differential reference station ID, 0000-1023
    diff_station: Option<u16>,
}
impl IDecode<RaxNmeaError> for Gga {
    fn decode(parser: &mut Parser) -> Result<Self, RaxNmeaError> {
        clerk::trace!("Gga::new: sentence='{}'", parser.full_str());

        parser.global(&NmeaValidate)?;

        clerk::debug!("Parsing utc_time...");
        let time = parser.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime);
        clerk::debug!("utc_time: {:?}", time);

        clerk::debug!("Parsing lat...");
        let lat = parser.take(&NmeaCoord);
        clerk::debug!("lat: {:?}", lat);

        clerk::debug!("Parsing lon...");
        let lon = parser.take(&NmeaCoord);
        clerk::debug!("lon: {:?}", lon);

        clerk::debug!("Parsing quality...");
        let quality = parser.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("quality: {:?}", quality);

        clerk::debug!("Parsing satellite_count...");
        let num_sv = parser.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("satellite_count: {:?}", num_sv);

        clerk::debug!("Parsing hdop...");
        let hdop = parser.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("hdop: {:?}", hdop);

        clerk::debug!("Parsing altitude...");
        let alt = parser.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("altitude: {:?}", alt);

        clerk::debug!("Skipping char_comma and char_m for altitude units...");
        parser.skip_strict(&UNTIL_COMMA_DISCARD)?;

        clerk::debug!("Parsing geoid_separation...");
        let sep = parser.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("geoid_separation: {:?}", sep);

        clerk::debug!("Skipping char_m for geoid units...");
        parser.skip_strict(&UNTIL_COMMA_DISCARD)?;

        clerk::debug!("Parsing age_of_differential_gps_data...");
        let diff_age = parser.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!("age_of_differential_gps_data: {:?}", diff_age);

        clerk::debug!("Parsing differential_reference_station_id...");
        let diff_station = parser.take(&UNTIL_STAR_DISCARD).parse_opt();
        clerk::debug!("differential_reference_station_id: {:?}", diff_station);

        Ok(Gga {
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
            let mut s = String::new();
            write!(s, "{alt} M")?;
            ds.field("alt", &s);
        }
        if let Some(sep) = self.sep {
            let mut s = String::new();
            write!(s, "{sep} M")?;
            ds.field("sep", &s);
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
    use std::println;
    use std::string::ToString;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;

    #[test]
    fn test_new_gga1() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGGA,110256,5505.676996,N,03856.028884,E,2,08,0.7,2135.0,M,14.0,M,,*7D";
        let mut ctx = Parser::new();
        let gga = Gga::decode(ctx.init(s.to_string()))?;
        println!("{gga:?}");
        insta::assert_debug_snapshot!(gga);
        Ok(())
    }
}
