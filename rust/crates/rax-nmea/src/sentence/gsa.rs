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
pub enum GsaOperationMode {
    #[strum(serialize = "Manual", serialize = "M")]
    Manual,
    #[strum(serialize = "Automatic", serialize = "A")]
    Automatic,
}

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GsaNavigationMode {
    #[strum(serialize = "No Fix", serialize = "1")]
    NoFix,
    #[strum(serialize = "Fix 2D", serialize = "2")]
    Fix2D,
    #[strum(serialize = "Fix 3D", serialize = "3")]
    Fix3D,
}

///GNSS DOP and active satellites
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gsa {
    /// Operation mode
    op_mode: Option<GsaOperationMode>,

    /// Navigation Mode
    nav_mode: Option<GsaNavigationMode>,

    /// Satellite IDs
    svid: Vec<u8>,

    /// Position dilution of precision
    pdop: Option<f64>,

    /// Horizontal dilution of precision
    hdop: Option<f64>,

    /// Vertical dilution of precision
    vdop: Option<f64>,

    /// System ID
    system_id: Option<SystemId>,
}

impl IDecode<RaxNmeaError> for Gsa {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let op_mode = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)?
            .parse_option()?;
        clerk::trace!("Gsa::new: selection_mode={:?}", op_mode);
        let nav_mode = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        clerk::trace!("Gsa::new: mode={:?}", nav_mode);

        let mut svid = Vec::with_capacity(12);
        for _ in 0..12 {
            if let Some(id)=parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()? {
               svid.push(id)          
            }
        }
        clerk::trace!("Gsa::new: satellite_ids={:?}", svid);

        let pdop = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        clerk::trace!("Gsa::new: pdop={:?}", pdop);

        let hdop = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        clerk::trace!("Gsa::new: hdop={:?}", hdop);

        let vdop = parser
            .take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?
            .parse_option()?;
        clerk::trace!("Gsa::new: vdop={:?}", vdop);
        let _ = parser.skip(&UNTIL_COMMA_DISCARD);
        let system_id = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;
        clerk::trace!("Gsa::new: system_id={:?}", system_id);

        Ok(Gsa {
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

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;

    #[test]
    fn test_new_gsa_with_system_id() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GNGSA,A,3,05,07,13,14,15,17,19,23,24,,,,1.0,0.7,0.7,1*38";
        let mut decoder = Decoder::new(s);
        let gsa = Gsa::decode(&mut decoder)?;
        println!("{gsa:?}");
        insta::assert_json_snapshot!(gsa);

        Ok(())
    }
    #[test]
    fn test_new_gsa_without_system_id() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GPGSA,A,3,05,07,08,10,15,17,18,19,30,,,,1.2,0.9,0.8*3B";
        let mut decoder = Decoder::new(s);
        let gsa = Gsa::decode(&mut decoder)?;
        println!("{gsa:?}");
        insta::assert_json_snapshot!(gsa);
        Ok(())
    }
}
