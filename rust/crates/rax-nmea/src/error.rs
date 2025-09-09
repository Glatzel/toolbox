use core::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum RaxNmeaError {
    #[error("Invalid sentence: {0}")]
    InvalidSentence(String),
    #[error("Sentence doesn't start with `$`: {0}")]
    InvalidSentencePrefix(String),
    #[error("require checksum_str length 2, get {0}")]
    InvalidChecksumLength(usize),
    #[error("Invalid hex checksum")]
    InvalidHexChecksum(#[from] ParseIntError),
    #[error("Missing checksum delimiter`*`: {0}")]
    MissingChecksumDelimiter(String),
    #[error("Checksum mismatch: calculated {calculated:02X}, expected {expected:02X}")]
    ChecksumMismatch { calculated: u8, expected: u8 },

    #[error("Unknown identifier: {0}")]
    UnknownIdentifier(String),
    #[error("Unknown talker: {0}")]
    UnknownTalker(String),
    #[error("Unknown Faa mode: {0}")]
    UnknownFaaMode(String),
    #[error("Unknown system ID: {0}")]
    UnknownSystemId(String),
    #[error("Unknown status: {0}")]
    UnknownStatus(String),
    #[error("Unknown DTM datum: {0}")]
    UnknownDtmDatum(String),
    #[error("Unknown navigation status: {0}")]
    UnknownNavigationStatus(String),
    #[error("Unknown GGA quality indicator: {0}")]
    UnknownGgaQualityIndicator(String),
    #[error("Unknown GRS residual mode: {0}")]
    UnknownGrsResidualMode(String),
    #[error("Unknown TXT type: {0}")]
    UnknownTxtType(u8),
    #[error("Unknown GSA selection mode: {0}")]
    UnknownGsaSelectionMode(String),
    #[error("Unknown GSA navigation mode: {0}")]
    UnknownGsaNavigationMode(String),

    #[error("RaxError")]
    RaxError(#[from] rax::RaxError),
}
