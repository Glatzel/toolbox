use derive_getters::Getters;
use rax::string::{IDecode, ParseOptExt, Parser};

use crate::RaxNmeaError;
use crate::data::{PosMode, Status};
use crate::rules::*;
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
    time: Option<chrono::NaiveTime>,
    
    /// Status of the data
    status: Option<Status>,
    
    /// FAA mode
    pos_mode: Option<PosMode>,
}
impl IDecode<RaxNmeaError> for Gll {
    fn decode(ctx: &mut Parser) -> Result<Self, RaxNmeaError> {
        clerk::trace!("Gll::decode: sentence='{}'", ctx.full_str());

        ctx.global(&NmeaValidate)?;

        clerk::debug!("Parsing lat...");
        let lat = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NmeaCoord);
        clerk::debug!("lat: {:?}", lat);

        clerk::debug!("Parsing lon...");
        let lon = ctx.take(&NmeaCoord);
        clerk::debug!("lon: {:?}", lon);

        clerk::debug!("Parsing utc_time...");
        let time = ctx.take(&NmeaTime);
        clerk::debug!("utc_time: {:?}", time);

        let status = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();

        let pos_mode = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();

        Ok(Gll {
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
    use std::println;
    use std::string::ToString;

    use clerk::{LevelFilter, init_log_with_level};

    extern crate std;
    use super::*;
    #[test]
    fn test_new_ggl() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGLL,2959.9925,S,12000.0090,E,235316.000,A,A*4E";
        let mut parser = Parser::new();
        let gll = Gll::decode(parser.init(s.to_string()))?;
        println!("{gll:?}");
        insta::assert_json_snapshot!(gll);
        Ok(())
    }
}
