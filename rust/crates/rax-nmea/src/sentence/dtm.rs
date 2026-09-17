extern crate alloc;

use alloc::string::String;

use derive_getters::Getters;
use rax::text::{IParseStr, StrParser};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::rules::{NmeaDegree, UNTIL_COMMA_DISCARD};
use crate::utils::ParseOptionPrimitive;
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DtmDatum {
    #[strum(serialize = "W84")]
    WGS84,
    #[strum(serialize = "P90")]
    PZ90,
    #[strum(serialize = "999")]
    UserDefined,
}

/// Datum reference
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
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
impl IParseStr<RaxNmeaError, true> for Dtm {
    fn parse_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut parser = StrParser::new(input);
        let datum = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)?
            .parse_option()?;
        let sub_datum = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let lat = parser.take(&NmeaDegree)?;
        let lon = parser.take(&NmeaDegree)?;
        let alt = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;

        Ok(Self {
            datum,
            sub_datum,
            lat,
            lon,
            alt,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_sentence;

    test_sentence!(test_dtm, 1, Dtm, "$GPDTM,999,,0.08,N,0.07,E,-47.7,W84*1B");
}
