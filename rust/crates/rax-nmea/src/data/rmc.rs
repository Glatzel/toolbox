use std::fmt;

use chrono::NaiveDate;
use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{INmeaData, PosMode, Status, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

readonly_struct!(
    Rmc ,
    "Recommended minimum data",
    {talker: Talker},

    {
        time: Option<chrono::NaiveTime>,
        "UTC time of the position fix"
    },
    {
        status: Option<Status>,
        "Status"
    },
    {
        lat: Option<f64>,
        "Latitude"
    },
    {
        lon: Option<f64>,
        "Longitude"
    },
    {
        spd: Option<f64>,
        "Speed over ground"
    },
    {
        cog: Option<f64>,
        "Track made good"
    },
    {
        date: Option<NaiveDate>,
        "Date"
    },
    {
        mv: Option<f64>,
        "Magnetic variation"
    },
    {
        pos_mode: Option<PosMode>,
        "FAA mode"
    }
);

impl INmeaData for Rmc {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> mischief::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let time = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NMEA_TIME);
        let status = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let lat = ctx.take(&NMEA_COORD);
        let lon = ctx.take(&NMEA_COORD);
        let spd = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let cog = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let date = ctx.take(&NMEA_DATE);
        let mv = ctx.take(&NMEA_DEGREE);
        let pos_mode = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();
        Ok(Rmc {
            talker,
            time,
            status,
            lat,
            lon,
            spd,
            cog,
            date,
            mv,
            pos_mode,
        })
    }
}

impl fmt::Debug for Rmc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("RMC");
        ds.field("talker", &self.talker);

        if let Some(ref time) = self.time {
            ds.field("time", time);
        }
        if let Some(ref status) = self.status {
            ds.field("status", status);
        }
        if let Some(lat) = self.lat {
            ds.field("lat", &lat);
        }
        if let Some(lon) = self.lon {
            ds.field("lon", &lon);
        }
        if let Some(spd) = self.spd {
            ds.field("spd", &spd);
        }
        if let Some(cog) = self.cog {
            ds.field("cog", &cog);
        }
        if let Some(ref date) = self.date {
            ds.field("date", date);
        }
        if let Some(mv) = self.mv {
            ds.field("mv", &mv);
        }
        if let Some(ref pos_mode) = self.pos_mode {
            ds.field("pos_mode", pos_mode);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use clerk::{LogLevel, init_log_with_level};
    use float_cmp::assert_approx_eq;

    use super::*;
    #[test]
    fn test_new_rmc1() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPRMC,110125,A,5505.337580,N,03858.653666,E,148.8,84.6,310317,8.9,E,D*2E";
        let mut ctx = StrParserContext::new();
        let rmc = Rmc::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{rmc:?}");
        assert_eq!(rmc.talker, Talker::GN);
        assert!(rmc.time.unwrap().to_string().contains("11:01:25"));
        assert_eq!(rmc.status.unwrap(), Status::Valid);
        assert_approx_eq!(f64, rmc.lat.unwrap(), 55.088959666666675);
        assert_approx_eq!(f64, rmc.lon.unwrap(), 38.9775611);
        assert_approx_eq!(f64, rmc.spd.unwrap(), 148.8);
        assert_approx_eq!(f64, rmc.cog.unwrap(), 84.6);
        assert_eq!(rmc.date.unwrap().to_string(), "2017-03-31");
        assert_approx_eq!(f64, rmc.mv.unwrap(), 8.9);
        assert_eq!(rmc.pos_mode.unwrap(), PosMode::Differential);
        Ok(())
    }
    #[test]
    fn test_new_rmc2() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPRMC,,V,,,,,,,,,,N*53";
        let mut ctx = StrParserContext::new();
        let rmc = Rmc::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{rmc:?}");
        assert_eq!(rmc.talker, Talker::GN);
        assert!(rmc.time.is_none());
        assert_eq!(rmc.status, Some(Status::Invalid));
        assert!(rmc.lat.is_none());
        assert!(rmc.lon.is_none());
        assert!(rmc.spd.is_none());
        assert!(rmc.cog.is_none());
        assert!(rmc.date.is_none());
        assert!(rmc.mv.is_none());
        assert_eq!(rmc.pos_mode, Some(PosMode::NotValid));
        Ok(())
    }
}
