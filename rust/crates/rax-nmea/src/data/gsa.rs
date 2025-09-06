use core::fmt;
use core::str::FromStr;
extern crate alloc;
use alloc::vec::Vec;

use rax::str_parser::{ParseOptExt, StrParserContext};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{INmeaData, SystemId, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GsaOperationMode {
    Manual,
    Automatic,
}
impl FromStr for GsaOperationMode {
    type Err = mischief::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Automatic),
            "M" => Ok(Self::Manual),
            other => mischief::bail!("Unknown GsaSelectionMode: {}", other),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GsaNavigationMode {
    NoFix,
    Fix2D,
    Fix3D,
}
impl FromStr for GsaNavigationMode {
    type Err = mischief::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::NoFix),
            "2" => Ok(Self::Fix2D),
            "3" => Ok(Self::Fix3D),
            other => mischief::bail!("Unknown GsaMode: {}", other),
        }
    }
}
readonly_struct!(
    Gsa ,
    " GNSS DOP and active satellites",
    {talker: Talker},

    {
        op_mode: Option<GsaOperationMode>,
        "Operation mode"
    },
    {
        nav_mode : Option<GsaNavigationMode>,
        "Navigation Mode"
    },
    {
        svid:Vec<u8>,
        "Satellite IDs"
    },
    {
        pdop: Option<f64>,
        "Position dilution of precision"
    },
    {
        hdop: Option<f64>,
        "Horizontal dilution of precision"
    },
    {
        vdop: Option<f64>,
        "Vertical dilution of precision"
    },
    {
        system_id:Option<SystemId>,
        "System ID"
    }
);
impl INmeaData for Gsa {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> mischief::Result<Self> {
        ctx.global(&NMEA_VALIDATE)?;

        let op_mode = ctx
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)
            .parse_opt();
        clerk::trace!("Gsa::new: selection_mode={:?}", op_mode);
        let nav_mode = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::trace!("Gsa::new: mode={:?}", nav_mode);

        let mut svid = Vec::with_capacity(12);
        for _ in 0..12 {
            match ctx.take(&UNTIL_COMMA_DISCARD).parse_opt::<u8>() {
                Some(sat_id) => svid.push(sat_id),
                None => continue,
            }
        }
        clerk::trace!("Gsa::new: satellite_ids={:?}", svid);

        let pdop = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::trace!("Gsa::new: pdop={:?}", pdop);

        let hdop = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        clerk::trace!("Gsa::new: hdop={:?}", hdop);

        let vdop = ctx.take(&UNTIL_COMMA_OR_STAR_DISCARD).parse_opt::<f64>();
        clerk::trace!("Gsa::new: vdop={:?}", vdop);

        let system_id = ctx.take(&UNTIL_STAR_DISCARD).parse_opt();
        clerk::trace!("Gsa::new: system_id={:?}", system_id);

        Ok(Gsa {
            talker,
            op_mode,
            nav_mode,
            svid,
            pdop,
            hdop,
            vdop,
            system_id,
        })
    }
}

impl fmt::Debug for Gsa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GSA");
        ds.field("talker", &self.talker);

        if let Some(op_mode) = self.op_mode {
            ds.field("op_mode", &op_mode);
        }
        if let Some(nav_mode) = self.nav_mode {
            ds.field("nav_mode", &nav_mode);
        }
        if !self.svid.is_empty() {
            ds.field("svid", &self.svid);
        }
        if let Some(pdop) = self.pdop {
            ds.field("svid", &pdop);
        }
        if let Some(hdop) = self.hdop {
            ds.field("hdop", &hdop);
        }
        if let Some(vdop) = self.vdop {
            ds.field("vdop", &vdop);
        }
        if let Some(system_id) = self.system_id {
            ds.field("system_id", &system_id);
        }

        ds.finish()
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::string::ToString;
    use std::{println, vec};

    use clerk::{LogLevel, init_log_with_level};
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn test_new_gsa_with_system_id() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GNGSA,A,3,05,07,13,14,15,17,19,23,24,,,,1.0,0.7,0.7,1*38";
        let mut ctx = StrParserContext::new();
        let gsa = Gsa::new(ctx.init(s.to_string()), Talker::GN)?;
        println!("{gsa:?}");
        assert_eq!(gsa.talker, Talker::GN);
        assert_eq!(gsa.op_mode.unwrap(), GsaOperationMode::Automatic);
        assert_eq!(gsa.nav_mode.unwrap(), GsaNavigationMode::Fix3D);
        assert_eq!(gsa.svid, vec![5, 7, 13, 14, 15, 17, 19, 23, 24]);
        assert_approx_eq!(f64, gsa.pdop.unwrap(), 1.0);
        assert_approx_eq!(f64, gsa.hdop.unwrap(), 0.7);
        assert_approx_eq!(f64, gsa.vdop.unwrap(), 0.7);
        assert_eq!(gsa.system_id, Some(SystemId::GPS));

        Ok(())
    }
    #[test]
    fn test_new_gsa_without_system_id() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGSA,A,3,05,07,08,10,15,17,18,19,30,,,,1.2,0.9,0.8*3B";
        let mut ctx = StrParserContext::new();
        let gsa = Gsa::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{gsa:?}");
        assert_eq!(gsa.talker, Talker::GP);
        assert_eq!(gsa.op_mode.unwrap(), GsaOperationMode::Automatic);
        assert_eq!(gsa.nav_mode.unwrap(), GsaNavigationMode::Fix3D);
        assert_eq!(gsa.svid, vec![5, 7, 8, 10, 15, 17, 18, 19, 30]);
        assert_approx_eq!(f64, gsa.pdop.unwrap(), 1.2);
        assert_approx_eq!(f64, gsa.hdop.unwrap(), 0.9);
        assert_approx_eq!(f64, gsa.vdop.unwrap(), 0.8);
        assert_eq!(gsa.system_id, None);

        Ok(())
    }
}
