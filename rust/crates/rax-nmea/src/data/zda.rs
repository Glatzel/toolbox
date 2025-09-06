use core::fmt;

use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;
readonly_struct!(
    Zda ,
    "Time and date",
    {talker: Talker},

    {
        time: Option<chrono::NaiveTime>,
        "UTC time of the position fix"
    },
    {
        day: Option<u8>,
        "Day of the month"
    },
    {
        month: Option<u8>,
        "Month of the year"
    },
    {
        year: Option<u16>,
        "Year"
    },
    {
        ltzh: Option<i8>,
        "Local zone description"
    },
    {
        ltzn: Option<u8>,
        "Local zone minutes description"
    }
);

impl INmeaData for Zda {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> mischief::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let time = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NMEA_TIME);
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
    use clerk::{LogLevel, init_log_with_level};
    extern crate std;
    use super::*;
    #[test]
    fn test_new_zda() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPZDA,160012.71,11,03,2004,-1,00*7D";
        let mut ctx = StrParserContext::new();
        let zda = Zda::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{zda:?}");
        assert!(zda.time.unwrap().to_string().contains("16:00:12.71"));
        assert_eq!(zda.day.unwrap(), 11);
        assert_eq!(zda.month.unwrap(), 3);
        assert_eq!(zda.year.unwrap(), 2004);
        assert_eq!(zda.ltzh.unwrap(), -1);
        assert_eq!(zda.ltzn.unwrap(), 0);
        Ok(())
    }
}
