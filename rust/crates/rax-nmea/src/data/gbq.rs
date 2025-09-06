use std::fmt;

use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

readonly_struct!(
    Gbq ,
    "Poll a standard message(Talker ID GB)",
    {talker: Talker},

    {
        msg_id: Option<String>,
        "Message ID of the message to be polled"
    }
);
impl INmeaData for Gbq {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> mischief::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;
        let msg_id = ctx
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)
            .parse_opt();

        Ok(Gbq { talker, msg_id })
    }
}

impl fmt::Debug for Gbq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("DHV");
        ds.field("talker", &self.talker);

        if let Some(ref msg_id) = self.msg_id {
            ds.field("msg_id", msg_id);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
 extern crate std;
    use clerk::{LogLevel, init_log_with_level};

    use super::*;
    #[test]
    fn test_new_gbq() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$EIGBQ,RMC*28";
        let mut ctx = StrParserContext::new();
        let gbq = Gbq::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{gbq:?}");
        assert_eq!(gbq.talker, Talker::GP);
        assert_eq!(gbq.msg_id.unwrap(), "RMC");
        Ok(())
    }
}
