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

impl ToCString for Option<&str> {
    /// Converts an `Option<&str>` to a `CString`.
    ///
    /// Returns an empty `CString` if `None`.
    ///
    /// # Panics
    ///
    /// Panics if the string contains an interior null byte.
    fn to_cstring(&self) -> CString {
        CString::new(self.unwrap_or_default()).expect(CSTRING_NEW_EXCEPTION)
    }
}
impl ToCString for Option<String> {
    /// Converts an `Option<String>` to a `CString`.
    ///
    /// Returns an empty `CString` if `None`.
    ///
    /// # Panics
    ///
    /// Panics if the string contains an interior null byte.
    fn to_cstring(&self) -> CString {
        match self {
            Some(s) => CString::new(s.to_owned()).expect(CSTRING_NEW_EXCEPTION),
            None => CString::default(),
        }
    }
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

    #[test]
    fn test_to_cstring_option_str_some() {
        let s = Some("foo");
        let cstr = s.to_cstring();
        assert_eq!(cstr.to_str().unwrap(), "foo");
    }

    #[test]
    fn test_to_cstring_option_str_none() {
        let s: Option<&str> = None;
        let cstr = s.to_cstring();
        assert_eq!(cstr.to_str().unwrap(), "");
    }

    #[test]
    fn test_to_cstring_option_string_some() {
        let s = Some(String::from("bar"));
        let cstr = s.to_cstring();
        assert_eq!(cstr.to_str().unwrap(), "bar");
    }

    #[test]
    fn test_to_cstring_option_string_none() {
        let s: Option<String> = None;
        let cstr = s.to_cstring();
        assert_eq!(cstr.to_str().unwrap(), "");
    }
}
