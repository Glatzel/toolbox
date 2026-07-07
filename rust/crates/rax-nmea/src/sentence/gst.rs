use derive_getters::Getters;
use rax::string::{Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;
///GNSS pseudorange error statistics
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gst {
    /// UTC time of the position fix
    time: Option<chrono::NaiveTime>,

    /// Root mean square
    rms: Option<f64>,

    /// Standard deviation semi-major
    std_major: Option<f64>,

    /// Standard deviation semi-minor
    std_minor: Option<f64>,

    /// Orientation
    orient: Option<f64>,

    /// Standard deviation semi-latitude
    std_lat: Option<f64>,

    /// Standard deviation semi-longitude
    std_lon: Option<f64>,

    /// Standard deviation semi-altitude
    std_alt: Option<f64>,
}
impl IDecode<RaxNmeaError> for Gst {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let time = parser.skip(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime)?;
        let rms = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let std_major = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let std_minor = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let orient = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let std_lat = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let std_lon = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let std_alt = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Gst {
            time,
            rms,
            std_major,
            std_minor,
            orient,
            std_lat,
            std_lon,
            std_alt,
        })
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[rstest::rstest]
    #[case("1", "$GPGST,182141.000,15.5,15.3,7.2,21.8,0.9,0.5,0.8*54")]
    fn test_gst(#[case] index: &str, #[case] input: &str) -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = Decoder::new(input);
        let gst = Gst::decode(&mut decoder)?;
        println!("{gst:?}");
        insta::assert_json_snapshot!(index, gst);
        Ok(())
    }
}
