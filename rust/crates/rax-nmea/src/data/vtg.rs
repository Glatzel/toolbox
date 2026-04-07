use core::fmt;

use derive_getters::Getters;
use rax::str_parser::{ParseOptExt, StrParserContext};
extern crate alloc;
use alloc::string::String;
use core::fmt::Write;

use crate::RaxNmeaError;
use crate::data::{INmeaData, PosMode, Talker};
use crate::rules::*;
///Course over ground and ground speed
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Vtg {
    talker: Talker,
    /// Course over ground (true)
    cogt: Option<f64>,
    /// Course over ground (magnetic)
    cogm: Option<f64>,
    /// Speed over ground (knots)
    sogn: Option<f64>,
    /// Speed over ground (kph)
    sogk: Option<f64>,
    /// Mode
    pos_mode: Option<PosMode>,
}

impl INmeaData for Vtg {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> Result<Self, RaxNmeaError> {
        ctx.global(&NmeaValidate)?;

        let cogt = ctx
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)
            .parse_opt();
        ctx.skip_strict(&UNTIL_COMMA_DISCARD)?;

        let cogm = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        ctx.skip_strict(&UNTIL_COMMA_DISCARD)?;

        let sogn = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        ctx.skip_strict(&UNTIL_COMMA_DISCARD)?;

        let sogk = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        ctx.skip_strict(&UNTIL_COMMA_DISCARD)?;

        let pos_mode = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();

        Ok(Vtg {
            talker,
            cogt,
            cogm,
            sogn,
            sogk,
            pos_mode,
        })
    }
}

impl fmt::Debug for Vtg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("VTG");
        ds.field("talker", &self.talker);

        if let Some(cogt) = self.cogt {
            let mut s = String::new();
            write!(s, "{cogt} Degrees")?;
            ds.field("cogt", &s);
        }
        if let Some(cogm) = self.cogm {
            let mut s = String::new();
            write!(s, "{cogm} Degrees")?;
            ds.field("cogm", &s);
        }
        if let Some(sogn) = self.sogn {
            let mut s = String::new();
            write!(s, "{sogn} Knots")?;
            ds.field("sogn", &s);
        }
        if let Some(sogk) = self.sogk {
            let mut s = String::new();
            write!(s, "{sogk} Kph")?;
            ds.field("sogk", &s);
        }
        if let Some(ref pos_mode) = self.pos_mode {
            ds.field("pos_mode", pos_mode);
        }

        ds.finish()
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
    fn test_new_vtg() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPVTG,83.7,T,83.7,M,146.3,N,271.0,K,D*22";
        let mut ctx = StrParserContext::new();
        let vtg = Vtg::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{vtg:?}");
        insta::assert_debug_snapshot!(vtg);
        Ok(())
    }
}
