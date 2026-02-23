use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ffi::c_char;

use crate::{EnvoyError, PtrAsStr, PtrToString};

pub trait PtrListToVecString {
    fn to_vec_string(&self) -> Result<Vec<String>, EnvoyError>;
    fn to_vec_string_lossy(&self) -> Result<Vec<String>, EnvoyError>;
}

impl PtrListToVecString for *mut *mut c_char {
    fn to_vec_string(&self) -> Result<Vec<String>, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }
        let mut vec_str = Vec::new();
        let mut offset = 0;
        loop {
            let current_ptr = unsafe { self.offset(offset).as_ref().unwrap() };
            if current_ptr.is_null() {
                break;
            }
            vec_str.push(current_ptr.as_str()?.to_string());
            offset += 1;
        }
        Ok(vec_str)
    }
    fn to_vec_string_lossy(&self) -> Result<Vec<String>, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }
        let mut vec_str = Vec::new();
        let mut offset = 0;
        loop {
            let current_ptr = unsafe { self.offset(offset).as_ref().unwrap() };
            if current_ptr.is_null() {
                break;
            }
            vec_str.push(current_ptr.cast_const().to_string_lossy().unwrap());
            offset += 1;
        }
        Ok(vec_str)
    }
}
impl PtrListToVecString for *const *const c_char {
    fn to_vec_string(&self) -> Result<Vec<String>, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }
        let mut vec_str = Vec::new();
        let mut offset = 0;
        loop {
            let current_ptr = unsafe { self.offset(offset).as_ref().unwrap() };
            if current_ptr.is_null() {
                break;
            }
            vec_str.push(current_ptr.as_str()?.to_string());
            offset += 1;
        }
        Ok(vec_str)
    }
    fn to_vec_string_lossy(&self) -> Result<Vec<String>, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }
        let mut vec_str = Vec::new();
        let mut offset = 0;
        loop {
            let current_ptr = unsafe { self.offset(offset).as_ref().unwrap() };
            if current_ptr.is_null() {
                break;
            }

            vec_str.push(current_ptr.to_string_lossy().unwrap());
            offset += 1;
        }
        Ok(vec_str)
    }
}
