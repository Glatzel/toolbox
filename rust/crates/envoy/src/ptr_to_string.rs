use alloc::string::{String, ToString};
use core::ffi::c_char;

use crate::PtrAsStr;

/// Extension trait for converting C-style string pointers or buffers
/// into an owned Rust [`String`].
///
/// # Semantics
///
/// - Returns `None` if the pointer is null (for raw pointers)
/// - Returns `None` if the C string is not valid UTF-8
/// - Allocates a new [`String`] and copies the contents
///
/// # Ownership
///
/// This trait **does not take ownership** of the underlying C string.
/// The returned [`String`] is fully owned and independent of the source.
///
/// # Relation to [`PtrAsStr`]
///
/// This trait is a convenience wrapper built on top of [`PtrAsStr`],
/// performing UTF-8 validation and allocation.
///
/// [`PtrAsStr`]: crate::PtrAsStr
pub trait PtrToString {
    /// Convert the underlying C string representation into an owned [`String`].
    ///
    /// Returns `None` if the pointer is null or if UTF-8 validation fails.
    fn to_string(&self) -> Option<String>;
}

impl PtrToString for *const c_char {
    fn to_string(&self) -> Option<String> { (*self).as_str().map(ToString::to_string) }
}

impl PtrToString for *mut c_char {
    fn to_string(&self) -> Option<String> { (*self).as_str().map(ToString::to_string) }
}

impl PtrToString for [c_char] {
    fn to_string(&self) -> Option<String> { (*self).as_str().map(ToString::to_string) }
}
#[cfg(test)]
mod tests {

    use alloc::ffi::CString;
    use core::ptr;

    use super::*;

    #[test]
    fn const_ptr_to_string() {
        let s = CString::new("hello").unwrap();
        let ptr = s.as_ptr();

        let out = ptr.to_string();
        assert_eq!(out.as_deref(), Some("hello"));
    }

    #[test]
    fn mut_ptr_to_string() {
        let s = CString::new("world").unwrap();
        let ptr = s.as_ptr().cast_mut();

        let out = ptr.to_string();
        assert_eq!(out.as_deref(), Some("world"));
    }

    #[test]
    fn null_ptr_returns_none() {
        let ptr: *const c_char = ptr::null();
        assert_eq!(ptr.to_string(), None);
    }

    #[test]
    fn invalid_utf8_returns_none() {
        let bytes = [0xFFu8 as c_char, 0];
        let ptr = bytes.as_ptr() as *const c_char;

        assert_eq!(ptr.to_string(), None);
    }

    #[test]
    fn slice_c_char_to_string() {
        let bytes = b"slice-owned\0";
        let slice: &[c_char] =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const c_char, bytes.len()) };

        let out = slice.to_string();
        assert_eq!(out.as_deref(), Some("slice-owned"));
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
