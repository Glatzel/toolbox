use derive_getters::Getters;
use jiff::civil::Time;
use rax::text::{IParseStr, StrParser};

use crate::RaxNmeaError;
use crate::rules::{NmeaTime, UNTIL_COMMA_DISCARD, UNTIL_STAR_DISCARD};
use crate::utils::ParseOptionPrimitive;
///GNSS pseudorange error statistics
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gst {
    /// UTC time of the position fix
    time: Option<Time>,

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
impl IParseStr<RaxNmeaError, true> for Gst {
    fn parse_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut parser = StrParser::new(input);
        let time = parser.skip(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime)?;
        let rms = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let std_major = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let std_minor = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let orient = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let std_lat = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let std_lon = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let std_alt = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Self {
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
    use super::*;
    use crate::test_sentence;

    test_sentence!(
        test_gst,1,
        Gst,
        "$GPGST,182141.000,15.5,15.3,7.2,21.8,0.9,0.5,0.8*54"
    );
}
