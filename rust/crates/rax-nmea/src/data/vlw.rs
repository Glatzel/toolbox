extern crate alloc;

use derive_getters::Getters;
use rax::string::{DecodeOptExt, Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;

///Poll a standard message (Talker ID GL)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Vlw {
    /// Total cumulative water distance
    twd: Option<f64>,

    /// Water distance since reset
    wd: Option<f64>,

    /// Total cumulative ground distance
    tgd: Option<f64>,

    /// Ground distance since reset
    gd: Option<f64>,
}

impl IDecode<RaxNmeaError> for Vlw {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let twd = parser
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)
            .decode_opt();
        parser.skip_strict(&UNTIL_COMMA_DISCARD)?;
        let wd = parser.take(&UNTIL_COMMA_DISCARD).decode_opt();
        parser.skip_strict(&UNTIL_COMMA_DISCARD)?;
        let tgd = parser.take(&UNTIL_COMMA_DISCARD).decode_opt();
        parser.skip_strict(&UNTIL_COMMA_DISCARD)?;
        let gd = parser.take(&UNTIL_COMMA_DISCARD).decode_opt();
        Ok(Vlw { twd, wd, tgd, gd })
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
    fn test_new_vlw() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPVLW,,N,,N,15.8,N,1.2,N*65";
        let mut parser = Decoder::new();
        let vlw = Vlw::decode(parser.init(s.to_string()))?;
        println!("{vlw:?}");
        insta::assert_json_snapshot!(vlw);
        Ok(())
    }
}
