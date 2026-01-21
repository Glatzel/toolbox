use alloc::string::{String, ToString};
use core::ffi::c_char;

use crate::{EnvoyError, PtrAsStr};

pub trait PtrToString {
    fn to_string(&self) -> Result<String, EnvoyError>;
}

impl PtrToString for *const c_char {
    fn to_string(&self) -> Result<String, EnvoyError> { (*self).as_str().map(ToString::to_string) }
}

impl PtrToString for *mut c_char {
    fn to_string(&self) -> Result<String, EnvoyError> { (*self).as_str().map(ToString::to_string) }
}

impl PtrToString for [c_char] {
    fn to_string(&self) -> Result<String, EnvoyError> { (*self).as_str().map(ToString::to_string) }
}
#[cfg(test)]
mod tests {

    use alloc::ffi::CString;
    use core::ptr;

    use super::*;

    #[test]
    fn const_ptr_to_string() -> mischief::Result<()> {
        let s = CString::new("hello").unwrap();
        let ptr = s.as_ptr();

        let out = ptr.to_string()?;
        assert_eq!(out, "hello");
        Ok(())
    }

    #[test]
    fn mut_ptr_to_string() -> mischief::Result<()> {
        let s = CString::new("world").unwrap();
        let ptr = s.as_ptr().cast_mut();

        let out = ptr.to_string()?;
        assert_eq!(out, "world");
        Ok(())
    }

    #[test]
    fn null_ptr_returns_none() {
        let ptr: *const c_char = ptr::null();
        assert!(ptr.to_string().is_err());
    }

    #[test]
    fn invalid_utf8_returns_none() {
        let bytes = [0xFFu8 as c_char, 0];
        let ptr = bytes.as_ptr() as *const c_char;
        assert!(ptr.to_string().is_err());
    }

    #[test]
    fn slice_c_char_to_string() -> mischief::Result<()> {
        let bytes = b"slice-owned\0";
        let slice: &[c_char] =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const c_char, bytes.len()) };

        let out = slice.to_string()?;
        assert_eq!(out, "slice-owned");
        Ok(())
    }

    #[test]
    fn returned_string_is_owned() {
        let s = CString::new("owned").unwrap();
        let ptr = s.as_ptr();

        let out = ptr.to_string().unwrap();
        drop(s);

        // The String must remain valid after the source is dropped
        assert_eq!(out, "owned");
    }
}
