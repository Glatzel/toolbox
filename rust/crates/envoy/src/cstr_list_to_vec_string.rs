use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;

use crate::CStrToString;

pub trait CStrListToVecString {
    fn to_vec_string(&self) -> Vec<String>;
}

impl CStrListToVecString for *mut *mut c_char {
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
impl CStrListToVecString for *const *const c_char {
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
#[cfg(test)]
mod tests {
    use alloc::ffi::CString;
    use alloc::vec;
    use core::ptr;

    use super::*;

    #[test]
    fn test_cstr_list_to_string() {
        //*mut *mut i8
        {
            // not null
            {
                let s1 = CString::new("foo").unwrap();
                let s2 = CString::new("bar").unwrap();
                let s3 = CString::new("baz").unwrap();
                let arr: [*mut c_char; 4] = [
                    s1.as_ptr() as *mut c_char,
                    s2.as_ptr() as *mut c_char,
                    s3.as_ptr() as *mut c_char,
                    core::ptr::null_mut(),
                ];
                let ptr: *const *mut c_char = arr.as_ptr();
                let result = ptr.cast_mut().to_vec_string();
                assert_eq!(
                    result,
                    vec![
                        String::from("foo"),
                        String::from("bar"),
                        String::from("baz")
                    ]
                );
            }
            // null
            {
                let ptr: *mut *mut c_char = ptr::null_mut();
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
                let arr: [*const c_char; 4] = [
                    s1.as_ptr() as *const c_char,
                    s2.as_ptr() as *const c_char,
                    s3.as_ptr() as *const c_char,
                    core::ptr::null_mut(),
                ];
                let ptr: *const *const c_char = arr.as_ptr();
                let result = ptr.to_vec_string();
                assert_eq!(
                    result,
                    vec![
                        String::from("foo"),
                        String::from("bar"),
                        String::from("baz")
                    ]
                );
            }
            // null
            {
                let ptr: *const *const c_char = ptr::null();
                assert!(ptr.is_null());
                assert!(ptr.to_vec_string().is_empty());
            }
        }
    }
}
