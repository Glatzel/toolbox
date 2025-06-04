use std::ffi::{CStr, CString};
use std::ptr;
const CSTRING_NEW_EXCEPTION: &str = "Failed to create CString";
/// Trait for converting C string pointers and slices to Rust `String`.
pub trait CStrToString {
    /// Converts the C string to a Rust `String`.
    /// Returns `None` if the pointer is null.
    fn to_string(&self) -> Option<String>;
}

impl CStrToString for *const i8 {
    /// Converts a raw C string pointer to a Rust `String`.
    /// Returns `None` if the pointer is null.
    fn to_string(&self) -> Option<String> {
        if self.is_null() {
            return None;
        }
        Some(
            unsafe { CStr::from_ptr(*self) }
                .to_string_lossy()
                .to_string(),
        )
    }
}
impl CStrToString for *mut i8 {
    /// Converts a slice of C string bytes to a Rust `String`.
    fn to_string(&self) -> Option<String> {
        if self.is_null() {
            return None;
        }
        Some(
            unsafe { CStr::from_ptr(*self) }
                .to_string_lossy()
                .to_string(),
        )
    }
}
impl CStrToString for [i8] {
    /// Converts a slice of C string bytes to a Rust `String`.
    fn to_string(&self) -> Option<String> {
        if self.is_empty() {
            return None;
        }
        Some(
            unsafe { CStr::from_ptr(self.as_ptr()) }
                .to_string_lossy()
                .to_string(),
        )
    }
}

pub trait CStrListToVecString {
    fn to_vec_string(&self) -> Vec<String>;
}

impl CStrListToVecString for *mut *mut i8 {
    fn to_vec_string(&self) -> Vec<String> {
        if self.is_null() {
            return Vec::new();
        }
        let mut vec_str = Vec::new();
        let mut offset = 0;

        loop {
            let current_ptr = unsafe { self.offset(offset).as_ref().unwrap() };
            if current_ptr.is_null() {
                break;
            }
            vec_str.push(current_ptr.cast_const().to_string().unwrap());
            offset += 1;
        }
        vec_str
    }
}
impl CStrListToVecString for *const *const i8 {
    fn to_vec_string(&self) -> Vec<String> {
        if self.is_null() {
            return Vec::new();
        }
        let mut vec_str = Vec::new();
        let mut offset = 0;

        loop {
            let current_ptr = unsafe { self.offset(offset).as_ref().unwrap() };
            if current_ptr.is_null() {
                break;
            }
            vec_str.push(current_ptr.to_string().unwrap());
            offset += 1;
        }
        vec_str
    }
}

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
pub trait ToCStrList {
    fn to_cstring_list(&self) -> Vec<CString>;
    fn to_cstr_list(&self) -> Vec<*const i8>;
}

impl ToCStrList for [&str] {
    fn to_cstring_list(&self) -> Vec<CString> {
        if self.is_empty() {
            return Vec::new();
        }
        self.iter().map(|s| s.to_cstring()).collect()
    }
    /// Returns a Vec of pointers, caller must ensure the Vec lives long enough.
    fn to_cstr_list(&self) -> Vec<*const i8> {
        self.iter().map(|s| s.to_cstr()).collect::<Vec<*const i8>>()
    }
}

impl ToCStrList for Vec<String> {
    fn to_cstring_list(&self) -> Vec<CString> {
        if self.is_empty() {
            return Vec::new();
        }
        self.iter().map(|s| s.to_cstring()).collect()
    }
    /// Returns a Vec of pointers, caller must ensure the Vec lives long enough.
    fn to_cstr_list(&self) -> Vec<*const i8> {
        self.iter().map(|s| s.to_cstr()).collect::<Vec<*const i8>>()
    }
}

impl ToCStrList for Option<Vec<&str>> {
    fn to_cstring_list(&self) -> Vec<CString> {
        match self {
            Some(s) => {
                if s.is_empty() {
                    return Vec::new();
                }
                s.iter().map(|l| l.to_cstring()).collect()
            }
            None => Vec::new(),
        }
    }
    /// Returns a Vec of pointers, caller must ensure the Vec lives long enough.
    fn to_cstr_list(&self) -> Vec<*const i8> {
        match self {
            Some(s) => {
                if s.is_empty() {
                    return Vec::new();
                }
                s.iter().map(|l| l.to_cstr()).collect()
            }
            None => Vec::new(),
        }
    }
}
impl ToCStrList for Option<Vec<String>> {
    fn to_cstring_list(&self) -> Vec<CString> {
        match self {
            Some(s) => {
                if s.is_empty() {
                    return Vec::new();
                }
                s.iter().map(|l| l.to_cstring()).collect()
            }
            None => Vec::new(),
        }
    }
    /// Returns a Vec of pointers, caller must ensure the Vec lives long enough.
    fn to_cstr_list(&self) -> Vec<*const i8> {
        match self {
            Some(s) => {
                if s.is_empty() {
                    return Vec::new();
                }
                s.iter().map(|l| l.to_cstr()).collect()
            }
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cstr_to_string() {
        let s = CString::new("foo").unwrap();

        //*const i8
        {
            let ptr: *const i8 = s.as_ptr();
            assert_eq!(ptr.to_string().unwrap(), "foo");
        }
        //*mut i8
        {
            let ptr: *mut i8 = s.as_ptr().cast_mut();
            assert_eq!(ptr.to_string().unwrap(), "foo");
        }
        //[i8]
        {
            let bytes: &[u8] = s.as_bytes_with_nul();
            let mut arr: [i8; 6] = [0i8; 6];
            for (i, b) in bytes.iter().enumerate() {
                arr[i] = *b as i8;
            }
            assert_eq!(arr.to_string().unwrap(), "foo");
        }
        //null
        {
            let ptr: *const i8 = ptr::null();
            assert!(ptr.to_string().is_none());
            let ptr: *mut i8 = ptr.cast_mut();
            assert!(ptr.to_string().is_none());
            let ptr: [i8; 0] = [];
            assert!(ptr.to_string().is_none());
        }
    }
    #[test]
    fn test_cstr_list_to_string() {
        //*mut *mut i8
        {
            // not null
            {
                let s1 = CString::new("foo").unwrap();
                let s2 = CString::new("bar").unwrap();
                let s3 = CString::new("baz").unwrap();
                let arr: [*mut i8; 4] = [
                    s1.as_ptr() as *mut i8,
                    s2.as_ptr() as *mut i8,
                    s3.as_ptr() as *mut i8,
                    std::ptr::null_mut(),
                ];
                let ptr: *const *mut i8 = arr.as_ptr();
                let result = ptr.cast_mut().to_vec_string();
                assert_eq!(
                    result,
                    vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]
                );
            }
            // null
            {
                let ptr: *mut *mut i8 = ptr::null_mut();
                assert!(ptr.is_null());
                assert!(ptr.to_vec_string().is_empty());
            }
        }
        //*const *const i8
        {
            // not null
            {
                let s1 = CString::new("foo").unwrap();
                let s2 = CString::new("bar").unwrap();
                let s3 = CString::new("baz").unwrap();
                let arr: [*const i8; 4] = [
                    s1.as_ptr() as *const i8,
                    s2.as_ptr() as *const i8,
                    s3.as_ptr() as *const i8,
                    std::ptr::null_mut(),
                ];
                let ptr: *const *const i8 = arr.as_ptr();
                let result = ptr.to_vec_string();
                assert_eq!(
                    result,
                    vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]
                );
            }
            // null
            {
                let ptr: *const *const i8 = ptr::null();
                assert!(ptr.is_null());
                assert!(ptr.to_vec_string().is_empty());
            }
        }
    }
    #[test]
    fn test_to_cstring() {
        let s = String::from("foo");
        //&str
        {
            let cs = s.to_cstring();
            assert_eq!(cs, CString::new("foo").unwrap());
        }
        //String
        {
            let cs = s.to_cstring();
            assert_eq!(cs, CString::new("foo").unwrap());
        }
        //Option<&str>
        {
            let cs = Some("foo").to_cstring();
            assert_eq!(cs, CString::new("foo").unwrap());
        }
        //Option<String>
        {
            let cs = Some(s).to_cstring();
            assert_eq!(cs, CString::new("foo").unwrap());
        }
        //None
        {
            assert_eq!(Option::<&str>::None.to_cstring(), CString::default());
            assert_eq!(Option::<String>::None.to_cstring(), CString::default());
        }
    }
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
