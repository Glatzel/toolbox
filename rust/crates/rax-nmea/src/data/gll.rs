use core::fmt;

use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::RaxNmeaError;
use crate::data::{INmeaData, PosMode, Status, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;
readonly_struct!(
    Gll ,
    "Latitude and longitude, with time of position fix and status",
    {talker: Talker},

    {
        lat: Option<f64>,
        "Latitude, ddmm.mmmm, where dd is degrees and mm.mmmm is minutes. Positive values indicate North, negative values indicate South."
    },
    {
        lon: Option<f64>,
        "Longitude, dddmm.mmmm, where ddd is degrees and mm.mmmm is minutes. Positive values indicate East, negative values indicate West."
    },
    {
        time: Option<chrono::NaiveTime>,
        "UTC time of the position fix"
    },
    {
        status: Option<Status>,
        "Status of the data"
    },
    {
        pos_mode: Option<PosMode>,
        "FAA mode"
    }
);
impl INmeaData for Gll {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> Result<Self, RaxNmeaError> {
        clerk::trace!("Gga::new: sentence='{}'", ctx.full_str());

        ctx.global(&NMEA_VALIDATE)?;

        clerk::debug!("Parsing lat...");
        let lat = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NMEA_COORD);
        clerk::debug!("lat: {:?}", lat);

        clerk::debug!("Parsing lon...");
        let lon = ctx.take(&NMEA_COORD);
        clerk::debug!("lon: {:?}", lon);

        clerk::debug!("Parsing utc_time...");
        let time = ctx.take(&NMEA_TIME);
        clerk::debug!("utc_time: {:?}", time);

        let status = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();

        let pos_mode = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();

        Ok(Gll {
            talker,
            lat,
            lon,
            time,
            status,
            pos_mode,
        })
    }
}

impl fmt::Debug for Gll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GLL");
        ds.field("talker", &self.talker);

        if let Some(lat) = self.lat {
            ds.field("lat", &lat);
        }
        if let Some(lon) = self.lon {
            ds.field("lon", &lon);
        }
        if let Some(ref time) = self.time {
            ds.field("time", time);
        }
        if let Some(ref status) = self.status {
            ds.field("status", status);
        }
        if let Some(ref pos_mode) = self.pos_mode {
            ds.field("pos_mode", pos_mode);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use std::println;
    use std::string::ToString;

    use clerk::{LogLevel, init_log_with_level};
    use float_cmp::assert_approx_eq;
    extern crate std;
    use super::*;
    #[test]
    fn test_new_ggl() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGLL,2959.9925,S,12000.0090,E,235316.000,A,A*4E";
        let mut ctx = StrParserContext::new();
        let gll = Gll::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{gll:?}");
        assert_eq!(gll.talker, Talker::GN);
        assert_approx_eq!(f64, gll.lat.unwrap(), -29.999874999999996);
        assert_approx_eq!(f64, gll.lon.unwrap(), 120.00015);
        assert!(gll.time.unwrap().to_string().contains("23:53:16"));
        assert_eq!(gll.status.unwrap(), Status::Valid);
        assert_eq!(gll.pos_mode.unwrap(), PosMode::Autonomous);
        Ok(())
    }
}
