use std::fmt;
use std::str::FromStr;

use rax::str_parser::{ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::data::{INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DtmDatum {
    WGS84,
    PZ90,
    UserDefined,
}
impl FromStr for DtmDatum {
    type Err = miette::Report;

    fn from_str(s: &str) -> miette::Result<Self> {
        let result = match s {
            "W84" => Self::WGS84,
            "P90" => Self::PZ90,
            "999" => Self::UserDefined,
            other => miette::bail!("Unknown DtmDatum: {}", other),
        };
        Ok(result)
    }
}
readonly_struct!(
    Dtm ,
    "Datum reference",
    {talker: Talker},

    {
        datum: Option<DtmDatum>,
        "Local datum"
    },
    {
        sub_datum: Option<String>,
        "sub datum"
    },
    {
        lat:Option<f64>,
        "Offset in Latitude"
    },
    {
        lon:Option<f64>,
        "Offset in Longitude"
    },
    {
        alt:Option<f64>,
        "Offset in altitude"
    }
);
impl INmeaData for Dtm {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;
        let datum = ctx
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)
            .parse_opt();
        let sub_datum = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let lat = ctx.take(&NMEA_DEGREE);
        let lon = ctx.take(&NMEA_DEGREE);
        let alt = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();

        Ok(Dtm {
            talker,
            datum,
            sub_datum,
            lat,
            lon,
            alt,
        })
    }
}

impl fmt::Debug for Dtm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("DHV");
        ds.field("talker", &self.talker);

        if let Some(ref datum) = self.datum {
            ds.field("datum", datum);
        }
        if let Some(sub_datum) = &self.sub_datum {
            ds.field("sub_datum", &sub_datum);
        }
        if let Some(lat) = self.lat {
            ds.field("lat", &lat);
        }
        if let Some(lon) = self.lon {
            ds.field("lon", &lon);
        }
        if let Some(alt) = self.alt {
            ds.field("alt", &alt);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {

    use clerk::{LogLevel, init_log_with_level};

    use super::*;
    #[test]
    fn test_new_dtm() -> miette::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPDTM,999,,0.08,N,0.07,E,-47.7,W84*1B";
        let mut ctx = StrParserContext::new();
        let dhv = Dtm::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{dhv:?}");
        assert_eq!(dhv.talker, Talker::GP);
        assert_eq!(dhv.datum.unwrap(), DtmDatum::UserDefined);
        assert_eq!(dhv.lat.unwrap(), 0.08);
        assert_eq!(dhv.lon.unwrap(), 0.07);
        assert_eq!(dhv.alt.unwrap(), -47.7);
        Ok(())
    }
}
