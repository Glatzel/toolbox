use core::fmt;
extern crate alloc;
use alloc::string::String;

use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::data::{INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

readonly_struct!(
    Gnq ,
    "Poll a standard message (Talker ID GL)",
    {talker: Talker},

    {
        msg_id: Option<String>,
        "Message ID of the message to be polled"
    }
);
impl INmeaData for Gnq {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> mischief::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;
        let msg_id = ctx
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)
            .parse_opt();

        Ok(Gnq { talker, msg_id })
    }
}

impl fmt::Debug for Gnq {
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
    use std::println;
    use std::string::ToString;

    use clerk::{LogLevel, init_log_with_level};

    use super::*;
    #[test]
    fn test_new_gnq() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$EIGNQ,RMC*24";
        let mut ctx = StrParserContext::new();
        let gnq = Gnq::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{gnq:?}");
        assert_eq!(gnq.talker, Talker::GP);
        assert_eq!(gnq.msg_id.unwrap(), "RMC");
        Ok(())
    }
}
