use alloc::string::{String, ToString};
use core::ffi::c_char;

use crate::{EnvoyError, PtrAsStr};

/// Extension trait for converting C-style string pointers or buffers
/// into an owned Rust [`String`].
///
/// # Semantics
///
/// - For raw pointers:
///   - Returns an error if the pointer is null
///   - Returns an error if the C string is not valid UTF-8
/// - For slices:
///   - Returns an error if the C string is not valid UTF-8
///
/// On success, a new [`String`] is allocated and populated by copying
/// the contents of the C string.
///
/// # Ownership
///
/// This trait does **not** take ownership of the underlying C string.
/// The returned [`String`] is fully owned and independent of the source
/// memory.
///
/// # Relation to `PtrAsStr`
///
/// This trait is a convenience layer built on top of [`PtrAsStr`].
/// It performs UTF-8 validation and allocation, and therefore should
/// be preferred when the returned value must outlive the source buffer.
///
/// # Errors
///
/// The exact error variants depend on [`EnvoyError`], but typically include:
///
/// - Null pointer
/// - Invalid UTF-8 sequence
///
/// [`PtrAsStr`]: crate::PtrAsStr
pub trait PtrToString {
    /// Convert the underlying C string representation into an owned
    /// Rust [`String`].
    ///
    /// # Errors
    ///
    /// Returns an error if the pointer is null (for raw pointers) or if
    /// UTF-8 validation fails.
    fn to_string(&self) -> Result<String, EnvoyError>;
}

impl PtrToString for *const c_char {
    fn to_string(&self) -> Result<String, EnvoyError> { (*self).as_str().map(ToString::to_string) }
}

impl PtrToString for *mut c_char {
    fn to_string(&self) -> Result<String, EnvoyError> { (*self).as_str().map(ToString::to_string) }
}

impl PtrToString for [c_char] {
    fn to_string(&self) -> Result<String, EnvoyError> { (*self).as_str().map(ToString::to_string) }
}
#[cfg(test)]
mod tests {

    use alloc::ffi::CString;
    use core::ptr;

    use super::*;

    #[test]
    fn const_ptr_to_string() -> mischief::Result<()> {
        let s = CString::new("hello").unwrap();
        let ptr = s.as_ptr();

        let out = ptr.to_string()?;
        assert_eq!(out, "hello");
        Ok(())
    }

    #[test]
    fn mut_ptr_to_string() -> mischief::Result<()> {
        let s = CString::new("world").unwrap();
        let ptr = s.as_ptr().cast_mut();

        let out = ptr.to_string()?;
        assert_eq!(out, "world");
        Ok(())
    }

    #[test]
    fn null_ptr_returns_none() {
        let ptr: *const c_char = ptr::null();
        assert!(ptr.to_string().is_err());
    }

    #[test]
    fn invalid_utf8_returns_none() {
        let bytes = [0xFFu8 as c_char, 0];
        let ptr = bytes.as_ptr() as *const c_char;
        assert!(ptr.to_string().is_err());
    }

    #[test]
    fn slice_c_char_to_string() -> mischief::Result<()> {
        let bytes = b"slice-owned\0";
        let slice: &[c_char] =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const c_char, bytes.len()) };

        let out = slice.to_string()?;
        assert_eq!(out, "slice-owned");
        Ok(())
    }

    #[test]
    fn returned_string_is_owned() {
        let s = CString::new("owned").unwrap();
        let ptr = s.as_ptr();

        let out = ptr.to_string().unwrap();
        drop(s);

        // The String must remain valid after the source is dropped
        assert_eq!(out, "owned");
    }
}
