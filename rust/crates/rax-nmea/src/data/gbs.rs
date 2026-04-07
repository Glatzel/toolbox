use core::fmt::Debug;

use derive_getters::Getters;
use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::RaxNmeaError;
use crate::data::{INmeaData, SystemId, Talker};
use crate::rules::*;

/// GNSS satellite fault detection
///
/// # References
///
/// * <https://gpsd.gitlab.io/gpsd/NMEA.html#_gbs_gps_satellite_fault_detection>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Gbs {
    talker: Talker,
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
    system_id: Option<SystemId>,
    signal_id: Option<u16>,
}

impl INmeaData for Gbs {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> Result<Self, RaxNmeaError> {
        let time = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime);
        let err_lat = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let err_lon = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let err_alt = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let svid = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let prob = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let bias = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let std_dev = ctx.take(&UNTIL_COMMA_OR_STAR_DISCARD).parse_opt();
        let system_id = ctx.take(&UNTIL_COMMA_OR_STAR_DISCARD).parse_opt();
        let signal_id = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();

        Ok(Gbs {
            talker,
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
impl Debug for Gbs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("GBS");
        ds.field("talker", &self.talker);

        if let Some(ref time) = self.time {
            ds.field("time", time);
        }
        if let Some(err_lat) = self.err_lat {
            ds.field("err_lat", &err_lat);
        }
        if let Some(err_lon) = self.err_lon {
            ds.field("err_lon", &err_lon);
        }
        if let Some(err_alt) = self.err_alt {
            ds.field("err_alt", &err_alt);
        }
        if let Some(svid) = self.svid {
            ds.field("svid", &svid);
        }
        if let Some(prob) = self.prob {
            ds.field("prob", &prob);
        }
        if let Some(bias) = self.bias {
            ds.field("bias", &bias);
        }
        if let Some(std_dev) = self.std_dev {
            ds.field("std_dev", &std_dev);
        }
        if let Some(system_id) = self.system_id {
            ds.field("system_id", &system_id);
        }
        if let Some(signal_id) = self.signal_id {
            ds.field("signal_id", &signal_id);
        }

        ds.finish()
    }
}
#[cfg(test)]
mod tests {
    use clerk::{Level, init_log_with_level};
    extern crate std;
    use std::println;
    use std::string::ToString;

    use super::*;
    #[test]
    fn test_gbs() {
        init_log_with_level(Level::TRACE);
        let s = "$GPGBS,125027,23.43,M,13.91,M,34.01,M*07";
        let mut ctx = StrParserContext::new();
        let gbs = Gbs::new(ctx.init(s.to_string()), Talker::GP).unwrap();
        println!("{gbs:?}");
        insta::assert_debug_snapshot!(gbs);
    }
    #[test]
    fn test_gbs_4_1() {
        init_log_with_level(Level::TRACE);
        let s = "$GPGBS,235458.00,1.4,1.3,3.1,03,,-21.4,3.8,1,0*5B";
        let mut ctx = StrParserContext::new();
        let gbs = Gbs::new(ctx.init(s.to_string()), Talker::GP).unwrap();
        println!("{gbs:?}");
        insta::assert_debug_snapshot!(gbs);
    }
}
