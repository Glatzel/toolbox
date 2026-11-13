extern crate alloc;

use alloc::borrow::Cow;
use core::convert::Infallible;
use core::num::{ParseFloatError, ParseIntError};

use mischief_api::IDiagnosis;

/// Errors produced while parsing or validating an NMEA sentence.
#[derive(Debug, IDiagnosis)]
#[diagnosis(error, display)]
pub enum RaxNmeaError {
    /// The sentence is structurally invalid.
    #[diagnosis(description = "Invalid sentence: {0:?}")]
    InvalidSentence(Cow<'static, str>),
    /// The sentence is shorter than the minimum length required by a parser.
    #[diagnosis(description = "Invalid sentence length: {0}")]
    InvalidSentenceLength(usize),
    /// The sentence does not start with the `$` delimiter.
    #[diagnosis(description = "Sentence doesn't start with `$`: {0}")]
    InvalidSentencePrefix(Cow<'static, str>),
    /// The checksum field is not exactly two hexadecimal characters.
    #[diagnosis(description = "require checksum_str length 2, get {0}")]
    InvalidChecksumLength(usize),
    /// The `*` checksum delimiter is missing.
    #[diagnosis(description = "Missing checksum delimiter`*`: {0}")]
    MissingChecksumDelimiter(Cow<'static, str>),
    /// The calculated checksum does not match the checksum encoded in the
    /// sentence.
    #[diagnosis(
        description = "Checksum mismatch: calculated {calculated:02X}, expected {expected:02X}"
    )]
    ChecksumMismatch {
        /// Checksum calculated from the sentence payload.
        calculated: u8,
        /// Checksum read from the sentence payload.
        expected: u8,
    },

    /// The sentence identifier is not recognized.
    #[diagnosis(description = "Unknown identifier: {0:?}")]
    UnknownIdentifier(Cow<'static, str>),
    /// The talker ID is not recognized.
    #[diagnosis(description = "Unknown talker: {0:?}")]
    UnknownTalker(Cow<'static, str>),
    /// The FAA mode indicator is not recognized.
    #[diagnosis(description = "Unknown Faa mode: {0:?}")]
    UnknownFaaMode(Cow<'static, str>),
    /// The GNSS system identifier is not recognized.
    #[diagnosis(description = "Unknown system ID: {0:?}")]
    UnknownSystemId(Cow<'static, str>),
    /// The status field is not recognized.
    #[diagnosis(description = "Unknown status: {0:?}")]
    UnknownStatus(Cow<'static, str>),
    /// The DTM datum field is not recognized.
    #[diagnosis(description = "Unknown DTM datum: {0:?}")]
    UnknownDtmDatum(Cow<'static, str>),
    /// The navigation status field is not recognized.
    #[diagnosis(description = "Unknown navigation status: {0:?}")]
    UnknownNavigationStatus(Cow<'static, str>),
    /// The GGA quality indicator is not recognized.
    #[diagnosis(description = "Unknown GGA quality indicator: {0:?}")]
    UnknownGgaQualityIndicator(Cow<'static, str>),
    /// The GRS residual mode is not recognized.
    #[diagnosis(description = "Unknown GRS residual mode: {0:?}")]
    UnknownGrsResidualMode(Cow<'static, str>),
    /// The TXT message type is not recognized.
    #[diagnosis(description = "Unknown TXT type: {0}")]
    UnknownTxtType(u8),
    /// The GSA selection mode is not recognized.
    #[diagnosis(description = "Unknown GSA selection mode: {0:?}")]
    UnknownGsaSelectionMode(Cow<'static, str>),
    /// The GSA navigation mode is not recognized.
    #[diagnosis(description = "Unknown GSA navigation mode: {0:?}")]
    UnknownGsaNavigationMode(Cow<'static, str>),

    /// A lower-level [`rax`] parsing rule failed.
    #[diagnosis(transparent(foreign))]
    RaxVerb(#[diagnosis(from)] rax::error::VerbError),
    /// A floating-point field could not be parsed.
    #[diagnosis(transparent(foreign))]
    ParseFloat(#[diagnosis(from)] ParseFloatError),
    /// An integer field could not be parsed.
    #[diagnosis(transparent(foreign))]
    ParseInt(#[diagnosis(from)] ParseIntError),
    /// An infallible conversion failed.
    #[diagnosis(transparent(foreign))]
    Infallible(#[diagnosis(from)] Infallible),
    /// A [`strum`] enum conversion failed.
    #[diagnosis(transparent(foreign))]
    Strum(#[diagnosis(from)] strum::ParseError),
}
