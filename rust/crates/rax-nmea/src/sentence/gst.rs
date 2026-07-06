use core::fmt;

use derive_getters::Getters;
use rax::string::{Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;
///GNSS pseudorange error statistics
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
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

impl fmt::Debug for Gst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GST");

        if let Some(ref time) = self.time {
            ds.field("time", time);
        }
        if let Some(rms) = self.rms {
            ds.field("rms", &rms);
        }
        if let Some(std_major) = self.std_major {
            ds.field("std_major", &std_major);
        }
        if let Some(std_minor) = self.std_minor {
            ds.field("std_minor", &std_minor);
        }
        if let Some(orient) = self.orient {
            ds.field("orient", &orient);
        }
        if let Some(std_lat) = self.std_lat {
            ds.field("std_lat", &std_lat);
        }
        if let Some(std_lon) = self.std_lon {
            ds.field("std_lon", &std_lon);
        }
        if let Some(std_alt) = self.std_alt {
            ds.field("std_alt", &std_alt);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[test]
    fn test_new_gst() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGST,182141.000,15.5,15.3,7.2,21.8,0.9,0.5,0.8*54";
        let mut decoder = Decoder::new(s);
        let vtg = Gst::decode(&mut decoder)?;
        println!("{vtg:?}");
        insta::assert_json_snapshot!(vtg);
        Ok(())
    }
}
