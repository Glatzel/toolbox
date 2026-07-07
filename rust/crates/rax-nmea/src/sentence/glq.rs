extern crate alloc;
use alloc::string::String;

use derive_getters::Getters;
use rax::string::{Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;

///Poll a standard message (Talker ID GL)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Glq {
    /// Message ID of the message to be polled
    msg_id: Option<String>,
}
impl IDecode<RaxNmeaError> for Glq {
    fn decode(ctx: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let msg_id = ctx
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)?
            .parse_option()?;

        Ok(Glq { msg_id })
    }
}

#[cfg(test)]
mod test {
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};
    extern crate std;
    use super::*;
    #[rstest::rstest]
    #[case("1", "$EIGLQ,RMC*26")]
    fn test_glq(#[case] index: &str, #[case] input: &str) -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = Decoder::new(input);
        let glq = Glq::decode(&mut decoder)?;
        println!("{glq:?}");
        insta::assert_json_snapshot!(index, glq);
        Ok(())
    }
}
