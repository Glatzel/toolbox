use alloc::string::{String, ToString};
use core::ffi::{CStr, c_char};
use core::str::Utf8Error;

pub trait PtrToStr {
    fn to_string(&self) -> Result<String, Utf8Error>;
}

impl PtrToStr for *const c_char {
    fn to_string(&self) -> Result<String, Utf8Error> {
        unsafe { Ok(CStr::from_ptr(*self).to_str()?.to_string()) }
    }
}

impl PtrToStr for *mut c_char {
    fn to_string(&self) -> Result<String, Utf8Error> {
        unsafe { Ok(CStr::from_ptr(*self).to_str()?.to_string()) }
    }
}

impl PtrToStr for [c_char] {
    fn to_string(&self) -> Result<String, Utf8Error> {
        unsafe { Ok(CStr::from_ptr(self.as_ptr()).to_str()?.to_string()) }
    }
}
#[cfg(test)]
mod tests {
    use alloc::ffi::CString;
    use core::ptr;

    use super::*;
    #[test]
    fn test_cstr_to_string() -> mischief::Result<()> {
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
            ptr.to_string()?;
            let ptr: *mut c_char = ptr.cast_mut();
            ptr.to_string()?;
            let ptr: [c_char; 0] = [];
            ptr.to_string()?;
        }
        Ok(())
    }
}
