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
use core::fmt::Display;
use core::str::FromStr;
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
use rax::str_parser::StrParserContext;
pub use rmc::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub use ths::*;
pub use txt::*;
pub use vlw::*;
pub use vtg::*;
pub use zda::*;

use crate::RaxNmeaError;
pub trait INmeaData {
    fn new(ctx: &mut StrParserContext, navigation_system: Talker) -> Result<Self, RaxNmeaError>
    where
        Self: Sized;
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Identifier {
    DHV,
    ///Datum reference
    DTM,
    /// Poll a standard message
    GBQ,
    ///GPS Satellite Fault Detection
    GBS,
    ///Global Positioning System Fix Data
    GGA,
    ///Geographic Position - Latitude/Longitude
    GLL,
    /// Poll a standard message
    GLQ,
    /// Poll a standard message
    GNQ,
    ///Fix data
    GNS,
    ///Poll a standard message
    GPQ,
    ///GPS Range Residuals
    GRS,
    ///GPS Pseudorange Noise Statistics
    GSA,
    ///GPS DOP and active satellites
    GST,
    ///Satellites in viewR
    GSV,
    ///Recommended Minimum Navigation Information
    RMC,
    ///True heading and status
    THS,
    ///Text transmission
    TXT,
    ///Dual ground/water distance
    VLW,
    ///Track made good and Ground speed
    VTG,
    ///Time & Date - UTC, day, month, year and local time zone
    ZDA,
}
impl FromStr for Identifier {
    type Err = RaxNmeaError;

    fn from_str(sentence: &str) -> Result<Identifier, RaxNmeaError> {
        if sentence.len() < 6 {
            return Err(RaxNmeaError::InvalidSentence(sentence.to_string()));
        }
        match &sentence.get(3..6) {
            Some("DHV") => Ok(Self::DHV),
            Some("DTM") => Ok(Self::DTM),
            Some("GBQ") => Ok(Self::GBQ),
            Some("GBS") => Ok(Self::GBS),
            Some("GGA") => Ok(Self::GGA),
            Some("GLL") => Ok(Self::GLL),
            Some("GLQ") => Ok(Self::GLQ),
            Some("GNQ") => Ok(Self::GNQ),
            Some("GNS") => Ok(Self::GNS),
            Some("GPQ") => Ok(Self::GPQ),
            Some("GRS") => Ok(Self::GRS),
            Some("GSA") => Ok(Self::GSA),
            Some("GST") => Ok(Self::GST),
            Some("GSV") => Ok(Self::GSV),
            Some("RMC") => Ok(Self::RMC),
            Some("THS") => Ok(Self::THS),
            Some("TXT") => Ok(Self::TXT),
            Some("VLW") => Ok(Self::VLW),
            Some("VTG") => Ok(Self::VTG),
            Some("ZDA") => Ok(Self::ZDA),

            _ => Err(RaxNmeaError::UnknownIdentifier(sentence.to_string())),
        }
    }
}
impl Display for Identifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Self::DHV => "DHV",
            Self::DTM => "DTM",
            Self::GBQ => "GBQ",
            Self::GBS => "GBS",
            Self::GGA => "GGA",
            Self::GLL => "GLL",
            Self::GLQ => "GLQ",
            Self::GNQ => "GNQ",
            Self::GNS => "GNS",
            Self::GPQ => "GPQ",
            Self::GRS => "GRS",
            Self::GSA => "GSA",
            Self::GST => "GST",
            Self::GSV => "GSV",
            Self::RMC => "RMC",
            Self::THS => "THS",
            Self::TXT => "TXT",
            Self::VLW => "VLW",
            Self::VTG => "VTG",
            Self::ZDA => "ZDA",
        };
        write!(f, "{s}")
    }
}
#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Talker {
    ///BeiDou (China)
    BD,
    //Galileo Positioning System
    GA,
    ///GLONASS, according to IEIC 61162-1
    GL,
    ///Combination of multiple satellite systems (NMEA 1083)
    GN,
    ///Global Positioning System receiver
    GP,
    //QZSS (Quectel Quirk)
    PQ,
}

impl FromStr for Talker {
    type Err = RaxNmeaError;

    fn from_str(sentence: &str) -> Result<Self, RaxNmeaError> {
        match &sentence.get(1..3) {
            Some("BD") => Ok(Self::BD),
            Some("GA") => Ok(Self::GA),
            Some("GL") => Ok(Self::GL),
            Some("GN") => Ok(Self::GN),
            Some("GP") => Ok(Self::GP),
            Some("PQ") => Ok(Self::PQ),
            _ => Err(RaxNmeaError::UnknownTalker(sentence.to_string())),
        }
    }
}
impl Display for Talker {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Self::BD => "BD",
            Self::GA => "GA",
            Self::GL => "GL",
            Self::GN => "GN",
            Self::GP => "GP",
            Self::PQ => "PQ",
        };
        write!(f, "{s}")
    }
}
#[derive(PartialEq, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PosMode {
    Autonomous,
    Differential,
    Estimated,
    RtkFloat,
    ManualInput,
    NotValid,
    Precise,
    RtkInteger,
    Simulator,
}
impl FromStr for PosMode {
    type Err = RaxNmeaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Autonomous),
            "D" => Ok(Self::Differential),
            "E" => Ok(Self::Estimated),
            "F" => Ok(Self::RtkFloat),
            "M" => Ok(Self::ManualInput),
            "N" => Ok(Self::NotValid),
            "P" => Ok(Self::Precise),
            "R" => Ok(Self::RtkInteger),
            "S" => Ok(Self::Simulator),
            "V" => Ok(Self::NotValid),

            other => Err(RaxNmeaError::UnknownFaaMode(other.to_string())),
        }
    }
}
impl Display for PosMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            PosMode::Autonomous => "Autonomous",
            PosMode::Differential => "Differential",
            PosMode::Estimated => "Estimated",
            PosMode::RtkFloat => "Rtk Float",
            PosMode::ManualInput => "Manual Input",
            PosMode::NotValid => "Not Valid",
            PosMode::Precise => "Precise",
            PosMode::RtkInteger => "Rtk Integer",
            PosMode::Simulator => "Simulator",
        };
        write!(f, "{s}")
    }
}
impl TryFrom<&char> for PosMode {
    type Error = RaxNmeaError;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
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

            other => Err(RaxNmeaError::UnknownFaaMode(other.to_string())),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SystemId {
    GPS = 1,
    GLONASS = 2,
    BDS = 3,
    QZSS = 4,
    NavIC = 5,
}
impl FromStr for SystemId {
    type Err = RaxNmeaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::GPS),
            "2" => Ok(Self::GLONASS),
            "3" => Ok(Self::BDS),
            "4" => Ok(Self::QZSS),
            "5" => Ok(Self::NavIC),
            other => Err(RaxNmeaError::UnknownSystemId(other.to_string())),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Status {
    Valid,
    Invalid,
}
impl FromStr for Status {
    type Err = RaxNmeaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Valid),
            "V" => Ok(Self::Invalid),
            other => Err(RaxNmeaError::UnknownStatus(other.to_string())),
        }
    }
}
