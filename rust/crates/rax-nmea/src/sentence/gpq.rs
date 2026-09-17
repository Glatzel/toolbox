extern crate alloc;

use alloc::string::String;
use core::fmt;

use derive_getters::Getters;
use rax::text::{IParseStr, StrParser};

use crate::RaxNmeaError;
use crate::rules::{UNTIL_COMMA_DISCARD, UNTIL_STAR_DISCARD};
use crate::utils::ParseOptionPrimitive;

///Poll a standard message (Talker ID GL)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Gpq {
    /// Message ID of the message to be polled
    msg_id: Option<String>,
}
impl IParseStr<RaxNmeaError, true> for Gpq {
    fn parse_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut parser = StrParser::new(input);
        let msg_id = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)?
            .parse_option()?;

        Ok(Self { msg_id })
    }
}

impl fmt::Debug for Gpq {
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
    use super::*;
    use crate::test_sentence;

    test_sentence!(test_gpq, 1, Gpq, "$EIGPQ,RMC*3A");
}
