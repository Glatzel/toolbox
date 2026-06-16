use derive_getters::Getters;
use rax::string::{Decoder, IDecode};
extern crate alloc;

use crate::RaxNmeaError;
use crate::common::FaaMode;
use crate::rules::*;
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
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)
            .and_then(|s| s.parse().ok());
        parser.skip_strict(&UNTIL_COMMA_DISCARD)?;

        let cogm = parser
            .take(&UNTIL_COMMA_DISCARD)
            .and_then(|s| s.parse().ok());
        parser.skip_strict(&UNTIL_COMMA_DISCARD)?;

        let sogn = parser
            .take(&UNTIL_COMMA_DISCARD)
            .and_then(|s| s.parse().ok());
        parser.skip_strict(&UNTIL_COMMA_DISCARD)?;

        let sogk = parser
            .take(&UNTIL_COMMA_DISCARD)
            .and_then(|s| s.parse().ok());
        parser.skip_strict(&UNTIL_COMMA_DISCARD)?;

        let pos_mode = parser
            .take(&UNTIL_STAR_DISCARD)
            .and_then(|s| s.parse().ok());

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
    #[test]
    fn test_new_vtg() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPVTG,83.7,T,83.7,M,146.3,N,271.0,K,D*22";
        let mut decoder = Decoder::new(s);
        let vtg = Vtg::decode(&mut decoder)?;
        println!("{vtg:?}");
        insta::assert_json_snapshot!(vtg);
        Ok(())
    }
}
