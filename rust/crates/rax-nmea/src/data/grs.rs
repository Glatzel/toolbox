use core::fmt;
use core::str::FromStr;
extern crate alloc;
use alloc::string::ToString;
use alloc::vec::Vec;

use derive_getters::Getters;
use rax::str_parser::{ParseOptExt, StrParserContext};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::data::{INmeaData, SystemId, Talker};
use crate::rules::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GrsResidualMode {
    UsedInGga,
    CalculatedAfterGga,
}
impl FromStr for GrsResidualMode {
    type Err = RaxNmeaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::UsedInGga),
            "1" => Ok(Self::CalculatedAfterGga),
            other => Err(RaxNmeaError::UnknownGrsResidualMode(other.to_string())),
        }
    }
}
/// GNSS range residuals
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Getters)]
pub struct Grs {
    talker: Talker,
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
impl INmeaData for Grs {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> Result<Self, RaxNmeaError> {
        ctx.global(&NmeaValidate)?;

        let time = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime);

        let mode = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::debug!(
            "Grs::new: utc_time={:?}, grs_residual_mode={:?}",
            time,
            mode
        );

        let mut residual = Vec::with_capacity(12);
        for _ in 0..12 {
            match ctx.take(&UNTIL_COMMA_DISCARD).parse_opt::<f64>() {
                Some(r) => residual.push(r),
                None => continue,
            }
        }
        clerk::debug!("Grs::new: satellite_residuals={:?}", residual);

        let system_id = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let signal_id = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();
        Ok(Grs {
            talker,
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
        ds.field("talker", &self.talker);

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
        let mut ctx = StrParserContext::new();
        let grs = Grs::new(ctx.init(input.to_string()), Talker::GP)?;
        println!("{grs:?}");
        insta::assert_debug_snapshot!(grs);
        Ok(())
    }
}
