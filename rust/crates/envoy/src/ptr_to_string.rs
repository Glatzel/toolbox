use alloc::string::{String, ToString};
use core::ffi::{CStr, c_char};

use crate::{EnvoyError, PtrAsStr};

/// Convert C-style string pointers or buffers into owned Rust `String`.
///
/// This trait provides two conversion modes:
///
/// - [`to_string`](PtrToString::to_string)
///   - Requires valid UTF-8
///   - Returns an error if encoding is invalid
///
/// - [`to_string_lossy`](PtrToString::to_string_lossy)
///   - Replaces invalid UTF-8 with U+FFFD replacement characters
///   - Never fails due to encoding errors
///
/// ## Safety expectations
///
/// Implementations assume:
///
/// - Pointer refers to valid readable memory
/// - Data is **NUL-terminated**
/// - Lifetime of the pointed memory outlives the conversion
///
/// Violating these requirements is undefined behavior.
pub trait PtrToString {
    /// Convert into `String`, requiring valid UTF-8.
    fn to_string(&self) -> Result<String, EnvoyError>;

    /// Convert into `String`, replacing invalid UTF-8 sequences.
    fn to_string_lossy(&self) -> Result<String, EnvoyError>;
}

impl PtrToString for *const c_char {
    /// # Examples
    ///
    /// ```
    /// use std::ffi::CString;
    ///
    /// use envoy::PtrToString;
    ///
    /// let s = CString::new("hello").unwrap();
    /// let ptr = s.as_ptr();
    ///
    /// assert_eq!(ptr.to_string().unwrap(), "hello");
    /// ```
    fn to_string(&self) -> Result<String, EnvoyError> { Ok(self.as_str()?.to_string()) }
    /// # Examples
    ///
    /// ```
    /// use core::ffi::c_char;
    ///
    /// use envoy::PtrToString;
    ///
    /// let bytes = [0xFFu8 as c_char, 0];
    /// let ptr = bytes.as_ptr();
    ///
    /// let s = ptr.to_string_lossy().unwrap();
    /// assert!(s.contains('\u{FFFD}'));
    /// ```
    fn to_string_lossy(&self) -> Result<String, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }
        unsafe { Ok(CStr::from_ptr(*self).to_string_lossy().to_string()) }
    }
}

impl PtrToString for *mut c_char {
    /// # Examples
    ///
    /// ```
    /// use std::ffi::{CString, c_char};
    ///
    /// use envoy::PtrToString;
    ///
    /// let s = CString::new("world").unwrap();
    /// let ptr = s.as_ptr() as *mut c_char;
    ///
    /// assert_eq!(ptr.to_string().unwrap(), "world");
    /// ```
    fn to_string(&self) -> Result<String, EnvoyError> { Ok(self.as_str()?.to_string()) }
    /// # Examples
    ///
    /// ```
    /// use core::ffi::c_char;
    ///
    /// use envoy::PtrToString;
    ///
    /// let bytes = [0xFFu8 as c_char, 0];
    /// let ptr = bytes.as_ptr() as *mut c_char;
    ///
    /// let s = ptr.to_string_lossy().unwrap();
    /// assert!(s.contains('\u{FFFD}'));
    /// ```
    fn to_string_lossy(&self) -> Result<String, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }
        unsafe { Ok(CStr::from_ptr(*self).to_string_lossy().to_string()) }
    }
}

impl PtrToString for [c_char] {
    /// # Examples
    ///
    /// ```
    /// use core::ffi::c_char;
    ///
    /// use envoy::PtrToString;
    ///
    /// let bytes = b"slice\0";
    /// let slice =
    ///     unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const c_char, bytes.len()) };
    ///
    /// assert_eq!(slice.to_string().unwrap(), "slice");
    /// ```
    fn to_string(&self) -> Result<String, EnvoyError> { Ok(self.as_str()?.to_string()) }
    /// # Examples
    ///
    /// ```
    /// use core::ffi::c_char;
    ///
    /// use envoy::PtrToString;
    ///
    /// let bytes = [0xFFu8 as c_char, 0];
    /// let slice = &bytes[..];
    ///
    /// let s = slice.to_string_lossy().unwrap();
    /// assert!(s.contains('\u{FFFD}'));
    /// ```
    fn to_string_lossy(&self) -> Result<String, EnvoyError> {
        unsafe { Ok(CStr::from_ptr(self.as_ptr()).to_string_lossy().to_string()) }
    }
}
#[cfg(test)]
mod tests {
    extern crate std;
    use core::ptr;
    use std::ffi::CString;

    use super::*;

    #[test]
    fn const_ptr_valid_utf8() {
        let c = CString::new("hello").unwrap();
        let p = c.as_ptr();

        assert_eq!(p.to_string().unwrap(), "hello");
        assert_eq!(p.to_string_lossy().unwrap(), "hello");
    }

    #[test]
    fn mut_ptr_valid_utf8() {
        let c = CString::new("world").unwrap();
        let p = c.as_ptr().cast_mut();

        assert_eq!(p.to_string().unwrap(), "world");
        assert_eq!(p.to_string_lossy().unwrap(), "world");
    }

    #[test]
    fn const_ptr_null() {
        let p: *const c_char = ptr::null();

        assert!(p.to_string().is_err());
        assert!(p.to_string_lossy().is_err());
    }

    #[test]
    fn mut_ptr_null() {
        let p: *mut c_char = ptr::null_mut();

        assert!(p.to_string().is_err());
        assert!(p.to_string_lossy().is_err());
    }

    #[test]
    fn lossy_invalid_utf8() {
        // 0xFF is invalid UTF-8 when alone
        let bytes = [0xFFu8, 0x00];
        let ptr = bytes.as_ptr() as *const c_char;

        let s = ptr.to_string_lossy().unwrap();
        assert!(s.contains('\u{FFFD}'));
    }

    #[test]
    fn strict_invalid_utf8_should_fail() {
        let bytes = [0xFFu8, 0x00];
        let ptr = bytes.as_ptr() as *const c_char;

        assert!(ptr.to_string().is_err());
    }

    #[test]
    fn slice_valid_utf8() {
        let bytes = *b"slice\0";
        let slice =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const c_char, bytes.len()) };

        assert_eq!(slice.to_string().unwrap(), "slice");
        assert_eq!(slice.to_string_lossy().unwrap(), "slice");
    }
}
