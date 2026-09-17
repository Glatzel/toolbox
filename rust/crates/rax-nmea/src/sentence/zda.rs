use derive_getters::Getters;
use jiff::civil::Time;
use rax::text::{IParseStr, StrParser};

use crate::RaxNmeaError;
use crate::rules::{NmeaTime, UNTIL_COMMA_DISCARD, UNTIL_STAR_DISCARD};
use crate::utils::ParseOptionPrimitive;
///Time and date
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Zda {
    /// UTC time of the position fix
    time: Option<Time>,

    /// Day of the month
    day: Option<u8>,

    /// Month of the year
    month: Option<u8>,

    /// Year
    year: Option<u16>,

    /// Local zone description
    ltzh: Option<i8>,

    /// Local zone minutes description
    ltzn: Option<u8>,
}

impl IParseStr<RaxNmeaError, true> for Zda {
    fn parse_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut parser = StrParser::new(input);
        let time = parser.skip(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime)?;
        let day = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let month = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let year = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let ltzh = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let ltzn = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Self {
            time,
            day,
            month,
            year,
            ltzh,
            ltzn,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_sentence;
    test_sentence!(test_zda1, 1, Zda, "$GPZDA,160012.71,11,03,2004,-1,00*7D");
}
