extern crate alloc;
use alloc::string::String;

use derive_getters::Getters;
use rax::string::{Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;

///Poll a standard message (Talker ID GL)"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gnq {
    /// Message ID of the message to be polled
    msg_id: Option<String>,
}
impl IDecode<RaxNmeaError> for Gnq {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let msg_id = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)?
            .parse_option()?;

        Ok(Gnq { msg_id })
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
        let mut decoder = Decoder::new(input);
        let gnq = Gnq::decode(&mut decoder)?;
        println!("{gnq:?}");
        insta::assert_json_snapshot!(index, gnq);
        Ok(())
    }
}
