extern crate alloc;

use alloc::vec::Vec;
use core::fmt::Debug;
use core::str::FromStr;

use derive_getters::Getters;
use jiff::civil::Time;
use rax::text::StrParser;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::common::FaaMode;
use crate::rules::{
    NmeaCoord, NmeaTime, UNTIL_COMMA_DISCARD, UNTIL_COMMA_OR_STAR_KEEP_RIGHT, UNTIL_STAR_DISCARD,
};
use crate::utils::ParseOptionPrimitive;

/// Navigational status reported in the GNS sentence.
#[derive(Debug, PartialEq, Eq, Clone, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GnsNavigationStatus {
    /// Safe: the position solution is within limits (`S`).
    #[strum(serialize = "Safe", serialize = "S")]
    Safe,
    /// Caution: the position solution is degraded (`C`).
    #[strum(serialize = "Caution", serialize = "C")]
    Caution,
    /// Unsafe: the position solution exceeds limits (`U`).
    #[strum(serialize = "Unsafe", serialize = "U")]
    Unsafe,
    /// Not valid: no usable position solution (`V`).
    #[strum(serialize = "Invalid", serialize = "V")]
    Invalid,
}

/// GNSS fix data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gns {
    /// UTC time of the position fix
    time: Option<Time>,
    /// Latitude, ddmm.mmmm, where dd is degrees and mm.mmmm is minutes.
    /// Positive values indicate North, negative values indicate South.
    lat: Option<f64>,
    /// Longitude, dddmm.mmmm, where ddd is degrees and mm.mmmm is minutes.
    /// Positive values indicate East, negative values indicate West.
    lon: Option<f64>,
    /// FAA mode
    pos_mode: Vec<FaaMode>,
    /// Number of satellites in use
    num_sv: Option<u8>,
    /// Horizontal dilution of precision
    hdop: Option<f64>,
    /// Altitude
    alt: Option<f64>,
    /// Geoidal separation
    sep: Option<f64>,
    /// Differential data age
    diff_age: Option<f64>,
    /// Differential reference station ID
    diff_station: Option<u16>,
    /// Navigational status
    nav_status: Option<GnsNavigationStatus>,
}

impl FromStr for Gns {
    type Err = RaxNmeaError;
    fn from_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut parser = StrParser::new(input);

        let time = parser.skip(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime)?;
        let lat = parser.take(&NmeaCoord)?;
        let lon = parser.take(&NmeaCoord)?;
        let mode_str = parser.take(&UNTIL_COMMA_DISCARD)?;
        let pos_mode = mode_str
            .char_indices()
            .filter_map(|(_, c)| FaaMode::try_from(&c).ok())
            .collect::<Vec<FaaMode>>();
        let num_sv = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let hdop = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let alt = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);
        let sep = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);
        let diff_age = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);
        let diff_station = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);
        let nav_status = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;
        Ok(Self {
            time,
            lat,
            lon,
            pos_mode,
            num_sv,
            hdop,
            alt,
            sep,
            diff_age,
            diff_station,
            nav_status,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_sentence;

    test_sentence!(
        test_gns1,
        1,
        Gns,
        "$GPGNS,112257.00,3844.24011,N,00908.43828,W,AN,03,10.5,,*57"
    );
    test_sentence!(test_gns2, 2, Gns, "$GNGNS,181604.00,,,,,NN,00,99.99,,,,*59");
}
