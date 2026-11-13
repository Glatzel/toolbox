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
    #[diagnosis(description = "The string is not valid UTF-8.")]
    InvalidUtf8(#[diagnosis(source(foreign), from)] Utf8Error),
    /// The input string contains an interior NUL byte.
    #[diagnosis(description = "The string contains an interior NUL byte.")]
    NulError(#[diagnosis(source(foreign), from)] NulError),
}
