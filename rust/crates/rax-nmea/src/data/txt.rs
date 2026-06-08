extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use derive_getters::Getters;
use rax::string::{IDecode, DecodeOptExt, Decoder};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::rules::*;
#[derive(Debug, Clone, Copy, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TxtType {
    #[strum(serialize = "Error", serialize = "0")]
    Error = 0,
    #[strum(serialize = "Warn", serialize = "1")]
    Warn = 1,
    #[strum(serialize = "Info", serialize = "2")]
    Info = 2,
    #[strum(serialize = "User", serialize = "7")]
    User = 7,
}
impl TryFrom<u8> for TxtType {
    type Error = RaxNmeaError;
    fn try_from(s: u8) -> Result<Self, RaxNmeaError> {
        match s {
            0 => Ok(Self::Error),
            1 => Ok(Self::Warn),
            2 => Ok(Self::Info),
            7 => Ok(Self::User),
            other => Err(RaxNmeaError::UnknownTxtType(other)),
        }
    }
}

///Text transmission
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Txt {
    /// Text information
    message: Vec<(Option<TxtType>, Option<String>)>,
}

impl IDecode<RaxNmeaError> for Txt {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        clerk::trace!("Txt::new: sentence='{}'", parser.full_str());
        let mut infos = Vec::new();
        for _ in 0..parser.full_str().lines().count() {
            let txt_type = parser
                .skip_strict(&UNTIL_COMMA_DISCARD)?
                .skip_strict(&UNTIL_COMMA_DISCARD)?
                .skip_strict(&UNTIL_COMMA_DISCARD)?
                .take(&UNTIL_COMMA_DISCARD)
                .decode_opt::<u8>()
                .map(TxtType::try_from)
                .and_then(Result::ok);
            let info = parser.take(&UNTIL_STAR_DISCARD).map(|f| f.to_string());
            infos.push((txt_type, info));
            parser.skip(&UNTIL_NEW_LINE_DISCARD);
        }

        Ok(Self { message: infos })
    }
}

#[cfg(test)]
mod test {
    use clerk::{LevelFilter, init_log_with_level};
    extern crate std;
    use std::println;
    use std::string::ToString;

    use super::*;
    #[test]
    fn test_new_txt() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPTXT,03,01,02,MA=CASIC*25\r\n$GPTXT,03,02,02,IC=ATGB03+ATGR201*70\r\n$GPTXT,03,03,02,SW=URANUS2,V2.2.1.0*1D";
        let mut parser = Decoder::new();
        let txt = Txt::decode(parser.init(s.to_string()))?;
        println!("{txt:?}");
        insta::assert_json_snapshot!(txt);
        Ok(())
    }
}
