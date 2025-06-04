use std::ffi::CString;
use std::ptr;

const CSTRING_NEW_EXCEPTION: &str = "Failed to create CString";

pub trait ToCStr {
    fn to_cstring(&self) -> CString;
    fn to_cstr(&self) -> *const i8;
}

impl ToCStr for &str {
    fn to_cstring(&self) -> CString { CString::new(*self).expect(CSTRING_NEW_EXCEPTION) }
    fn to_cstr(&self) -> *const i8 { self.to_cstring().into_raw() }
}

impl ToCStr for String {
    fn to_cstring(&self) -> CString { CString::new(self as &str).expect(CSTRING_NEW_EXCEPTION) }
    fn to_cstr(&self) -> *const i8 { self.to_cstring().into_raw() }
}

impl ToCStr for Option<&str> {
    fn to_cstring(&self) -> CString {
        CString::new(self.unwrap_or_default()).expect(CSTRING_NEW_EXCEPTION)
    }
    fn to_cstr(&self) -> *const i8 {
        match self {
            Some(_) => self.to_cstring().into_raw(),
            None => ptr::null(),
        }
    }
}
impl ToCStr for Option<String> {
    fn to_cstring(&self) -> CString {
        match self {
            Some(s) => CString::new(s.to_owned()).expect(CSTRING_NEW_EXCEPTION),
            None => CString::default(),
        }
    }
    fn to_cstr(&self) -> *const i8 {
        match self {
            Some(_) => self.to_cstring().into_raw(),
            None => ptr::null(),
        }
    }
}
