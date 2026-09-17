extern crate alloc;
use alloc::string::String;

use derive_getters::Getters;
use rax::text::{IParseStr, StrParser};

use crate::RaxNmeaError;
use crate::rules::{UNTIL_COMMA_DISCARD, UNTIL_STAR_DISCARD};
use crate::utils::ParseOptionPrimitive;

///Poll a standard message (Talker ID GL)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Glq {
    /// Message ID of the message to be polled
    msg_id: Option<String>,
}
impl IParseStr<RaxNmeaError, true> for Glq {
    fn parse_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut ctx = StrParser::new(input);
        let msg_id = ctx
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)?
            .parse_option()?;

        Ok(Self { msg_id })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_sentence;

    test_sentence!(test_glq, 1, Glq, "$EIGLQ,RMC*26");
}
