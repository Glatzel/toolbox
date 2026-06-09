use derive_getters::Getters;
use rax::string::{Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;
///Time and date
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Zda {
    /// UTC time of the position fix
    time: Option<chrono::NaiveTime>,

    /// Day of the month
    day: Option<u8>,

    /// Month of the year
    month: Option<u8>,

    /// Year
    year: Option<u16>,

    /// Local zone description
    ltzh: Option<i8>,

    /// Local zone minutes description
    ltzn: Option<u8>,
}

impl IDecode<RaxNmeaError> for Zda {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let time = parser.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime);
        let day = parser
            .take(&UNTIL_COMMA_DISCARD)
            .and_then(|s| s.parse().ok());
        let month = parser
            .take(&UNTIL_COMMA_DISCARD)
            .and_then(|s| s.parse().ok());
        let year = parser
            .take(&UNTIL_COMMA_DISCARD)
            .and_then(|s| s.parse().ok());
        let ltzh = parser
            .take(&UNTIL_COMMA_DISCARD)
            .and_then(|s| s.parse().ok());
        let ltzn = parser
            .take(&UNTIL_STAR_DISCARD)
            .and_then(|s| s.parse().ok());

        Ok(Zda {
            time,
            day,
            month,
            year,
            ltzh,
            ltzn,
        })
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
    fn test_new_zda() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPZDA,160012.71,11,03,2004,-1,00*7D";
        let mut parser = Decoder::new();
        let zda = Zda::decode(parser.init(s.to_string()))?;
        println!("{zda:?}");
        insta::assert_json_snapshot!(zda);
        Ok(())
    }
}
