use std::ffi::{CStr, CString};
use std::ptr;
const CSTRING_NEW_EXPECTION: &str = "Failed to create CString";
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
        Some(
            unsafe { CStr::from_ptr(self.as_ptr()) }
                .to_string_lossy()
                .to_string(),
        )
    }
}

/// Trait for converting a null-terminated list of C string pointers to a
/// `Vec<String>`.
pub trait CStrListToVecString {
    /// Converts the list to a vector of Rust `String`.
    /// Returns `None` if the pointer is null.
    fn to_vec_string(&self) -> Option<Vec<String>>;
}

impl CStrListToVecString for *mut *mut i8 {
    /// Converts a null-terminated array of C string pointers to a vector of
    /// Rust `String`.
    fn to_vec_string(&self) -> Option<Vec<String>> {
        if self.is_null() {
            return None;
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
        Some(vec_str)
    }
}

/// Trait for converting Rust strings to `CString`.
pub trait ToCStr {
    fn to_cstring(&self) -> CString;
    fn to_cstr(&self) -> *const i8;
}

impl ToCStr for &str {
    fn to_cstring(&self) -> CString { CString::new(*self).expect(CSTRING_NEW_EXPECTION) }
    fn to_cstr(&self) -> *const i8 { self.to_cstring().into_raw() }
}

impl ToCStr for String {
    fn to_cstring(&self) -> CString { CString::new(self.as_str()).expect(CSTRING_NEW_EXPECTION) }
    fn to_cstr(&self) -> *const i8 { self.to_cstring().into_raw() }
}

impl ToCStr for Option<&str> {
    fn to_cstring(&self) -> CString {
        CString::new(self.unwrap_or_default()).expect(CSTRING_NEW_EXPECTION)
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
            Some(s) => CString::new(s.as_str()).expect(CSTRING_NEW_EXPECTION),
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
            let ptr: *const i8 = s.as_ptr();
            assert_eq!(ptr.to_string().unwrap(), "foo");
        }
        //[i8]
        {
            let bytes = s.as_bytes_with_nul();
            let mut arr = [0i8; 6];
            for (i, b) in bytes.iter().enumerate() {
                arr[i] = *b as i8;
            }
            assert_eq!(arr.to_string().unwrap(), "foo");
        }
        //null
        {
            let ptr: *const i8 = ptr::null();
            assert!(ptr.to_string().is_none());
        }
    }
    #[test]
    fn test_cstr_list_to_string() {
        let s1 = CString::new("foo").unwrap();
        let s2 = CString::new("bar").unwrap();
        let s3 = CString::new("baz").unwrap();
        let arr = [
            s1.as_ptr() as *mut i8,
            s2.as_ptr() as *mut i8,
            s3.as_ptr() as *mut i8,
            std::ptr::null_mut(),
        ];
        let ptr = arr.as_ptr();
        let result = ptr.cast_mut().to_vec_string();
        assert_eq!(
            result,
            Some(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string()
            ])
        );
    }
    #[test]
    fn test_to_cstring() {
        let s = String::from("foo");
        //&str
        {
            let cs = s.as_str().to_cstring();
            assert_eq!(cs, CString::new("foo").unwrap());
        }
        //String
        {
            let cs = s.to_cstring();
            assert_eq!(cs, CString::new("foo").unwrap());
        }
        //Option<&str>
        {
            let cs = Some(s.as_str()).to_cstring();
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
        let s = String::from("foo");
        //&str
        {
            let ptr = s.as_str().to_cstr();
            assert_eq!(ptr.to_string().unwrap(), "foo");
            assert!(!ptr.is_null())
        }
        //String
        {
            let ptr = s.to_cstr();
            assert_eq!(ptr.to_string().unwrap(), "foo");
            assert!(!ptr.is_null());
        }
        //Option<&str>
        {
            let ptr = Some(s.as_str()).to_cstr();
            assert!(!ptr.is_null());
            assert_eq!(ptr.to_string().unwrap(), "foo");
        }
        //Option<String>
        {
            let ptr = Some(s).to_cstr();
            assert!(!ptr.is_null());
            assert_eq!(ptr.to_string().unwrap(), "foo");
        }
        //None
        {
            assert_eq!(Option::<&str>::None.to_cstr(), ptr::null());
            assert_eq!(Option::<String>::None.to_cstr(), ptr::null());
        }
    }
}
