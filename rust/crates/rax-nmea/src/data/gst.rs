use core::fmt;

use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;
readonly_struct!(
    Gst ,
    "GNSS pseudorange error statistics",
    {talker: Talker},

    {
        time: Option<chrono::NaiveTime>,
        "UTC time of the position fix"
    },
    {
        rms: Option<f64>,
        "Root mean square"
    },
    {
        std_major: Option<f64>,
        "Standard deviation semi-major"
    },
    {
        std_minor: Option<f64>,
        "Standard deviation semi-minor"
    },
    {
        orient: Option<f64>,
        "Orientation"
    },
    {
        std_lat: Option<f64>,
        "Standard deviation semi-latitude"
    },
    {
        std_lon: Option<f64>,
        "Standard deviation semi-longitude"
    },
    {
        std_alt: Option<f64>,
        "Standard deviation semi-altitude"
    }
);
impl INmeaData for Gst {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> mischief::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let time = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NMEA_TIME);
        let rms = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let std_major = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let std_minor = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let orient = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let std_lat = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let std_lon = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let std_alt = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();

        Ok(Gst {
            talker,
            time,
            rms,
            std_major,
            std_minor,
            orient,
            std_lat,
            std_lon,
            std_alt,
        })
    }
}

impl fmt::Debug for Gst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GST");
        ds.field("talker", &self.talker);

        if let Some(ref time) = self.time {
            ds.field("time", time);
        }
        if let Some(rms) = self.rms {
            ds.field("rms", &rms);
        }
        if let Some(std_major) = self.std_major {
            ds.field("std_major", &std_major);
        }
        if let Some(std_minor) = self.std_minor {
            ds.field("std_minor", &std_minor);
        }
        if let Some(orient) = self.orient {
            ds.field("orient", &orient);
        }
        if let Some(std_lat) = self.std_lat {
            ds.field("std_lat", &std_lat);
        }
        if let Some(std_lon) = self.std_lon {
            ds.field("std_lon", &std_lon);
        }
        if let Some(std_alt) = self.std_alt {
            ds.field("std_alt", &std_alt);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;
    use std::string::ToString;

    use clerk::{LogLevel, init_log_with_level};

    use super::*;
    #[test]
    fn test_new_gst() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGST,182141.000,15.5,15.3,7.2,21.8,0.9,0.5,0.8*54";
        let mut ctx = StrParserContext::new();
        let vtg = Gst::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{vtg:?}");
        assert_eq!(vtg.talker, Talker::GN);
        assert!(vtg.time.unwrap().to_string().contains("18:21:41"));
        assert_eq!(vtg.rms.unwrap(), 15.5);
        assert_eq!(vtg.std_major.unwrap(), 15.3);
        assert_eq!(vtg.std_minor.unwrap(), 7.2);
        assert_eq!(vtg.orient.unwrap(), 21.8);
        assert_eq!(vtg.std_lat.unwrap(), 0.9);
        assert_eq!(vtg.std_lon.unwrap(), 0.5);
        assert_eq!(vtg.std_alt.unwrap(), 0.8);

        Ok(())
    }
}
