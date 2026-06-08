extern crate alloc;
use alloc::string::String;

use derive_getters::Getters;
use rax::string::{IDecode, ParseOptExt, Parser};

use crate::RaxNmeaError;
use crate::rules::*;

/// Poll a standard message(Talker ID GB)"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gbq {
    /// Message ID of the message to be polled
    msg_id: Option<String>,
}
impl IDecode<RaxNmeaError> for Gbq {
    fn decode(parser: &mut Parser) -> Result<Self, RaxNmeaError> {
        parser.global(&NmeaValidate)?;
        let msg_id = parser
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_STAR_DISCARD)
            .parse_opt();

        Ok(Gbq { msg_id })
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
    fn test_new_gbq() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$EIGBQ,RMC*28";
        let mut parser = Parser::new();
        let gbq = Gbq::decode(parser.init(s.to_string()))?;
        println!("{gbq:?}");
        insta::assert_json_snapshot!(gbq);
        Ok(())
    }
}
