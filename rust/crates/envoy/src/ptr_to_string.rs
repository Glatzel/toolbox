use alloc::string::{String, ToString};
use core::ffi::c_char;

use crate::PtrAsStr;

pub trait PtrToString {
    fn to_string(&self) -> Option<String>;
}

impl PtrToString for *const c_char {
    fn to_string(&self) -> Option<String> { (*self).as_str().map(|s| s.to_string()) }
}

impl PtrToString for *mut c_char {
    fn to_string(&self) -> Option<String> { (*self).as_str().map(|s| s.to_string()) }
}

impl PtrToString for [c_char] {
    fn to_string(&self) -> Option<String> { (*self).as_str().map(|s| s.to_string()) }
}
