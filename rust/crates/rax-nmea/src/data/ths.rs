use derive_getters::Getters;
use rax::string::{IDecode, DecodeOptExt, Decoder};

use crate::RaxNmeaError;
use crate::data::PosMode;
use crate::rules::*;

#[doc = "Poll a standard message (Talker ID GL)"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Ths {
    /// Heading of vehicle (true)
    headt: Option<f64>,

    /// Mode indicator
    mi: Option<PosMode>,
}
impl IDecode<RaxNmeaError> for Ths {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let headt = parser
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)
            .decode_opt();
        let mi = parser.take(&UNTIL_STAR_DISCARD).decode_opt();

        Ok(Ths { headt, mi })
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
    fn test_parse() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPTHS,77.52,E*34";
        let mut parser = Decoder::new();
        let ths = Ths::decode(parser.init(s.to_string()))?;
        println!("{ths:?}");
        insta::assert_json_snapshot!(ths);
        Ok(())
    }
}
