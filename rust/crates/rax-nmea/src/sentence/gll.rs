use derive_getters::Getters;
use jiff::civil::Time;
use rax::text::{IParseStr, StrParser};

use crate::RaxNmeaError;
use crate::common::{FaaMode, Status};
use crate::rules::{NmeaCoord, NmeaTime, UNTIL_COMMA_DISCARD, UNTIL_STAR_DISCARD};
use crate::utils::ParseOptionPrimitive;

/// Latitude and longitude, with time of position fix and status
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gll {
    /// Latitude, ddmm.mmmm, where dd is degrees and mm.mmmm is minutes.
    /// Positive values indicate North, negative values indicate South.
    lat: Option<f64>,

    ///Longitude, dddmm.mmmm, where ddd is degrees and mm.mmmm is minutes.
    /// Positive values indicate East, negative values indicate West.
    lon: Option<f64>,

    /// UTC time of the position fix
    time: Option<Time>,

    /// Status of the data
    status: Option<Status>,

    /// FAA mode
    pos_mode: Option<FaaMode>,
}
impl IParseStr<RaxNmeaError, true> for Gll {
    fn parse_str(input: &str) -> Result<Self, RaxNmeaError> {
        let mut ctx = StrParser::new(input);
        clerk::trace!("Gll::decode: sentence='{}'", ctx.full_str());

        clerk::debug!("Parsing lat...");
        let lat = ctx.skip(&UNTIL_COMMA_DISCARD)?.take(&NmeaCoord)?;
        clerk::debug!("lat: {:?}", lat);

        clerk::debug!("Parsing lon...");
        let lon = ctx.take(&NmeaCoord)?;
        clerk::debug!("lon: {:?}", lon);

        clerk::debug!("Parsing utc_time...");
        let time = ctx.take(&NmeaTime)?;
        clerk::debug!("utc_time: {:?}", time);

        let status = ctx.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let pos_mode = ctx.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Self {
            lat,
            lon,
            time,
            status,
            pos_mode,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_sentence;

    test_sentence!(
        test_gll,
        1,
        Gll,
        "$GPGLL,2959.9925,S,12000.0090,E,235316.000,A,A*4E"
    );
}
