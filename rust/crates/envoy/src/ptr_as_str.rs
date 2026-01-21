use core::ffi::{CStr, c_char};
use core::str::Utf8Error;

/// Convert a C-style string pointer or buffer into a Rust `&str`.
///
/// # Purpose
///
/// This trait provides a lightweight, zero-copy view (`&str`) over
/// a NUL-terminated C string (`char *`) originating from FFI.
///
/// It is intended for **borrowed** C strings whose lifetime is managed
/// by the caller (typically C code).
///
/// # Safety and assumptions
///
/// All implementations assume:
///
/// * The underlying memory is **valid and NUL-terminated**
/// * The memory remains alive for the duration of the returned `&str`
/// * The contents are valid UTF-8
///
/// Violating any of these conditions results in **undefined behavior**
/// (due to `CStr::from_ptr`) or a returned `Utf8Error`.
///
/// # Implementations
///
/// Implemented for:
///
/// * `*const c_char`
/// * `*mut c_char`
/// * `[c_char]` (interpreted as a NUL-terminated buffer)
///
/// # Errors
///
/// Returns [`Utf8Error`] if the C string contains invalid UTF-8.
pub trait PtrAsStr {
    /// Interpret the value as a NUL-terminated C string and return it as
    /// `&str`.
    ///
    /// # Errors
    ///
    /// Returns [`Utf8Error`] if the underlying C string is not valid UTF-8.
    fn as_str(&self) -> Result<&str, Utf8Error>;
}

impl PtrAsStr for *const c_char {
    fn as_str(&self) -> Result<&str, Utf8Error> { unsafe { CStr::from_ptr(*self).to_str() } }
}

impl PtrAsStr for *mut c_char {
    fn as_str(&self) -> Result<&str, Utf8Error> { unsafe { CStr::from_ptr(*self).to_str() } }
}

impl PtrAsStr for [c_char] {
    fn as_str(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.as_ptr()).to_str() }
    }
}
#[cfg(test)]
mod tests {
    use alloc::ffi::CString;

    use super::*;

    #[test]
    fn const_ptr_as_str_ok() {
        let c = CString::new("hello").unwrap();
        let p: *const c_char = c.as_ptr();

        let s = p.as_str().unwrap();
        assert_eq!(s, "hello");
    }

    #[test]
    fn mut_ptr_as_str_ok() {
        let c = CString::new("world").unwrap();
        let p: *mut c_char = c.as_ptr().cast_mut();

        let s = p.as_str().unwrap();
        assert_eq!(s, "world");
    }

    #[test]
    fn slice_as_str_ok() {
        let c = CString::new("slice").unwrap();
        let bytes = c.as_bytes_with_nul();

        // Convert &[u8] → &[c_char]
        let slice: &[c_char] =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const c_char, bytes.len()) };

        let s = slice.as_str().unwrap();
        assert_eq!(s, "slice");
    }

    #[test]
    fn invalid_utf8_returns_error() {
        // 0xFF is invalid UTF-8
        let bytes = [0xff_u8, 0x00];
        let ptr = bytes.as_ptr() as *const c_char;

        let err = ptr.as_str().unwrap_err();
        let _ = err; // just ensure error is returned
    }
}
