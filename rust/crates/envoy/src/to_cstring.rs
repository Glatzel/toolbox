use std::ffi::CString;

const CSTRING_NEW_EXCEPTION: &str = "Failed to create CString";

/// Provides conversion from Rust string types to `CString`.
///
/// # Examples
///
/// ```
/// use envoy::ToCString;
/// let cstr = "hello".to_cstring();
/// assert_eq!(cstr.to_str().unwrap(), "hello");
/// ```
pub trait ToCString {
    /// Converts the value to a `CString`.
    ///
    /// # Panics
    ///
    /// Panics if the string contains an interior null byte.
    fn to_cstring(&self) -> CString;
}

impl ToCString for &str {
    /// Converts a `&str` to a `CString`.
    ///
    /// # Panics
    ///
    /// Panics if the string contains an interior null byte.
    fn to_cstring(&self) -> CString { CString::new(*self).expect(CSTRING_NEW_EXCEPTION) }
}

impl ToCString for String {
    /// Converts a `String` to a `CString`.
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
