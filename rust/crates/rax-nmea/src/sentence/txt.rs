extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use derive_getters::Getters;
use rax::text::{IParseStr, StrParser};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::rules::{UNTIL_COMMA_DISCARD, UNTIL_NEW_LINE_DISCARD, UNTIL_STAR_DISCARD};

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TxtType {
    #[strum(serialize = "Error", serialize = "00")]
    Error = 0,
    #[strum(serialize = "Warn", serialize = "01")]
    Warn = 1,
    #[strum(serialize = "Info", serialize = "02")]
    Info = 2,
    #[strum(serialize = "User", serialize = "07")]
    User = 7,
}

///Text transmission
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Txt {
    /// Text information
    message: Vec<(TxtType, String)>,
}

impl IParseStr<RaxNmeaError, true> for Txt {
    fn parse_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut parser = StrParser::new(input);
        clerk::trace!("Txt::new: sentence='{}'", parser.full_str());
        let mut infos = Vec::new();
        for _ in 0..parser.full_str().lines().count() {
            let txt_type = parser
                .skip(&UNTIL_COMMA_DISCARD)?
                .skip(&UNTIL_COMMA_DISCARD)?
                .skip(&UNTIL_COMMA_DISCARD)?
                .take(&UNTIL_COMMA_DISCARD)?
                .parse::<TxtType>()?;
            clerk::debug!("txt_type: {:?}", txt_type);
            let info = parser.take(&UNTIL_STAR_DISCARD)?.to_string();
            clerk::debug!("info: {:?}", info);
            infos.push((txt_type, info));
            let _ = parser.skip(&UNTIL_NEW_LINE_DISCARD);
        }

        Ok(Self { message: infos })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_sentence;

    test_sentence!(
        test_txt,1,
        Txt,
        "$GPTXT,03,01,02,MA=CASIC*25\r\n$GPTXT,03,02,02,IC=ATGB03+ATGR201*70\r\n$GPTXT,03,03,02,SW=URANUS2,V2.2.1.0*1D"
    );
}
