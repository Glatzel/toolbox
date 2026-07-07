use derive_getters::Getters;
use rax::string::{Decoder, IDecode};
extern crate alloc;

use crate::RaxNmeaError;
use crate::common::FaaMode;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;
///Course over ground and ground speed
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Vtg {
    /// Course over ground (true)
    cogt: Option<f64>,

    /// Course over ground (magnetic)
    cogm: Option<f64>,

    /// Speed over ground (knots)
    sogn: Option<f64>,

    /// Speed over ground (kph)
    sogk: Option<f64>,

    /// Mode
    pos_mode: Option<FaaMode>,
}

impl IDecode<RaxNmeaError> for Vtg {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let cogt = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)?
            .parse_option()?;
        parser.skip(&UNTIL_COMMA_DISCARD)?;

        let cogm = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        parser.skip(&UNTIL_COMMA_DISCARD)?;

        let sogn = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        parser.skip(&UNTIL_COMMA_DISCARD)?;

        let sogk = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        parser.skip(&UNTIL_COMMA_DISCARD)?;

        let pos_mode = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Vtg {
            cogt,
            cogm,
            sogn,
            sogk,
            pos_mode,
        })
    }
}

#[cfg(test)]
mod test {
    use clerk::{LevelFilter, init_log_with_level};
    extern crate std;
    use std::println;

    use super::*;
    #[rstest::rstest]
    #[case("1", "$GPVTG,83.7,T,83.7,M,146.3,N,271.0,K,D*22")]
    fn test_vtg(#[case] index: &str, #[case] input: &str) -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = Decoder::new(input);
        let vtg = Vtg::decode(&mut decoder)?;
        println!("{vtg:?}");
        insta::assert_json_snapshot!(index, vtg);
        Ok(())
    }
}
