use std::fmt;

use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{INmeaData, PosMode, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;
readonly_struct!(
    Vtg ,
    "Course over ground and ground speed",
    {talker: Talker},

    {
        cogt: Option<f64>,
        "Course over ground (true)"
    },
    {
        cogm: Option<f64>,
        "Course over ground (magnetic)"
    },
    {
        sogn: Option<f64>,
        "Speed over ground (knots)"
    },
    {
        sogk: Option<f64>,
        "Speed over ground (kph)"
    },
    {
        pos_mode: Option<PosMode>,
        "Mode"
    }
);
impl INmeaData for Vtg {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> miette::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

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
            ds.field("cogt", &format!("{cogt} Degrees"));
        }
        if let Some(cogm) = self.cogm {
            ds.field("cogm", &format!("{cogm} Degrees"));
        }
        if let Some(sogn) = self.sogn {
            ds.field("sogn", &format!("{sogn} Knots"));
        }
        if let Some(sogk) = self.sogk {
            ds.field("sogk", &format!("{sogk} Kph"));
        }
        if let Some(ref pos_mode) = self.pos_mode {
            ds.field("pos_mode", pos_mode);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use clerk::{LogLevel, init_log_with_level};
    use float_cmp::assert_approx_eq;

    use super::*;
    #[test]
    fn test_new_vtg() -> miette::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPVTG,83.7,T,83.7,M,146.3,N,271.0,K,D*22";
        let mut ctx = StrParserContext::new();
        let vtg = Vtg::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{vtg:?}");
        assert_eq!(vtg.talker, Talker::GN);
        assert_approx_eq!(f64, vtg.cogt.unwrap(), 83.7);
        assert_approx_eq!(f64, vtg.cogm.unwrap(), 83.7);
        assert_approx_eq!(f64, vtg.sogn.unwrap(), 146.3);
        assert_approx_eq!(f64, vtg.sogk.unwrap(), 271.0);
        assert_eq!(vtg.pos_mode.unwrap(), PosMode::Differential);
        Ok(())
    }
}
