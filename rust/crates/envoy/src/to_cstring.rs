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
