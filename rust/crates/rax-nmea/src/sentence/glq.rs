extern crate alloc;
use alloc::string::String;

use derive_getters::Getters;
use rax::string::{ Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;

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
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)
            .and_then(|s| s.parse().ok());

        Ok(Glq { msg_id })
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
        let mut parser = Decoder::new();
        let glq = Glq::decode(parser.init(s.to_string()))?;
        println!("{glq:?}");
        insta::assert_json_snapshot!(glq);
        Ok(())
    }
}
