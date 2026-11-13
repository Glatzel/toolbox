use alloc::ffi::NulError;
use core::error::Error;
use core::fmt;
use core::str::Utf8Error;

use mischief::IDiagnosis;

/// Error returned by Envoy pointer and string conversions.
#[derive(Debug, IDiagnosis)]
#[diagnosis(description = "The pointer was null.", code = "E_ENVOY_NULL_PTR")]
pub enum EnvoyError {
    /// The input pointer was null.
    NullPtr,
    /// The input bytes are not valid UTF-8.
    #[diagnosis(
        description = "The string is not valid UTF-8.",
        code = "E_ENVOY_INVALID_UTF8"
    )]
    InvalidUtf8(Utf8Error),
    /// The input string contains an interior NUL byte.
    #[diagnosis(
        description = "The string contains an interior NUL byte.",
        code = "E_ENVOY_NUL_ERROR"
    )]
    NulError(NulError),
}

impl fmt::Display for EnvoyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NullPtr => f.write_str("The pointer was null."),
            Self::InvalidUtf8(error) => error.fmt(f),
            Self::NulError(error) => error.fmt(f),
        }
    }
}

impl Error for EnvoyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NullPtr => None,
            Self::InvalidUtf8(error) => Some(error),
            Self::NulError(error) => Some(error),
        }
    }
}

impl From<Utf8Error> for EnvoyError {
    fn from(error: Utf8Error) -> Self { Self::InvalidUtf8(error) }
}

impl From<NulError> for EnvoyError {
    fn from(error: NulError) -> Self { Self::NulError(error) }
}

#[cfg(test)]
mod tests {
    use alloc::ffi::CString;
    use alloc::string::ToString;
    use core::error::Error;
    use core::str;

    use mischief::IDiagnosis;

    use super::EnvoyError;

    #[test]
    fn null_pointer_keeps_display_text_and_diagnostic_metadata() {
        let error = EnvoyError::NullPtr;

        assert_eq!(error.to_string(), "The pointer was null.");
        assert_eq!(IDiagnosis::description(&error), "The pointer was null.");
        assert_eq!(error.code(), Some("E_ENVOY_NULL_PTR"));
        assert!(IDiagnosis::source(&error).is_none());
        assert!(Error::source(&error).is_none());
    }

    #[test]
    #[allow(
        invalid_from_utf8,
        reason = "The test intentionally constructs invalid UTF-8."
    )]
    fn invalid_utf8_keeps_upstream_source_and_diagnostic_metadata() {
        let bytes = [0xff];
        let utf8 = str::from_utf8(&bytes).expect_err("the byte is not UTF-8");
        let error = EnvoyError::from(utf8);

        assert_eq!(error.to_string(), utf8.to_string());
        assert_eq!(
            IDiagnosis::description(&error),
            "The string is not valid UTF-8."
        );
        assert_eq!(error.code(), Some("E_ENVOY_INVALID_UTF8"));
        assert!(IDiagnosis::source(&error).is_none());
        assert!(Error::source(&error).is_some());
    }

    #[test]
    fn nul_error_keeps_upstream_source_and_diagnostic_metadata() {
        let nul = CString::new("a\0b").expect_err("the string contains a NUL byte");
        let message = nul.to_string();
        let error = EnvoyError::from(nul);

        assert_eq!(error.to_string(), message);
        assert_eq!(
            IDiagnosis::description(&error),
            "The string contains an interior NUL byte."
        );
        assert_eq!(error.code(), Some("E_ENVOY_NUL_ERROR"));
        assert!(IDiagnosis::source(&error).is_none());
        assert!(Error::source(&error).is_some());
    }
}
