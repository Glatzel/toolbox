use alloc::ffi::NulError;
use core::str::Utf8Error;

use mischief::IDiagnosis;

/// Error returned by Envoy pointer and string conversions.
#[derive(Debug, IDiagnosis)]
#[diagnosis(display, error)]
pub enum EnvoyError {
    /// The input pointer was null.
    #[diagnosis(description = "The pointer was null.")]
    NullPtr,
    /// The input bytes are not valid UTF-8.
    #[diagnosis(transparent)]
    InvalidUtf8(#[diagnosis(from)] Utf8Error),
    /// The input string contains an interior NUL byte.
    #[diagnosis(transparent)]
    NulError(#[diagnosis(from)] NulError),
}
