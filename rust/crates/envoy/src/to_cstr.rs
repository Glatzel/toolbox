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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::CStrToString;

    #[test]
    fn test_to_cstr() {
        //&str
        {
            let ptr: *const i8 = "foo".to_cstr();
            assert_eq!(ptr.to_string().unwrap(), "foo");
            assert!(!ptr.is_null());
            // SAFETY: ptr was allocated by CString::into_raw, so we must reclaim it
            unsafe {
                let _ = CString::from_raw(ptr as *mut i8);
            }
        }
        //String
        {
            let s = String::from("foo");
            let ptr: *const i8 = s.to_cstr();
            assert_eq!(ptr.to_string().unwrap(), "foo");
            assert!(!ptr.is_null());
            unsafe {
                let _ = CString::from_raw(ptr as *mut i8);
            }
        }
        //Option<&str>
        {
            let ptr: *const i8 = Some("foo").to_cstr();
            assert!(!ptr.is_null());
            assert_eq!(ptr.to_string().unwrap(), "foo");
            unsafe {
                let _ = CString::from_raw(ptr as *mut i8);
            }
        }
        //Option<String>
        {
            let s = String::from("foo");
            let ptr: *const i8 = Some(s).to_cstr();
            assert!(!ptr.is_null());
            assert_eq!(ptr.to_string().unwrap(), "foo");
            unsafe {
                let _ = CString::from_raw(ptr as *mut i8);
            }
        }
        //None
        {
            assert_eq!(Option::<&str>::None.to_cstr(), ptr::null());
            assert_eq!(Option::<String>::None.to_cstr(), ptr::null());
        }
    }
}
