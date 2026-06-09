extern crate alloc;

use alloc::string::String;
use core::fmt;

use derive_getters::Getters;
use rax::string::{ Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;

///Poll a standard message (Talker ID GL)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Gpq {
    /// Message ID of the message to be polled
    msg_id: Option<String>,
}
impl IDecode<RaxNmeaError> for Gpq {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let msg_id = parser
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)
            .and_then(|s| s.parse().ok());

        Ok(Gpq { msg_id })
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

    use clerk::{LevelFilter, init_log_with_level};
    extern crate std;
    use std::println;
    use std::string::ToString;

    use super::*;
    #[test]
    fn test_new_gpq() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$EIGPQ,RMC*3A";
        let mut parser = Decoder::new();
        let gpq = Gpq::decode(parser.init(s.to_string()))?;
        println!("{gpq:?}");
        insta::assert_json_snapshot!(gpq);
        Ok(())
    }
}
