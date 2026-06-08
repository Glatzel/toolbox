use core::fmt;
extern crate alloc;
use alloc::string::String;

use derive_getters::Getters;
use rax::string::{IDecode, ParseOptExt, Parser};

use crate::RaxNmeaError;
use crate::rules::*;

///Poll a standard message (Talker ID GL)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Glq {
    /// Message ID of the message to be polled
    msg_id: Option<String>,
}
impl IDecode<RaxNmeaError> for Glq {
    fn decode(ctx: &mut Parser) -> Result<Self, RaxNmeaError> {
        ctx.global(&NmeaValidate)?;
        let msg_id = ctx
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)
            .parse_opt();

        Ok(Glq { msg_id })
    }
}

impl fmt::Debug for Glq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("DHV");

        if let Some(ref msg_id) = self.msg_id {
            ds.field("msg_id", msg_id);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use std::println;
    use std::string::ToString;

    use clerk::{LevelFilter, init_log_with_level};
    extern crate std;
    use super::*;
    #[test]
    fn test_new_glq() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$EIGLQ,RMC*26";
        let mut parser = Parser::new();
        let glq = Glq::decode(parser.init(s.to_string()))?;
        println!("{glq:?}");
        insta::assert_debug_snapshot!(glq);
        Ok(())
    }
}
