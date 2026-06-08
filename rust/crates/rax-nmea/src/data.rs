extern crate alloc;

use alloc::string::ToString;
use core::fmt::Debug;

mod dhv;
mod gbs;
mod gga;
mod gll;
mod gns;
mod grs;
mod gsa;
mod gst;
mod gsv;
mod rmc;
mod txt;
mod vtg;
mod zda;

mod dtm;
mod gbq;
mod glq;
mod gnq;
mod gpq;
mod ths;
mod vlw;

pub use dhv::*;
pub use dtm::*;
pub use gbq::*;
pub use gbs::*;
pub use gga::*;
pub use gll::*;
pub use glq::*;
pub use gnq::*;
pub use gns::*;
pub use gpq::*;
pub use grs::*;
pub use gsa::*;
pub use gst::*;
pub use gsv::*;
pub use rmc::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub use ths::*;
pub use txt::*;
pub use vlw::*;
pub use vtg::*;
pub use zda::*;

use crate::RaxNmeaError;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Identifier {
    #[strum(serialize = "DHV")]
    DHV,

    ///Datum reference
    #[strum(serialize = "DTM")]
    DTM,

    /// Poll a standard message
    #[strum(serialize = "GBQ")]
    GBQ,

    ///GPS Satellite Fault Detection
    #[strum(serialize = "GBS")]
    GBS,

    ///Global Positioning System Fix Data
    #[strum(serialize = "GGA")]
    GGA,

    ///Geographic Position - Latitude/Longitude
    #[strum(serialize = "GLL")]
    GLL,

    /// Poll a standard message
    #[strum(serialize = "GLQ")]
    GLQ,

    /// Poll a standard message
    #[strum(serialize = "GNQ")]
    GNQ,

    ///Fix data
    #[strum(serialize = "GNS")]
    GNS,

    ///Poll a standard message
    #[strum(serialize = "GPQ")]
    GPQ,

    ///GPS Range Residuals
    #[strum(serialize = "GRS")]
    GRS,

    ///GPS Pseudorange Noise Statistics
    #[strum(serialize = "GSA")]
    GSA,

    ///GPS DOP and active satellites
    #[strum(serialize = "GST")]
    GST,

    ///Satellites in viewR
    #[strum(serialize = "GSV")]
    GSV,

    ///Recommended Minimum Navigation Information
    #[strum(serialize = "RMC")]
    RMC,

    ///True heading and status
    #[strum(serialize = "THS")]
    THS,

    ///Text transmission
    #[strum(serialize = "TXT")]
    TXT,

    ///Dual ground/water distance
    #[strum(serialize = "VLW")]
    VLW,

    ///Track made good and Ground speed
    #[strum(serialize = "VTG")]
    VTG,

    ///Time & Date - UTC, day, month, year and local time zone
    #[strum(serialize = "ZDA")]
    ZDA,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Talker {
    ///BeiDou (China)
    #[strum(serialize = "BD")]
    BD,
    ///Galileo Positioning System
    #[strum(serialize = "GA")]
    GA,
    ///GLONASS, according to IEIC 61162-1
    #[strum(serialize = "GL")]
    GL,
    ///Combination of multiple satellite systems (NMEA 1083)
    #[strum(serialize = "GN")]
    GN,
    ///Global Positioning System receiver
    #[strum(serialize = "GP")]
    GP,
    ///QZSS (Quectel Quirk)
    #[strum(serialize = "PQ")]
    PQ,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PosMode {
    #[strum(serialize = "Autonomous", serialize = "A")]
    Autonomous,

    #[strum(serialize = "Differential", serialize = "D")]
    Differential,

    #[strum(serialize = "Estimated", serialize = "E")]
    Estimated,

    #[strum(serialize = "RtkFloat", serialize = "F")]
    RtkFloat,

    #[strum(serialize = "ManualInput", serialize = "M")]
    ManualInput,

    #[strum(serialize = "NotValid", serialize = "N")]
    NotValid,

    #[strum(serialize = "Precise", serialize = "P")]
    Precise,

    #[strum(serialize = "RtkInteger", serialize = "R")]
    RtkInteger,

    #[strum(serialize = "Simulator", serialize = "S")]
    Simulator,
}
impl TryFrom<&char> for PosMode {
    type Error = RaxNmeaError;
    fn try_from(s: &char) -> Result<Self, Self::Error> {
        match *s {
            'A' => Ok(Self::Autonomous),
            'D' => Ok(Self::Differential),
            'E' => Ok(Self::Estimated),
            'F' => Ok(Self::RtkFloat),
            'M' => Ok(Self::ManualInput),
            'N' => Ok(Self::NotValid),
            'P' => Ok(Self::Precise),
            'R' => Ok(Self::RtkInteger),
            'S' => Ok(Self::Simulator),
            'V' => Ok(Self::NotValid),
            _ => Err(RaxNmeaError::UnknownFaaMode(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SystemId {
    #[strum(serialize = "GPS", serialize = "G")]
    GPS = 1,

    #[strum(serialize = "GLONASS", serialize = "L")]
    GLONASS = 2,

    #[strum(serialize = "BDS", serialize = "B")]
    BDS = 3,

    #[strum(serialize = "QZSS", serialize = "Q")]
    QZSS = 4,

    #[strum(serialize = "NavIC", serialize = "I")]
    NavIC = 5,
}

#[derive(
    Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord, strum::EnumString, strum::AsRefStr,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Status {
    #[strum(serialize = "Valid", serialize = "A")]
    Valid,

    #[strum(serialize = "Invalid", serialize = "V")]
    Invalid,
}
