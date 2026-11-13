extern crate alloc;
use alloc::string::String;
use core::convert::Infallible;
use core::fmt;
use core::num::{ParseFloatError, ParseIntError};

use mischief::IDiagnosis;

/// Errors produced while parsing or validating an NMEA sentence.
#[derive(Debug, IDiagnosis)]
pub enum RaxNmeaError {
    /// The sentence is structurally invalid.
    #[diagnosis(description = "invalid NMEA sentence")]
    InvalidSentence(String),
    /// The sentence is shorter than the minimum length required by a parser.
    #[diagnosis(description = "invalid NMEA sentence length")]
    InvalidSentenceLength(usize),
    /// The sentence does not start with the `$` delimiter.
    #[diagnosis(description = "NMEA sentence is missing the `$` prefix")]
    InvalidSentencePrefix(String),
    /// The checksum field is not exactly two hexadecimal characters.
    #[diagnosis(description = "invalid NMEA checksum length")]
    InvalidChecksumLength(usize),
    /// The `*` checksum delimiter is missing.
    #[diagnosis(description = "missing NMEA checksum delimiter")]
    MissingChecksumDelimiter(String),
    /// The calculated checksum does not match the checksum encoded in the
    /// sentence.
    #[diagnosis(description = "NMEA checksum mismatch")]
    ChecksumMismatch {
        /// Checksum calculated from the sentence payload.
        calculated: u8,
        /// Checksum read from the sentence payload.
        expected: u8,
    },

    /// The sentence identifier is not recognized.
    #[diagnosis(description = "unknown NMEA identifier")]
    UnknownIdentifier(String),
    /// The talker ID is not recognized.
    #[diagnosis(description = "unknown NMEA talker")]
    UnknownTalker(String),
    /// The FAA mode indicator is not recognized.
    #[diagnosis(description = "unknown NMEA FAA mode")]
    UnknownFaaMode(String),
    /// The GNSS system identifier is not recognized.
    #[diagnosis(description = "unknown NMEA system ID")]
    UnknownSystemId(String),
    /// The status field is not recognized.
    #[diagnosis(description = "unknown NMEA status")]
    UnknownStatus(String),
    /// The DTM datum field is not recognized.
    #[diagnosis(description = "unknown NMEA DTM datum")]
    UnknownDtmDatum(String),
    /// The navigation status field is not recognized.
    #[diagnosis(description = "unknown NMEA navigation status")]
    UnknownNavigationStatus(String),
    /// The GGA quality indicator is not recognized.
    #[diagnosis(description = "unknown NMEA GGA quality indicator")]
    UnknownGgaQualityIndicator(String),
    /// The GRS residual mode is not recognized.
    #[diagnosis(description = "unknown NMEA GRS residual mode")]
    UnknownGrsResidualMode(String),
    /// The TXT message type is not recognized.
    #[diagnosis(description = "unknown NMEA TXT type")]
    UnknownTxtType(u8),
    /// The GSA selection mode is not recognized.
    #[diagnosis(description = "unknown NMEA GSA selection mode")]
    UnknownGsaSelectionMode(String),
    /// The GSA navigation mode is not recognized.
    #[diagnosis(description = "unknown NMEA GSA navigation mode")]
    UnknownGsaNavigationMode(String),

    /// A lower-level [`rax`] parsing rule failed.
    #[diagnosis(description = "RAX parser error")]
    RaxVerb(#[diagnosis(from)] rax::error::VerbError),
    /// A floating-point field could not be parsed.
    #[diagnosis(description = "floating-point parse error")]
    ParseFloat(#[diagnosis(from)] ParseFloatError),
    /// An integer field could not be parsed.
    #[diagnosis(description = "integer parse error")]
    ParseInt(#[diagnosis(from)] ParseIntError),
    /// An infallible conversion failed.
    #[diagnosis(description = "infallible conversion failed")]
    Infallible(#[diagnosis(from)] Infallible),
    /// A [`strum`] enum conversion failed.
    #[diagnosis(description = "enum parse error")]
    Strum(#[diagnosis(from)] strum::ParseError),
}

impl fmt::Display for RaxNmeaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSentence(value) => write!(f, "Invalid sentence: {value:?}"),
            Self::InvalidSentenceLength(value) => write!(f, "Invalid sentence length: {value}"),
            Self::InvalidSentencePrefix(value) => {
                write!(f, "Sentence doesn't start with `$`: {value}")
            }
            Self::InvalidChecksumLength(value) => {
                write!(f, "require checksum_str length 2, get {value}")
            }
            Self::MissingChecksumDelimiter(value) => {
                write!(f, "Missing checksum delimiter`*`: {value}")
            }
            Self::ChecksumMismatch {
                calculated,
                expected,
            } => write!(
                f,
                "Checksum mismatch: calculated {calculated:02X}, expected {expected:02X}"
            ),
            Self::UnknownIdentifier(value) => write!(f, "Unknown identifier: {value:?}"),
            Self::UnknownTalker(value) => write!(f, "Unknown talker: {value:?}"),
            Self::UnknownFaaMode(value) => write!(f, "Unknown Faa mode: {value:?}"),
            Self::UnknownSystemId(value) => write!(f, "Unknown system ID: {value:?}"),
            Self::UnknownStatus(value) => write!(f, "Unknown status: {value:?}"),
            Self::UnknownDtmDatum(value) => write!(f, "Unknown DTM datum: {value:?}"),
            Self::UnknownNavigationStatus(value) => {
                write!(f, "Unknown navigation status: {value:?}")
            }
            Self::UnknownGgaQualityIndicator(value) => {
                write!(f, "Unknown GGA quality indicator: {value:?}")
            }
            Self::UnknownGrsResidualMode(value) => {
                write!(f, "Unknown GRS residual mode: {value:?}")
            }
            Self::UnknownTxtType(value) => write!(f, "Unknown TXT type: {value}"),
            Self::UnknownGsaSelectionMode(value) => {
                write!(f, "Unknown GSA selection mode: {value:?}")
            }
            Self::UnknownGsaNavigationMode(value) => {
                write!(f, "Unknown GSA navigation mode: {value:?}")
            }
            Self::RaxVerb(source) => fmt::Display::fmt(source, f),
            Self::ParseFloat(source) => fmt::Display::fmt(source, f),
            Self::ParseInt(source) => fmt::Display::fmt(source, f),
            Self::Infallible(source) => fmt::Display::fmt(source, f),
            Self::Strum(source) => fmt::Display::fmt(source, f),
        }
    }
}

impl core::error::Error for RaxNmeaError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::RaxVerb(source) => core::error::Error::source(source),
            Self::ParseFloat(source) => core::error::Error::source(source),
            Self::ParseInt(source) => core::error::Error::source(source),
            Self::Infallible(source) => core::error::Error::source(source),
            Self::Strum(source) => core::error::Error::source(source),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn error_display_is_unchanged() {
        assert_eq!(
            RaxNmeaError::InvalidSentence(String::from("$GPGGA")).to_string(),
            "Invalid sentence: \"$GPGGA\""
        );
        assert_eq!(
            RaxNmeaError::ChecksumMismatch {
                calculated: 0x0A,
                expected: 0xBC,
            }
            .to_string(),
            "Checksum mismatch: calculated 0A, expected BC"
        );
        assert_eq!(
            RaxNmeaError::UnknownTxtType(2).to_string(),
            "Unknown TXT type: 2"
        );
    }

    #[test]
    fn transparent_variants_forward_display_and_source() {
        let parse = "not an integer".parse::<u8>().expect_err("parse must fail");
        let expected = parse.to_string();
        let error = RaxNmeaError::from(parse);

        assert_eq!(error.to_string(), expected);
        assert!(core::error::Error::source(&error).is_none());
    }
}
