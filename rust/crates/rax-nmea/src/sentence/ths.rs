use derive_getters::Getters;
use rax::string::{Decoder, IDecode};

use crate::RaxNmeaError;
use crate::common::FaaMode;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;

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
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)?
            .parse_option()?;
        let mi = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Ths { headt, mi })
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[rstest::rstest]
    #[case("1", "$GPTHS,77.52,E*34")]
    fn test_ths(#[case] index: &str, #[case] input: &str) -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = Decoder::new(input);
        let ths = Ths::decode(&mut decoder)?;
        println!("{ths:?}");
        insta::assert_json_snapshot!(index, ths);
        Ok(())
    }
}
