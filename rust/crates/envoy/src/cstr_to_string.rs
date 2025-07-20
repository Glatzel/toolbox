use std::ffi::{c_char, CStr};

/// Trait for converting C string pointers and slices to Rust `String`.
pub trait CStrToString {
    /// Converts the C string to a Rust `String`.
    /// Returns `None` if the pointer is null.
    fn to_string(&self) -> Option<String>;
}

impl CStrToString for *const c_char {
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
impl CStrToString for *mut c_char {
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
impl CStrToString for [c_char] {
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
#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::ptr;

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
}
