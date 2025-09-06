use std::fmt;
use std::str::FromStr;

use rax::str_parser::{ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::data::{INmeaData, SystemId, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum GrsResidualMode {
    UsedInGga,
    CalculatedAfterGga,
}
impl FromStr for GrsResidualMode {
    type Err = mischief::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::UsedInGga),
            "1" => Ok(Self::CalculatedAfterGga),
            other => mischief::bail!("Unknown GrsResidualMode: {}", other),
        }
    }
}
readonly_struct!(
    Grs ,
    "GNSS range residuals",
    {talker: Talker},

    {
        time: Option<chrono::NaiveTime>,
        "UTC time of the position fix"
    },
    {
        mode : Option<GrsResidualMode>,
        "GRS residual mode"
    },
    {
        residual:Vec<f64>,
        "Satellite residuals"
    },
    {
        system_id: Option<SystemId>,
        "System ID"
    },
    {
        signal_id: Option<u16>,
        "Signal ID"
    }
);
impl INmeaData for Grs {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> mischief::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let time = ctx.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NMEA_TIME);

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

    use clerk::{LogLevel, init_log_with_level};
    use float_cmp::assert_approx_eq;

    use super::*;
    #[test]
    fn test_grs() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let input = "$GPGRS,220320.0,0,-0.8,-0.2,-0.1,-0.2,0.8,0.6,,,,,,,*55";
        let mut ctx = StrParserContext::new();
        let grs = Grs::new(ctx.init(input.to_string()), Talker::GP)?;
        println!("{grs:?}");

        assert_eq!(grs.talker, Talker::GP);
        assert!(grs.time.unwrap().to_string().contains("22:03:20"));
        assert_eq!(grs.mode.unwrap(), GrsResidualMode::UsedInGga);
        assert_eq!(grs.residual.len(), 6);
        assert_approx_eq!(f64, grs.residual[0], -0.8);
        assert_approx_eq!(f64, grs.residual[1], -0.2);
        assert_approx_eq!(f64, grs.residual[2], -0.1);
        assert_approx_eq!(f64, grs.residual[3], -0.2);
        assert_approx_eq!(f64, grs.residual[4], 0.8);
        assert_approx_eq!(f64, grs.residual[5], 0.6);
        assert!(grs.system_id.is_none());
        assert!(grs.signal_id.is_none());

        Ok(())
    }
}
