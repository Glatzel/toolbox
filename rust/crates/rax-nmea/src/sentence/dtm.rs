extern crate alloc;

use alloc::string::String;

use derive_getters::Getters;
use rax::string::{Decoder, IDecode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;
#[derive(Debug, Clone, Copy, PartialEq, strum::EnumString, strum::AsRefStr)]
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
impl IDecode<RaxNmeaError> for Dtm {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let datum = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)?
            .parse_option()?;
        let sub_datum = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let lat = parser.take(&NmeaDegree)?;
        let lon = parser.take(&NmeaDegree)?;
        let alt = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;

        Ok(Dtm {
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
    extern crate std;
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[rstest::rstest]
    #[case("1", "$GPDTM,999,,0.08,N,0.07,E,-47.7,W84*1B")]
    fn test_dtm(#[case] index: &str, #[case] input: &str) -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = Decoder::new(input);
        let dtm = Dtm::decode(&mut decoder)?;
        println!("{dtm:?}");
        insta::assert_json_snapshot!(index, dtm);
        Ok(())
    }
}
