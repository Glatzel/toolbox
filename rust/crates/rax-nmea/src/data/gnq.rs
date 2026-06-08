use core::fmt;
extern crate alloc;
use alloc::string::String;

use derive_getters::Getters;
use rax::string::{IDecode, ParseOptExt, Parser};

use crate::RaxNmeaError;
use crate::rules::*;

///Poll a standard message (Talker ID GL)"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Gnq {
    /// Message ID of the message to be polled
    msg_id: Option<String>,
}
impl IDecode<RaxNmeaError> for Gnq {
    fn decode(parser: &mut Parser) -> Result<Self, RaxNmeaError> {
        parser.global(&NmeaValidate)?;
        let msg_id = parser
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)
            .parse_opt();

        Ok(Gnq { msg_id })
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

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[test]
    fn test_new_gnq() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$EIGNQ,RMC*24";
        let mut parser = Parser::new();
        let gnq = Gnq::decode(parser.init(s.to_string()))?;
        println!("{gnq:?}");
        insta::assert_debug_snapshot!(gnq);
        Ok(())
    }
}
