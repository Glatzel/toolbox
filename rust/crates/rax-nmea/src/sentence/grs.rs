extern crate alloc;

use alloc::vec::Vec;

use derive_getters::Getters;
use rax::string::{Decoder, IDecode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::common::SystemId;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GrsResidualMode {
    #[strum(serialize = "Used in GGA", serialize = "0")]
    UsedInGga,

    #[strum(serialize = "Calculated after GGA", serialize = "1")]
    CalculatedAfterGga,
}

/// GNSS range residuals
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Grs {
    /// UTC time of the position fix
    time: Option<chrono::NaiveTime>,

    /// GRS residual mode
    mode: Option<GrsResidualMode>,

    /// Satellite residuals
    residual: Vec<f64>,

    /// System ID
    system_id: Option<SystemId>,

    /// Signal ID
    signal_id: Option<u16>,
}
impl IDecode<RaxNmeaError> for Grs {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let time = parser.skip(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime)?;

        let mode = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        clerk::debug!(
            "Grs::new: utc_time={:?}, grs_residual_mode={:?}",
            time,
            mode
        );

        let mut residual = Vec::with_capacity(12);
        for _ in 0..12 {
            match parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()? {
                Some(r) => residual.push(r),
                None => (),
            }
        }
        clerk::debug!("Grs::new: satellite_residuals={:?}", residual);

        let system_id = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);
        let signal_id = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;
        Ok(Grs {
            time,
            mode,
            residual,
            system_id,
            signal_id,
        })
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[test]
    fn test_grs() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGRS,220320.0,0,-0.8,-0.2,-0.1,-0.2,0.8,0.6,,,,,,,*55";
        let mut decoder = Decoder::new(s);
        let grs = Grs::decode(&mut decoder)?;
        println!("{grs:?}");
        insta::assert_json_snapshot!(grs);
        Ok(())
    }
}
