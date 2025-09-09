use alloc::string::String;
use core::ffi::{CStr, c_char};

/// A trait for safely converting C string pointers (`*const c_char`, `*mut
/// c_char`) and C string slices (`[c_char]`) into Rust-owned [`String`]s.
///
/// This trait provides a fallible conversion that returns [`Option<String>`]:
/// - Returns `Some(String)` if the pointer/slice is valid and non-empty.
/// - Returns `None` if the pointer is null or the slice is empty.
///
/// # UTF-8 Handling
///
/// The underlying data is interpreted as a C-style **null-terminated string**.
/// If the bytes are not valid UTF-8, invalid sequences are replaced with the
/// Unicode replacement character (`U+FFFD`), using `Cstr::to_string_lossy``.
///
/// # Safety
///
/// - The caller must ensure that the provided pointer or slice is valid for
///   reads until the first `NUL` byte (`0u8`).
/// - Passing an invalid or dangling pointer is undefined behavior.
pub trait CStrToString {
    /// Converts the C string to a Rust [`String`].
    ///
    /// Returns:
    /// - `Some(String)` if conversion succeeds.
    /// - `None` if the pointer is null or the slice is empty.
    fn to_string(&self) -> Option<String>;
}

impl CStrToString for *const c_char {
    /// Implementation for immutable C string pointers (`*const c_char`).
    ///
    /// # Safety
    /// - The pointer must be valid and point to a null-terminated string.
    fn to_string(&self) -> Option<String> {
        if self.is_null() {
            return None;
        }
        Some(String::from(
            unsafe { CStr::from_ptr(*self) }.to_string_lossy(),
        ))
    }
}

impl CStrToString for *mut c_char {
    /// Implementation for mutable C string pointers (`*mut c_char`).
    ///
    /// # Safety
    /// - The pointer must be valid and point to a null-terminated string.
    /// - The string data will not be modified.
    fn to_string(&self) -> Option<String> {
        if self.is_null() {
            return None;
        }
        Some(String::from(
            unsafe { CStr::from_ptr(*self) }.to_string_lossy(),
        ))
    }
}

impl CStrToString for [c_char] {
    /// Implementation for raw C string slices (`[c_char]`).
    ///
    /// # Safety
    /// - The slice must contain a valid null-terminated string.
    /// - If the slice is empty, returns `None`.
    fn to_string(&self) -> Option<String> {
        if self.is_empty() {
            return None;
        }
        Some(String::from(
            unsafe { CStr::from_ptr(self.as_ptr()) }.to_string_lossy(),
        ))
    }
}
#[cfg(test)]
mod tests {
    use alloc::ffi::CString;
    use core::ptr;

    use super::*;
    #[test]
    fn test_cstr_to_string() {
        let s = CString::new("foo").unwrap();

        //*const i8
        {
            let ptr: *const c_char = s.as_ptr();
            assert_eq!(ptr.to_string().unwrap(), "foo");
        }
        //*mut i8
        {
            let ptr: *mut c_char = s.as_ptr().cast_mut();
            assert_eq!(ptr.to_string().unwrap(), "foo");
        }
        //null
        {
            let ptr: *const c_char = ptr::null();
            assert!(ptr.to_string().is_none());
            let ptr: *mut c_char = ptr.cast_mut();
            assert!(ptr.to_string().is_none());
            let ptr: [c_char; 0] = [];
            assert!(ptr.to_string().is_none());
        }
    }
}
