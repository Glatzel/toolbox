use core::fmt;

use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::RaxNmeaError;
use crate::data::{INmeaData, PosMode, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

readonly_struct!(
    Ths ,
    "Poll a standard message (Talker ID GL)",
    {talker: Talker},

    {
        headt: Option<f64>,
        "Heading of vehicle (true)"
    },
    {
        mi: Option<PosMode>,
        "Mode indicator"
    }
);
impl INmeaData for Ths {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> Result<Self, RaxNmeaError> {
        ctx.global(&NmeaValidate)?;
        let headt = ctx
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)
            .parse_opt();
        let mi = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();

        Ok(Ths { talker, headt, mi })
    }
}

impl fmt::Debug for Ths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("DHV");
        ds.field("talker", &self.talker);

        if let Some(ref headt) = self.headt {
            ds.field("headt", headt);
        }
        if let Some(ref mi) = self.mi {
            ds.field("mi", mi);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;
    use std::string::ToString;

    use clerk::{LogLevel, init_log_with_level};

    use super::*;
    #[test]
    fn test_parse() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPTHS,77.52,E*34";
        let mut ctx = StrParserContext::new();
        let ths = Ths::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{ths:?}");
        assert_eq!(ths.headt.unwrap(), 77.52);
        assert_eq!(ths.mi.unwrap(), PosMode::Estimated);
        Ok(())
    }
}
