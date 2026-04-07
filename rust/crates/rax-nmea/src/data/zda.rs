use core::fmt;

use derive_getters::Getters;
use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::RaxNmeaError;
use crate::data::{INmeaData, Talker};
use crate::rules::*;
///Time and date
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Zda {
    talker: Talker,
    /// UTC time of the position fix
    time: Option<chrono::NaiveTime>,
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

impl INmeaData for Zda {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> Result<Self, RaxNmeaError> {
        ctx.global(&NmeaValidate)?;

        let time = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime);
        let day = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let month = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let year = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let ltzh = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let ltzn = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();

        Ok(Zda {
            talker,
            time,
            day,
            month,
            year,
            ltzh,
            ltzn,
        })
    }
}

impl fmt::Debug for Zda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("ZDA");
        ds.field("talker", &self.talker);

        if let Some(ref time) = self.time {
            ds.field("time", time);
        }
        if let Some(day) = self.day {
            ds.field("day", &day);
        }
        if let Some(month) = self.month {
            ds.field("month", &month);
        }
        if let Some(year) = self.year {
            ds.field("year", &year);
        }
        if let Some(ltzh) = self.ltzh {
            ds.field("ltzh", &ltzh);
        }
        if let Some(ltzn) = self.ltzn {
            ds.field("ltzn", &ltzn);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use clerk::{Level, init_log_with_level};
    extern crate std;
    use std::println;
    use std::string::ToString;

    use super::*;
    #[test]
    fn test_new_zda() -> mischief::Result<()> {
        init_log_with_level(Level::TRACE);
        let s = "$GPZDA,160012.71,11,03,2004,-1,00*7D";
        let mut ctx = StrParserContext::new();
        let zda = Zda::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{zda:?}");
        insta::assert_debug_snapshot!(zda);
        Ok(())
    }
}
