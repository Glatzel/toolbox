use core::fmt;
extern crate alloc;
use alloc::string::String;
use core::fmt::Write;

use derive_getters::Getters;
use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::RaxNmeaError;
use crate::data::{INmeaData, Talker};
use crate::rules::*;

///Poll a standard message (Talker ID GL)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Vlw {
    talker: Talker,
    /// Total cumulative water distance
    twd: Option<f64>,
    /// Water distance since reset
    wd: Option<f64>,
    /// Total cumulative ground distance
    tgd: Option<f64>,
    /// Ground distance since reset
    gd: Option<f64>,
}

impl INmeaData for Vlw {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> Result<Self, RaxNmeaError> {
        ctx.global(&NmeaValidate)?;
        let twd = ctx
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)
            .parse_opt();
        ctx.skip_strict(&UNTIL_COMMA_DISCARD)?;
        let wd = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        ctx.skip_strict(&UNTIL_COMMA_DISCARD)?;
        let tgd = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        ctx.skip_strict(&UNTIL_COMMA_DISCARD)?;
        let gd = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        Ok(Vlw {
            talker,
            twd,
            wd,
            tgd,
            gd,
        })
    }
}

impl fmt::Debug for Vlw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("DHV");
        ds.field("talker", &self.talker);

        if let Some(ref twd) = self.twd {
            let mut s = String::new();
            write!(s, "{twd} N")?;
            ds.field("twd", &s);
        }
        if let Some(ref wd) = self.wd {
            let mut s = String::new();
            write!(s, "{wd} N")?;
            ds.field("wd", &s);
        }
        if let Some(ref tgd) = self.tgd {
            let mut s = String::new();
            write!(s, "{tgd} N")?;
            ds.field("tgd", &s);
        }
        if let Some(ref gd) = self.gd {
            let mut s = String::new();
            write!(s, "{gd} N")?;
            ds.field("gd", &s);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use std::println;
    use std::string::ToString;

    use clerk::{LogLevel, init_log_with_level};
    extern crate std;
    use super::*;
    #[test]
    fn test_new_vlw() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPVLW,,N,,N,15.8,N,1.2,N*65";
        let mut ctx = StrParserContext::new();
        let vlw = Vlw::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{vlw:?}");
        insta::assert_debug_snapshot!(vlw);
        Ok(())
    }
}
