use alloc::string::{String, ToString};
use core::ffi::{CStr, c_char};

use crate::{EnvoyError, PtrAsStr};

pub trait PtrToString {
    fn to_string(&self) -> Result<String, EnvoyError>;
    fn to_string_lossy(&self) -> Result<String, EnvoyError>;
}

impl PtrToString for *const c_char {
    fn to_string(&self) -> Result<String, EnvoyError> { Ok(self.as_str()?.to_string()) }
    fn to_string_lossy(&self) -> Result<String, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }
        unsafe { Ok(CStr::from_ptr(*self).to_string_lossy().to_string()) }
    }
}

impl PtrToString for *mut c_char {
    fn to_string(&self) -> Result<String, EnvoyError> { Ok(self.as_str()?.to_string()) }
    fn to_string_lossy(&self) -> Result<String, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }

        // SAFETY:
        // - `self` is non-null
        // - Caller guarantees valid, null-terminated memory
        unsafe { Ok(CStr::from_ptr(*self).to_string_lossy().to_string()) }
    }
}

impl PtrToString for [c_char] {
    fn to_string(&self) -> Result<String, EnvoyError> { Ok(self.as_str()?.to_string()) }
    fn to_string_lossy(&self) -> Result<String, EnvoyError> {
        // SAFETY:
        // - Slice pointer is non-null by construction
        // - Caller guarantees a terminating NUL byte
        unsafe { Ok(CStr::from_ptr(self.as_ptr()).to_string_lossy().to_string()) }
    }
}
