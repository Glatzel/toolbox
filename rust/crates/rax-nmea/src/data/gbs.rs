use std::fmt::Debug;

use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{INmeaData, SystemId, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

readonly_struct!(
    Gbs,
    "GNSS satellite fault detection"
    "# References"
    "* <https://gpsd.gitlab.io/gpsd/NMEA.html#_gbs_gps_satellite_fault_detection>"
    ,
   {talker: Talker},

   {
       time:  Option<chrono::NaiveTime>,
       "UTC time to which this RAIM sentence belongs. See section UTC representation in the integration manual for details."
   },
   {
       err_lat:Option<f64>,
       "Expected 1-sigma error in latitude (meters)"
   },
   {
       err_lon:Option<f64>,
       "Expected 1-sigma error in longitude (meters)"
   },
   {
       err_alt:Option<f64>,
       "Expected 1-sigma error in altitude (meters)"
   },
   {
       svid:Option<u16>,
       "Satellite ID of most likely failed satellite."
   },
   {
       prob:Option<f64>,
       "Probability of missed detection."
   },
   {
       bias:Option<f64>,
       " Estimated bias of most likely failed satellite (a priori residual)"
   },
   {
       std_dev:Option<f64>,
       "Standard deviation of bias estimate"
   },
   {
    system_id:Option<SystemId>
   },
   {
    signal_id:Option<u16>
   }
);

impl INmeaData for Gbs {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> mischief::Result<Self> {
        let time = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NMEA_TIME);
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    use clerk::{LogLevel, init_log_with_level};
    extern crate std;
    use super::*;

    #[test]
    fn test_gbs() {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGBS,125027,23.43,M,13.91,M,34.01,M*07";
        let mut ctx = StrParserContext::new();
        let gbs = Gbs::new(ctx.init(s.to_string()), Talker::GP).unwrap();
        println!("{gbs:?}");
        assert_eq!(gbs.talker(), &Talker::GP);
        assert!(gbs.time().unwrap().to_string().contains("12:50:27"));
        assert_eq!(gbs.err_lat().unwrap(), 23.43);
        assert_eq!(gbs.err_alt().unwrap(), 13.91);
        assert_eq!(gbs.prob().unwrap(), 34.01);
    }
    #[test]
    fn test_gbs_4_1() {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGBS,235458.00,1.4,1.3,3.1,03,,-21.4,3.8,1,0*5B";
        let mut ctx = StrParserContext::new();
        let gbs = Gbs::new(ctx.init(s.to_string()), Talker::GP).unwrap();
        println!("{gbs:?}");
        assert_eq!(gbs.talker(), &Talker::GP);
        assert!(gbs.time().unwrap().to_string().contains("23:54:58"));
        assert_eq!(gbs.err_lat().unwrap(), 1.4);
        assert_eq!(gbs.err_lon().unwrap(), 1.3);
        assert_eq!(gbs.err_alt().unwrap(), 3.1);
        assert_eq!(gbs.svid().unwrap(), 3);
        assert_eq!(gbs.bias().unwrap(), -21.4);
        assert_eq!(gbs.std_dev().unwrap(), 3.8);
        assert_eq!(gbs.system_id().unwrap(), SystemId::GPS);
        assert_eq!(gbs.signal_id().unwrap(), 0);
    }
}
