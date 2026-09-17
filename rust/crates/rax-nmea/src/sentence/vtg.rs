use derive_getters::Getters;
use rax::text::{IParseStr, StrParser};

use crate::RaxNmeaError;
use crate::common::FaaMode;
use crate::rules::{UNTIL_COMMA_DISCARD, UNTIL_STAR_DISCARD};
use crate::utils::ParseOptionPrimitive;
///Course over ground and ground speed
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Vtg {
    /// Course over ground (true)
    cogt: Option<f64>,

    /// Course over ground (magnetic)
    cogm: Option<f64>,

    /// Speed over ground (knots)
    sogn: Option<f64>,

    /// Speed over ground (kph)
    sogk: Option<f64>,

    /// Mode
    pos_mode: Option<FaaMode>,
}

impl IParseStr<RaxNmeaError, true> for Vtg {
    fn parse_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut parser = StrParser::new(input);
        let cogt = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)?
            .parse_option()?;
        parser.skip(&UNTIL_COMMA_DISCARD)?;

        let cogm = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        parser.skip(&UNTIL_COMMA_DISCARD)?;

        let sogn = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        parser.skip(&UNTIL_COMMA_DISCARD)?;

        let sogk = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        parser.skip(&UNTIL_COMMA_DISCARD)?;

        let pos_mode = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Self {
            cogt,
            cogm,
            sogn,
            sogk,
            pos_mode,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_sentence;

    test_sentence!(
        test_vtg1,
        1,
        Vtg,
        "$GPVTG,83.7,T,83.7,M,146.3,N,271.0,K,D*22"
    );
}
