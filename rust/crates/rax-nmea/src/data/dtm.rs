use core::fmt;
use core::str::FromStr;
extern crate alloc;
use alloc::string::{String, ToString};

use derive_getters::Getters;
use rax::string::{IDecode, ParseOptExt, Parser};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::rules::*;
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DtmDatum {
    WGS84,
    PZ90,
    UserDefined,
}
impl FromStr for DtmDatum {
    type Err = RaxNmeaError;

    fn from_str(s: &str) -> Result<Self, RaxNmeaError> {
        match s {
            "W84" => Ok(Self::WGS84),
            "P90" => Ok(Self::PZ90),
            "999" => Ok(Self::UserDefined),
            other => Err(RaxNmeaError::UnknownDtmDatum(other.to_string())),
        }
    }
}
/// Datum reference
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Dtm {
    /// Local datum
    datum: Option<DtmDatum>,
    
    /// sub datum
    sub_datum: Option<String>,
    
    /// Offset in Latitude
    lat: Option<f64>,
    
    /// Offset in Longitude
    lon: Option<f64>,
    
    /// Offset in altitude
    alt: Option<f64>,
}
impl IDecode<RaxNmeaError> for Dtm {
    fn decode(parser: &mut Parser) -> Result<Self, RaxNmeaError> {
        parser.global(&NmeaValidate)?;
        let datum = parser
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)
            .parse_opt();
        let sub_datum = parser.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let lat = parser.take(&NmeaDegree);
        let lon = parser.take(&NmeaDegree);
        let alt = parser.take(&UNTIL_COMMA_DISCARD).parse_opt();

        Ok(Dtm {
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
    extern crate std;
    use std::println;
    use std::string::ToString;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[test]
    fn test_new_dtm() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPDTM,999,,0.08,N,0.07,E,-47.7,W84*1B";
        let mut parser = Parser::new();
        let dtm = Dtm::decode(parser.init(s.to_string()))?;
        println!("{dtm:?}");
        insta::assert_debug_snapshot!(dtm);
        Ok(())
    }
}
