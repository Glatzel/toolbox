extern crate alloc;

use alloc::string::ToString;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
    /// Alarm Indicator (AIS)
    #[strum(serialize = "AI")]
    AI,

    /// Auto Pilot
    #[strum(serialize = "AP")]
    AP,

    /// BeiDou (China)
    #[strum(serialize = "BD")]
    BD,

    /// Digital Selective Calling (DSC)
    #[strum(serialize = "CD")]
    CD,

    /// Electronic Chart Display & Information System (ECDIS)
    #[strum(serialize = "EC")]
    EC,

    /// Galileo Positioning System
    #[strum(serialize = "GA")]
    GA,

    /// BeiDou (China)
    #[strum(serialize = "GB")]
    GB,

    /// NavIC / IRNSS (India)
    #[strum(serialize = "GI")]
    GI,

    /// GLONASS
    #[strum(serialize = "GL")]
    GL,

    /// Combination of multiple satellite systems
    #[strum(serialize = "GN")]
    GN,

    /// Global Positioning System receiver
    #[strum(serialize = "GP")]
    GP,

    /// QZSS regional GPS augmentation system (Japan)
    #[strum(serialize = "GQ")]
    GQ,

    /// Heading / Compass
    #[strum(serialize = "HC")]
    HC,

    /// Gyro, north seeking
    #[strum(serialize = "HE")]
    HE,

    /// Integrated Instrumentation
    #[strum(serialize = "II")]
    II,

    /// Integrated Navigation
    #[strum(serialize = "IN")]
    IN,

    /// Loran-C receiver (obsolete)
    #[strum(serialize = "LC")]
    LC,

    /// QZSS (Quectel quirk)
    #[strum(serialize = "PQ")]
    PQ,

    /// QZSS regional GPS augmentation system (Japan)
    #[strum(serialize = "QZ")]
    QZ,

    /// Depth Sounder
    #[strum(serialize = "SD")]
    SD,

    /// Skytraq
    #[strum(serialize = "ST")]
    ST,

    /// Turn Indicator
    #[strum(serialize = "TI")]
    TI,

    /// Transducer
    #[strum(serialize = "YX")]
    YX,

    /// Weather Instrument
    #[strum(serialize = "WI")]
    WI,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FaaMode {
    #[strum(serialize = "Autonomous", serialize = "A")]
    Autonomous,

    #[strum(serialize = "Differential", serialize = "D")]
    Differential,

    #[strum(serialize = "Estimated", serialize = "E")]
    Estimated,

    #[strum(serialize = "Rtk Float", serialize = "F")]
    RtkFloat,

    #[strum(serialize = "Manual Input", serialize = "M")]
    ManualInput,

    #[strum(serialize = "Not Valid", serialize = "N")]
    NotValid,

    #[strum(serialize = "Precise", serialize = "P")]
    Precise,

    #[strum(serialize = "Rtk Integer", serialize = "R")]
    RtkInteger,

    #[strum(serialize = "Simulator", serialize = "S")]
    Simulator,

    #[strum(serialize = "Quectel Querk", serialize = "U")]
    QuectelQuerk,
}
impl TryFrom<&char> for FaaMode {
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
            'U' => Ok(Self::QuectelQuerk),

            _ => Err(RaxNmeaError::UnknownFaaMode(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SystemId {
    #[strum(serialize = "GPS", serialize = "G", serialize = "1")]
    GPS = 1,

    #[strum(serialize = "GLONASS", serialize = "L", serialize = "2")]
    GLONASS = 2,

    #[strum(serialize = "Galileo", serialize = "A", serialize = "3")]
    Galileo = 3,

    #[strum(serialize = "BDS", serialize = "B", serialize = "4")]
    BDS = 4,

    #[strum(serialize = "QZSS", serialize = "Q", serialize = "5")]
    QZSS = 5,

    #[strum(serialize = "NavIC", serialize = "I", serialize = "6")]
    NavIC = 6,
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
