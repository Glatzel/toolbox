use derive_getters::Getters;
use rax::string::{Decoder, IDecode};

use crate::RaxNmeaError;
use crate::common::FaaMode;
use crate::rules::*;

#[doc = "Poll a standard message (Talker ID GL)"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Ths {
    /// Heading of vehicle (true)
    headt: Option<f64>,

    /// Mode indicator
    mi: Option<FaaMode>,
}
impl IDecode<RaxNmeaError> for Ths {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let headt = parser
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)
            .and_then(|s| s.parse().ok());
        let mi = parser
            .take(&UNTIL_STAR_DISCARD)
            .and_then(|s| s.parse().ok());

        Ok(Ths { headt, mi })
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[test]
    fn test_parse() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPTHS,77.52,E*34";
        let mut decoder = Decoder::new(s);
        let ths = Ths::decode(&mut decoder)?;
        println!("{ths:?}");
        insta::assert_json_snapshot!(ths);
        Ok(())
    }
}
