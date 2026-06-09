extern crate alloc;

use derive_getters::Getters;
use rax::string::{ Decoder, IDecode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::rules::*;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GgaQualityIndicator {
    #[strum(serialize = "Invalid", serialize = "0")]
    Invalid = 0,
    #[strum(serialize = "Gps Fix", serialize = "1")]
    GpsFix = 1,
    #[strum(serialize = "Differential Gps Fix", serialize = "2")]
    DifferentialGpsFix = 2,
    #[strum(serialize = "Pps Fix", serialize = "3")]
    PpsFix = 3,
    #[strum(serialize = "Real Time Kinematic", serialize = "4")]
    RealTimeKinematic = 4,
    #[strum(serialize = "Float RTK", serialize = "5")]
    FloatRTK = 5,
    #[strum(serialize = "Dead Reckoning", serialize = "6")]
    DeadReckoning = 6,
    #[strum(serialize = "Manual Input Mode", serialize = "7")]
    ManualInputMode = 7,
    #[strum(serialize = "Simulation Mode", serialize = "8")]
    SimulationMode = 8,
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
#[derive(Debug, Clone, Getters)]
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
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        clerk::trace!("Gga::new: sentence='{}'", parser.full_str());

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
        let quality = parser.take(&UNTIL_COMMA_DISCARD).and_then(|s| s.parse().ok());
        clerk::debug!("quality: {:?}", quality);

        clerk::debug!("Parsing satellite_count...");
        let num_sv = parser.take(&UNTIL_COMMA_DISCARD).and_then(|s| s.parse().ok());
        clerk::debug!("satellite_count: {:?}", num_sv);

        clerk::debug!("Parsing hdop...");
        let hdop = parser.take(&UNTIL_COMMA_DISCARD).and_then(|s| s.parse().ok());
        clerk::debug!("hdop: {:?}", hdop);

        clerk::debug!("Parsing altitude...");
        let alt = parser.take(&UNTIL_COMMA_DISCARD).and_then(|s| s.parse().ok());
        clerk::debug!("altitude: {:?}", alt);

        clerk::debug!("Skipping char_comma and char_m for altitude units...");
        parser.skip_strict(&UNTIL_COMMA_DISCARD)?;

        clerk::debug!("Parsing geoid_separation...");
        let sep = parser.take(&UNTIL_COMMA_DISCARD).and_then(|s| s.parse().ok());
        clerk::debug!("geoid_separation: {:?}", sep);

        clerk::debug!("Skipping char_m for geoid units...");
        parser.skip_strict(&UNTIL_COMMA_DISCARD)?;

        clerk::debug!("Parsing age_of_differential_gps_data...");
        let diff_age = parser.take(&UNTIL_COMMA_DISCARD).and_then(|s| s.parse().ok());
        clerk::debug!("age_of_differential_gps_data: {:?}", diff_age);

        clerk::debug!("Parsing differential_reference_station_id...");
        let diff_station = parser.take(&UNTIL_STAR_DISCARD).and_then(|s| s.parse().ok());
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
        let mut ctx = Decoder::new();
        let gga = Gga::decode(ctx.init(s.to_string()))?;
        println!("{gga:?}");
        insta::assert_json_snapshot!(gga);
        Ok(())
    }
}
