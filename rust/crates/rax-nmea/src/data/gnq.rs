use core::fmt;
extern crate alloc;
use alloc::string::String;

use derive_getters::Getters;
use rax::str_parser::{ParseOptExt, StrParserContext};

use crate::RaxNmeaError;
use crate::data::{INmeaData, Talker};
use crate::rules::*;

///Poll a standard message (Talker ID GL)"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Gnq {
    talker: Talker,
    /// Message ID of the message to be polled
    msg_id: Option<String>,
}
impl INmeaData for Gnq {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> Result<Self, RaxNmeaError> {
        ctx.global(&NmeaValidate)?;
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

    use clerk::{Level, init_log_with_level};

    use super::*;
    #[test]
    fn test_new_gnq() -> mischief::Result<()> {
        init_log_with_level(Level::TRACE);
        let s = "$EIGNQ,RMC*24";
        let mut ctx = StrParserContext::new();
        let gnq = Gnq::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{gnq:?}");
        insta::assert_debug_snapshot!(gnq);
        Ok(())
    }
}
