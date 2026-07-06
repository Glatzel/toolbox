extern crate alloc;
use alloc::string::String;

use derive_getters::Getters;
use rax::string::{Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;

/// Poll a standard message(Talker ID GB)"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gbq {
    /// Message ID of the message to be polled
    msg_id: Option<String>,
}
impl IDecode<RaxNmeaError> for Gbq {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let msg_id = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)?
            .parse_option()?;

        Ok(Gbq { msg_id })
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[test]
    fn test_new_gbq() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$EIGBQ,RMC*28";
        let mut decoder = Decoder::new(s);
        let gbq = Gbq::decode(&mut decoder)?;
        println!("{gbq:?}");
        insta::assert_json_snapshot!(gbq);
        Ok(())
    }
}
