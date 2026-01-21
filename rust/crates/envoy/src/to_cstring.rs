use alloc::ffi::CString;
use alloc::string::String;
use core::str::FromStr;

use crate::EnvoyError;

/// Converts Rust string types into a [`CString`] suitable for FFI usage.
///
/// This trait provides a small convenience abstraction over
/// [`CString::from_str`] for types commonly used in Rust APIs.
///
/// # NUL Byte Handling
///
/// C strings cannot contain interior NUL (`'\0'`) bytes. If the source
/// string contains a NUL byte, conversion fails with [`EnvoyError`].
///
/// # Typical Use Case
///
/// This trait is intended for FFI boundaries where Rust strings must be
/// passed to C APIs expecting `char *` or `const char *`.
pub trait ToCString {
    /// Converts the value into a [`CString`].
    ///
    /// # Errors
    ///
    /// Returns [`EnvoyError`] if the string contains an interior NUL byte.
    fn to_cstring(&self) -> Result<CString, EnvoyError>;
}

/// Implementation for string slices.
impl ToCString for &str {
    fn to_cstring(&self) -> Result<CString, EnvoyError> { Ok(CString::from_str(self)?) }
}

/// Implementation for owned `String` values.
impl ToCString for String {
    fn to_cstring(&self) -> Result<CString, EnvoyError> { Ok(CString::from_str(self)?) }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_cstring_str() -> mischief::Result<()> {
        let s = "hello";
        let cstr = s.to_cstring()?;
        assert_eq!(cstr.to_str().unwrap(), "hello");
        Ok(())
    }

    #[test]
    fn test_to_cstring_string() -> mischief::Result<()> {
        let s = String::from("world");
        let cstr = s.to_cstring()?;
        assert_eq!(cstr.to_str().unwrap(), "world");
        Ok(())
    }
}
