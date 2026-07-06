use derive_getters::Getters;
use rax::string::{Decoder, IDecode};

use crate::RaxNmeaError;
use crate::common::SystemId;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;

/// GNSS satellite fault detection
///
/// # References
///
/// * <https://gpsd.gitlab.io/gpsd/NMEA.html#_gbs_gps_satellite_fault_detection>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gbs {
    /// UTC time to which this RAIM sentence belongs. See section UTC
    /// representation in the integration manual for details.
    time: Option<chrono::NaiveTime>,

    /// Expected 1-sigma error in latitude (meters)
    err_lat: Option<f64>,

    /// Expected 1-sigma error in longitude (meters)
    err_lon: Option<f64>,

    /// Expected 1-sigma error in altitude (meters)
    err_alt: Option<f64>,

    /// Satellite ID of most likely failed satellite.
    svid: Option<u16>,

    /// Probability of missed detection.
    prob: Option<f64>,

    /// Estimated bias of most likely failed satellite (a priori residual)
    bias: Option<f64>,

    /// Standard deviation of bias estimate
    std_dev: Option<f64>,

    /// System ID
    system_id: Option<SystemId>,

    /// Signal ID
    signal_id: Option<u16>,
}

impl IDecode<RaxNmeaError> for Gbs {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let time = parser.skip(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime)?;
        let err_lat = parser.take(&UNTIL_COMMA_KEEP_RIGHT)?.parse_option()?;
        let _ = parser.skip(&UNTIL_M_DISCARD);
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);

        let err_lon = parser.take(&UNTIL_COMMA_KEEP_RIGHT)?.parse_option()?;
        let _ = parser.skip(&UNTIL_M_DISCARD);
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);

        let err_alt = parser.take(&UNTIL_COMMA_KEEP_RIGHT)?.parse_option()?;
        let _ = parser.skip(&UNTIL_M_DISCARD);
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);

        let svid = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);

        let prob = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);

        let bias = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);

        let std_dev = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);

        let system_id = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);

        let signal_id = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Gbs {
            time,
            err_lat,
            err_lon,
            err_alt,
            svid,
            prob,
            bias,
            std_dev,
            system_id,
            signal_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use clerk::{LevelFilter, init_log_with_level};
    extern crate std;
    use std::println;

    use super::*;
    #[test]
    fn test_gbs() {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGBS,125027,23.43,M,13.91,M,34.01,M*07";
        let mut decoder = Decoder::new(s);
        let gbs = Gbs::decode(&mut decoder).unwrap();
        println!("{gbs:?}");
        insta::assert_json_snapshot!(gbs);
    }
    #[test]
    fn test_gbs_4_1() {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGBS,235458.00,1.4,1.3,3.1,03,,-21.4,3.8,1,0*5B";
        let mut decoder = Decoder::new(s);
        let gbs = Gbs::decode(&mut decoder).unwrap();
        println!("{gbs:?}");
        insta::assert_json_snapshot!(gbs);
    }
}
