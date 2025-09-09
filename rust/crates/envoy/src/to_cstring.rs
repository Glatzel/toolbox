use alloc::ffi::CString;
use alloc::string::String;

const CSTRING_NEW_EXCEPTION: &str = "Failed to create CString";

/// A helper trait for converting Rust string types into [`CString`],
/// which is safe to pass across FFI boundaries.
///
/// Unlike plain Rust strings, [`CString`] requires:
/// - No interior null bytes (`\0`) inside the string.
/// - A terminating null byte will be appended automatically.
///
/// This trait provides a convenient `to_cstring()` method for both `&str`
/// and [`String`] values.
pub trait ToCString {
    /// Converts the value into a [`CString`].
    ///
    /// # Panics
    ///
    /// Panics if the string contains an **interior null byte** (`\0`),
    /// since this would violate the C string contract.
    fn to_cstring(&self) -> CString;
}

impl ToCString for &str {
    /// Converts a string slice (`&str`) into a [`CString`].
    ///
    /// # Panics
    ///
    /// Panics if the slice contains an interior null byte.
    fn to_cstring(&self) -> CString { CString::new(*self).expect(CSTRING_NEW_EXCEPTION) }
}

impl ToCString for String {
    /// Converts an owned [`String`] into a [`CString`].
    ///
    /// # Panics
    ///
    /// Panics if the string contains an interior null byte.
    fn to_cstring(&self) -> CString { CString::new(self as &str).expect(CSTRING_NEW_EXCEPTION) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_cstring_str() {
        let s = "hello";
        let cstr = s.to_cstring();
        assert_eq!(cstr.to_str().unwrap(), "hello");
    }

    #[test]
    fn test_to_cstring_string() {
        let s = String::from("world");
        let cstr = s.to_cstring();
        assert_eq!(cstr.to_str().unwrap(), "world");
    }
}
