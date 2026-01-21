use core::ffi::{CStr, c_char};
use core::str::Utf8Error;

pub trait PtrAsStr {
    fn as_str(&self) -> Result<&str, Utf8Error>;
}

impl PtrAsStr for *const c_char {
    fn as_str(&self) -> Result<&str, Utf8Error> { unsafe { CStr::from_ptr(*self).to_str() } }
}

impl PtrAsStr for *mut c_char {
    fn as_str(&self) -> Result<&str, Utf8Error> { unsafe { CStr::from_ptr(*self).to_str() } }
}

impl PtrAsStr for [c_char] {
    fn as_str(&self) -> Result<&str, Utf8Error> {
        unsafe { CStr::from_ptr(self.as_ptr()).to_str() }
    }
}
