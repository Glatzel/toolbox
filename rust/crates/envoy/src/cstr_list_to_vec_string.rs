use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;

use crate::CStrToString;

/// A helper trait for converting null-terminated lists of C strings (`char**`)
/// into a safe Rust [`Vec<String>`].
///
/// # Safety
///
/// - The underlying pointer must represent a **null-terminated list** of
///   C-style string pointers.
/// - Each non-null element must point to a valid, null-terminated C string.
/// - This trait will traverse the list until it encounters a `NULL` pointer.
/// - If any pointer is invalid or points to non-UTF-8 data, conversion may
///   fail.
pub trait CStrListToVecString {
    /// Convert the underlying C string list into a Rust [`Vec<String>`].
    ///
    /// Returns an empty vector if the pointer itself is null.
    fn to_vec_string(&self) -> Vec<String>;
}

impl CStrListToVecString for *mut *mut c_char {
    /// Implementation for mutable double pointers (`*mut *mut c_char`),
    /// often used for `argv` in `main(int argc, char** argv)`.
    ///
    /// # Safety
    ///
    /// - Traverses the pointer list until a `NULL` entry is found.
    /// - Assumes each pointer is valid and points to a UTF-8 C string.
    fn to_vec_string(&self) -> Vec<String> {
        if self.is_null() {
            return Vec::new();
        }
        let mut vec_str = Vec::new();
        let mut offset = 0;

        loop {
            // SAFETY: `offset` walks through a null-terminated list of pointers.
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
    /// Implementation for const double pointers (`*const *const c_char`),
    /// also common in FFI contexts.
    ///
    /// # Safety
    ///
    /// - Traverses the pointer list until a `NULL` entry is found.
    /// - Assumes each pointer is valid and points to a UTF-8 C string.
    fn to_vec_string(&self) -> Vec<String> {
        if self.is_null() {
            return Vec::new();
        }
        let mut vec_str = Vec::new();
        let mut offset = 0;

        loop {
            // SAFETY: `offset` walks through a null-terminated list of pointers.
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
