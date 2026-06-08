extern crate alloc;

use alloc::vec::Vec;
use core::fmt;

use derive_getters::Getters;
use rax::string::{IDecode, ParseOptExt, Parser};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::data::SystemId;
use crate::rules::*;

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
#[derive(Clone, Getters)]
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
    fn decode(parser: &mut Parser) -> Result<Self, RaxNmeaError> {
        parser.global(&NmeaValidate)?;

        let time = parser.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime);

        let mode = parser.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!(
            "Grs::new: utc_time={:?}, grs_residual_mode={:?}",
            time,
            mode
        );

        let mut residual = Vec::with_capacity(12);
        for _ in 0..12 {
            match parser.take(&UNTIL_COMMA_DISCARD).parse_opt::<f64>() {
                Some(r) => residual.push(r),
                None => continue,
            }
        }
        clerk::debug!("Grs::new: satellite_residuals={:?}", residual);

        let system_id = parser.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let signal_id = parser.take(&UNTIL_STAR_DISCARD).parse_opt();
        Ok(Grs {
            time,
            mode,
            residual,
            system_id,
            signal_id,
        })
    }
}

impl fmt::Debug for Grs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GSA");

        if let Some(time) = self.time {
            ds.field("time", &time);
        }

        if let Some(mode) = self.mode {
            ds.field("mode", &mode);
        }

        ds.field("residual", &self.residual);

        if let Some(system_id) = self.system_id {
            ds.field("system_id", &system_id);
        }

        if let Some(signal_id) = self.signal_id {
            ds.field("signal_id", &signal_id);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;
    use std::string::ToString;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[test]
    fn test_grs() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let input = "$GPGRS,220320.0,0,-0.8,-0.2,-0.1,-0.2,0.8,0.6,,,,,,,*55";
        let mut parser = Parser::new();
        let grs = Grs::decode(parser.init(input.to_string()))?;
        println!("{grs:?}");
        insta::assert_json_snapshot!(grs);
        Ok(())
    }
}
