use std::ffi::{CStr, c_char};

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
            let ptr: *const c_char = s.as_ptr();
            assert_eq!(ptr.to_string().unwrap(), "foo");
        }
        //*mut i8
        {
            let ptr: *mut c_char = s.as_ptr().cast_mut();
            assert_eq!(ptr.to_string().unwrap(), "foo");
        }
        //null
        {
            let ptr: *const c_char = ptr::null();
            assert!(ptr.to_string().is_none());
            let ptr: *mut c_char = ptr.cast_mut();
            assert!(ptr.to_string().is_none());
            let ptr: [c_char; 0] = [];
            assert!(ptr.to_string().is_none());
        }
    }
}
