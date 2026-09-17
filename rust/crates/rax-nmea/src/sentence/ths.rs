use derive_getters::Getters;
use rax::text::{IParseStr, StrParser};

use crate::RaxNmeaError;
use crate::common::FaaMode;
use crate::rules::{UNTIL_COMMA_DISCARD, UNTIL_STAR_DISCARD};
use crate::utils::ParseOptionPrimitive;

#[doc = "Poll a standard message (Talker ID GL)"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Ths {
    /// Heading of vehicle (true)
    headt: Option<f64>,

    /// Mode indicator
    mi: Option<FaaMode>,
}
impl IParseStr<RaxNmeaError, true> for Ths {
    fn parse_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut parser = StrParser::new(input);
        let headt = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)?
            .parse_option()?;
        let mi = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Self { headt, mi })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_sentence;
    test_sentence!(test_ths, 1, Ths, "$GPTHS,77.52,E*34");
}
