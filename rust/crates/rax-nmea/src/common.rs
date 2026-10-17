extern crate alloc;

use alloc::string::ToString;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;

/// NMEA sentence formatter identifier.
///
/// The identifier is the three-character field following the talker ID in an
/// NMEA sentence, for example `GGA` in `$GPGGA,...`.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Identifier {
    /// Delta position (DHV).
    #[strum(serialize = "DHV")]
    DHV,

    /// Datum reference.
    #[strum(serialize = "DTM")]
    DTM,

    /// Poll a standard message
    #[strum(serialize = "GBQ")]
    GBQ,

    /// GPS satellite fault detection.
    #[strum(serialize = "GBS")]
    GBS,

    /// Global Positioning System fix data.
    #[strum(serialize = "GGA")]
    GGA,

    /// Geographic position: latitude and longitude.
    #[strum(serialize = "GLL")]
    GLL,

    /// Poll a standard message
    #[strum(serialize = "GLQ")]
    GLQ,

    /// Poll a standard message
    #[strum(serialize = "GNQ")]
    GNQ,

    /// GNSS fix data.
    #[strum(serialize = "GNS")]
    GNS,

    /// Poll a standard message.
    #[strum(serialize = "GPQ")]
    GPQ,

    /// GNSS range residuals.
    #[strum(serialize = "GRS")]
    GRS,

    /// GNSS dilution of precision and active satellites.
    #[strum(serialize = "GSA")]
    GSA,

    /// GNSS pseudorange error statistics.
    #[strum(serialize = "GST")]
    GST,

    /// Satellites in view.
    #[strum(serialize = "GSV")]
    GSV,

    /// Recommended minimum specific GNSS data.
    #[strum(serialize = "RMC")]
    RMC,

    /// True heading and status.
    #[strum(serialize = "THS")]
    THS,

    /// Text transmission.
    #[strum(serialize = "TXT")]
    TXT,

    /// Dual ground/water distance.
    #[strum(serialize = "VLW")]
    VLW,

    /// Track made good and ground speed.
    #[strum(serialize = "VTG")]
    VTG,

    /// Time and date: UTC, day, month, year, and local time zone.
    #[strum(serialize = "ZDA")]
    ZDA,
}

/// Two-character NMEA talker ID.
///
/// The talker identifies the source system that produced a sentence, for
/// example `GP` for a GPS receiver in `$GPGGA,...`.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Talker {
    /// Alarm Indicator (AIS)
    #[strum(serialize = "AI")]
    AI,

    /// Auto Pilot
    #[strum(serialize = "AP")]
    AP,

    /// `BeiDou` (China)
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

    /// `BeiDou` (China)
    #[strum(serialize = "GB")]
    GB,

    /// `NavIC` / IRNSS (India)
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

/// FAA mode indicator.
///
/// The mode indicator describes the quality and provenance of the position
/// solution reported in a sentence.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FaaMode {
    /// Autonomous position solution.
    #[strum(serialize = "Autonomous", serialize = "A")]
    Autonomous,

    /// Differential position solution.
    #[strum(serialize = "Differential", serialize = "D")]
    Differential,

    /// Estimated (dead-reckoning) position.
    #[strum(serialize = "Estimated", serialize = "E")]
    Estimated,

    /// RTK float solution.
    #[strum(serialize = "Rtk Float", serialize = "F")]
    RtkFloat,

    /// Manual input mode.
    #[strum(serialize = "Manual Input", serialize = "M")]
    ManualInput,

    /// Position is not valid.
    #[strum(serialize = "Not Valid", serialize = "N")]
    NotValid,

    /// Precise (RTK fixed) solution.
    #[strum(serialize = "Precise", serialize = "P")]
    Precise,

    /// RTK integer (fixed) solution.
    #[strum(serialize = "Rtk Integer", serialize = "R")]
    RtkInteger,

    /// Simulated position.
    #[strum(serialize = "Simulator", serialize = "S")]
    Simulator,

    /// Quectel-specific mode indicator.
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
            'N' | 'V' => Ok(Self::NotValid),
            'P' => Ok(Self::Precise),
            'R' => Ok(Self::RtkInteger),
            'S' => Ok(Self::Simulator),
            'U' => Ok(Self::QuectelQuerk),

            _ => Err(RaxNmeaError::UnknownFaaMode(s.to_string())),
        }
    }
}

/// GNSS system identifier.
///
/// Used by sentences that report the constellation a satellite or signal
/// belongs to.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, strum::EnumString, strum::AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SystemId {
    /// Global Positioning System (GPS).
    #[strum(serialize = "GPS", serialize = "G", serialize = "1")]
    GPS = 1,

    /// GLONASS.
    #[strum(serialize = "GLONASS", serialize = "L", serialize = "2")]
    GLONASS = 2,

    /// Galileo.
    #[strum(serialize = "Galileo", serialize = "A", serialize = "3")]
    Galileo = 3,

    /// BeiDou Navigation Satellite System (BDS).
    #[strum(serialize = "BDS", serialize = "B", serialize = "4")]
    BDS = 4,

    /// Quasi-Zenith Satellite System (QZSS).
    #[strum(serialize = "QZSS", serialize = "Q", serialize = "5")]
    QZSS = 5,

    /// Navigation with Indian Constellation (NavIC).
    #[strum(serialize = "NavIC", serialize = "I", serialize = "6")]
    NavIC = 6,
}

/// Validity status reported by a sentence.
#[derive(
    Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord, strum::EnumString, strum::AsRefStr,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Status {
    /// Data is valid.
    #[strum(serialize = "Valid", serialize = "A")]
    Valid,

    /// Data is not valid.
    #[strum(serialize = "Invalid", serialize = "V")]
    Invalid,
}
