use core::fmt;
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;

use rax::str_parser::{IStrGlobalRule, ParseOptExt, StrParserContext};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TxtType {
    Error = 0,
    Warn = 1,
    Info = 2,
    User = 7,
}
impl TryFrom<u8> for TxtType {
    type Error = mischief::Report;
    fn try_from(s: u8) -> mischief::Result<Self> {
        match s {
            0 => Ok(Self::Error),
            1 => Ok(Self::Warn),
            2 => Ok(Self::Info),
            7 => Ok(Self::User),
            _ => mischief::bail!("Unknown txt type: {}", s),
        }
    }
}
impl core::fmt::Display for TxtType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            TxtType::Error => "Error",
            TxtType::Warn => "Warn",
            TxtType::Info => "Info",
            TxtType::User => "User",
        };
        write!(f, "{s}")
    }
}
readonly_struct!(
    Txt ,
    "Text transmission",
    {talker: Talker},

    {
        message : Vec<( Option<TxtType>,Option<String>)>,
        "Text information"
    }
);

impl INmeaData for Txt {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> mischief::Result<Self> {
        clerk::trace!("Txt::new: sentence='{}'", ctx.full_str());

        for l in ctx.full_str().lines() {
            NMEA_VALIDATE.apply(l)?;
        }

        let mut infos = Vec::new();
        for _ in 0..ctx.full_str().lines().count() {
            let txt_type = ctx
                .skip_strict(&UNTIL_COMMA_DISCARD)?
                .skip_strict(&UNTIL_COMMA_DISCARD)?
                .skip_strict(&UNTIL_COMMA_DISCARD)?
                .take(&UNTIL_COMMA_DISCARD)
                .parse_opt::<u8>()
                .map(TxtType::try_from)
                .and_then(Result::ok);
            let info = ctx.take(&UNTIL_STAR_DISCARD).map(|f| f.to_string());
            infos.push((txt_type, info));
            ctx.skip(&UNTIL_NEW_LINE_DISCARD);
        }

        Ok(Self {
            talker,
            message: infos,
        })
    }
}

impl fmt::Debug for Txt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("TXT");
        ds.field("talker", &self.talker);

        ds.field(
            "message",
            &self
                .message
                .iter()
                .filter(|x| x.0.is_some() || x.1.is_some())
                .map(|x| match x {
                    (None, None) => panic!("Null txt info"),
                    (None, Some(i)) => i.to_string(),
                    (Some(t), None) => {
                        let mut s = String::new();
                        write!(s, "{t}: ").unwrap();
                        s
                    }
                    (Some(t), Some(i)) => {
                        let mut s = String::new();
                        write!(s, "{t}: {i}").unwrap();
                        s
                    }
                })
                .collect::<Vec<String>>(),
        );

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use clerk::{LogLevel, init_log_with_level};
    extern crate std;
    use std::println;
    use std::string::ToString;

    use super::*;
    #[test]
    fn test_new_txt() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPTXT,03,01,02,MA=CASIC*25\r\n$GPTXT,03,02,02,IC=ATGB03+ATGR201*70\r\n$GPTXT,03,03,02,SW=URANUS2,V2.2.1.0*1D";
        let mut ctx = StrParserContext::new();
        let txt = Txt::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{txt:?}");
        assert_eq!(txt.talker, Talker::GP);
        assert_eq!(txt.message.len(), 3);
        assert_eq!(txt.message[0].0, Some(TxtType::Info));
        assert_eq!(txt.message[0].1, Some("MA=CASIC".to_string()));
        assert_eq!(txt.message[1].0, Some(TxtType::Info));
        assert_eq!(txt.message[1].1, Some("IC=ATGB03+ATGR201".to_string()));
        assert_eq!(txt.message[2].0, Some(TxtType::Info));
        assert_eq!(txt.message[2].1, Some("SW=URANUS2,V2.2.1.0".to_string()));
        Ok(())
    }
}
