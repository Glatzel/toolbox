use derive_getters::Getters;
use rax::text::{IParseStr, StrParser};

use crate::RaxNmeaError;
use crate::rules::UNTIL_COMMA_DISCARD;
use crate::utils::ParseOptionPrimitive;

///Poll a standard message (Talker ID GL)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Vlw {
    /// Total cumulative water distance
    twd: Option<f64>,

    /// Water distance since reset
    wd: Option<f64>,

    /// Total cumulative ground distance
    tgd: Option<f64>,

    /// Ground distance since reset
    gd: Option<f64>,
}

impl IParseStr<RaxNmeaError, true> for Vlw {
    fn parse_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut parser = StrParser::new(input);
        let twd = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)?
            .parse_option()?;
        parser.skip(&UNTIL_COMMA_DISCARD)?;
        let wd = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        parser.skip(&UNTIL_COMMA_DISCARD)?;
        let tgd = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        parser.skip(&UNTIL_COMMA_DISCARD)?;
        let gd = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        Ok(Self { twd, wd, tgd, gd })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_sentence;

    test_sentence!(test_vlw1, 1, Vlw, "$GPVLW,,N,,N,15.8,N,1.2,N*65");
}
