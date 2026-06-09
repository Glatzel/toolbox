extern crate alloc;
use alloc::string::String;

use derive_getters::Getters;
use rax::string::{ Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;

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
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)
            .and_then(|s| s.parse().ok());

        Ok(Gnq { msg_id })
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
        let mut parser = Decoder::new();
        let gnq = Gnq::decode(parser.init(s.to_string()))?;
        println!("{gnq:?}");
        insta::assert_json_snapshot!(gnq);
        Ok(())
    }
}
