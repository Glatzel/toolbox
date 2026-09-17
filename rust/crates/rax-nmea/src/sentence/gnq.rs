extern crate alloc;
use alloc::string::String;

use derive_getters::Getters;
use rax::text::{IParseStr, StrParser};

use crate::RaxNmeaError;
use crate::rules::{UNTIL_COMMA_DISCARD, UNTIL_STAR_DISCARD};
use crate::utils::ParseOptionPrimitive;

///Poll a standard message (Talker ID GL)"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gnq {
    /// Message ID of the message to be polled
    msg_id: Option<String>,
}
impl<'a> IParseStr<'a, RaxNmeaError, true> for Gnq {
    fn parse_str(parser: &mut StrParser<'a, true>) -> Result<Self, RaxNmeaError> {
        let msg_id = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)?
            .parse_option()?;

        Ok(Self { msg_id })
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[rstest::rstest]
    #[case("1", "$EIGNQ,RMC*24")]
    fn test_gnq(#[case] index: &str, #[case] input: &str) -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = StrParser::new(input);
        let gnq = Gnq::parse_str(&mut decoder)?;
        println!("{gnq:?}");
        insta::assert_json_snapshot!(index, gnq);
        Ok(())
    }
}
