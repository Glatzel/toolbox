use std::ffi::CString;

const CSTRING_NEW_EXCEPTION: &str = "Failed to create CString";

pub trait ToCString {
    fn to_cstring(&self) -> CString;
}

impl ToCString for &str {
    fn to_cstring(&self) -> CString { CString::new(*self).expect(CSTRING_NEW_EXCEPTION) }
}

impl ToCString for String {
    fn to_cstring(&self) -> CString { CString::new(self as &str).expect(CSTRING_NEW_EXCEPTION) }
}

impl ToCString for Option<&str> {
    fn to_cstring(&self) -> CString {
        CString::new(self.unwrap_or_default()).expect(CSTRING_NEW_EXCEPTION)
    }
}
impl ToCString for Option<String> {
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
