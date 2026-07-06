extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use derive_getters::Getters;
use rax::string::{Decoder, IDecode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::rules::*;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumString, strum::AsRefStr)]
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

impl IDecode<RaxNmeaError> for Txt {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
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
    use clerk::{LevelFilter, init_log_with_level};
    extern crate std;
    use std::println;

    use super::*;
    #[test]
    fn test_new_txt() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPTXT,03,01,02,MA=CASIC*25\r\n$GPTXT,03,02,02,IC=ATGB03+ATGR201*70\r\n$GPTXT,03,03,02,SW=URANUS2,V2.2.1.0*1D";
        let mut decoder = Decoder::new(s);
        let txt = Txt::decode(&mut decoder)?;
        println!("{txt:?}");
        insta::assert_json_snapshot!(txt);
        Ok(())
    }
}
