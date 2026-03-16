use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ffi::c_char;

use crate::{EnvoyError, PtrAsStr, PtrToString};

/// Convert a NULL-terminated list of C string pointers into `Vec<String>`.
///
/// This trait is intended for FFI patterns such as:
///
/// ```text
/// char **argv;
/// argv[0] -> "foo"
/// argv[1] -> "bar"
/// argv[2] -> NULL
/// ```
///
/// ## Structure requirements
///
/// The pointer must reference a contiguous array of pointers
/// terminated by a NULL pointer.
///
/// ```text
/// [ ptr, ptr, ptr, NULL ]
/// ```
///
/// ## Safety requirements
///
/// Implementations assume:
///
/// - `self` is either NULL or points to valid readable memory
/// - pointer array is contiguous
/// - pointer array ends with a NULL sentinel
/// - each non-NULL element points to a valid NUL-terminated C string
/// - memory remains valid during traversal
///
/// Violating these requirements is undefined behavior.
///
/// ## Conversion modes
///
/// - `to_vec_string`
///   - requires valid UTF-8 for every element
///   - fails on first invalid string
///
/// - `to_vec_string_lossy`
///   - replaces invalid UTF-8 with U+FFFD
///   - still fails if pointer list itself is NULL
pub trait PtrListToVecString {
    fn to_vec_string(&self) -> Result<Vec<String>, EnvoyError>;
    fn to_vec_string_lossy(&self) -> Result<Vec<String>, EnvoyError>;
}

impl PtrListToVecString for *mut *mut c_char {
    /// # Examples
    ///
    /// ```
    /// use std::ffi::CString;
    /// use std::ptr;
    ///
    /// use envoy::PtrListToVecString;
    ///
    /// let a = CString::new("a").unwrap();
    /// let b = CString::new("b").unwrap();
    ///
    /// let list = [a.as_ptr(), b.as_ptr(), ptr::null()];
    /// let ptr = list.as_ptr();
    ///
    /// assert_eq!(ptr.to_vec_string().unwrap(), ["a", "b"]);
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// use std::ffi::c_char;
    /// use std::ptr;
    ///
    /// use envoy::PtrListToVecString;
    ///
    /// let bad = [0xFFu8 as c_char, 0];
    /// let bad_ptr = bad.as_ptr();
    ///
    /// let list = [bad_ptr, ptr::null()];
    /// let ptr = list.as_ptr();
    ///
    /// let v = ptr.to_vec_string_lossy().unwrap();
    /// assert_eq!(v.len(), 1);
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// use std::ffi::{CString, c_char};
    /// use std::ptr;
    ///
    /// use envoy::PtrListToVecString;
    ///
    /// let a = CString::new("x").unwrap();
    /// let b = CString::new("y").unwrap();
    ///
    /// let mut list = [
    ///     a.as_ptr() as *mut c_char,
    ///     b.as_ptr() as *mut c_char,
    ///     ptr::null_mut(),
    /// ];
    /// let ptr = list.as_mut_ptr();
    ///
    /// assert_eq!(ptr.to_vec_string().unwrap(), ["x", "y"]);
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// use core::ffi::c_char;
    /// use core::ptr;
    ///
    /// use envoy::PtrListToVecString;
    ///
    /// let bad = [0xFFu8 as c_char, 0];
    /// let bad_ptr = bad.as_ptr() as *mut c_char;
    ///
    /// let mut list = [bad_ptr, ptr::null_mut()];
    /// let ptr = list.as_mut_ptr();
    ///
    /// let v = ptr.to_vec_string_lossy().unwrap();
    /// assert_eq!(v.len(), 1);
    /// ```
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
#[cfg(test)]
mod tests {
    extern crate std;
    use core::ptr;
    use std::ffi::CString;

    use super::*;

    fn make_list(strings: &[&str]) -> (Vec<CString>, Vec<*const c_char>) {
        let cstrings: Vec<CString> = strings.iter().map(|s| CString::new(*s).unwrap()).collect();

        let mut ptrs: Vec<*const c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();

        ptrs.push(ptr::null());

        (cstrings, ptrs)
    }

    #[test]
    fn const_list_valid() {
        let (_keep_alive, ptrs) = make_list(&["a", "b", "c"]);
        let list = ptrs.as_ptr();

        let out = list.to_vec_string().unwrap();
        assert_eq!(out, ["a", "b", "c"]);
    }

    #[test]
    fn mut_list_valid() {
        let (_keep_alive, mut ptrs) = make_list(&["x", "y"]);
        let list = ptrs.as_mut_ptr() as *mut *mut c_char;

        let out = list.to_vec_string().unwrap();
        assert_eq!(out, ["x", "y"]);
    }

    #[test]
    fn empty_list() {
        let ptrs = [ptr::null::<c_char>()];
        let list = ptrs.as_ptr();

        let out = list.to_vec_string().unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn null_list_pointer() {
        let list: *const *const c_char = ptr::null();
        assert!(list.to_vec_string().is_err());
        assert!(list.to_vec_string_lossy().is_err());
    }

    #[test]
    fn lossy_invalid_utf8() {
        let bad = [0xFFu8, 0x00];
        let bad_ptr = bad.as_ptr() as *const c_char;

        let ptrs = [bad_ptr, ptr::null()];
        let list = ptrs.as_ptr();

        let out = list.to_vec_string_lossy().unwrap();
        assert_eq!(out.len(), 1);
        assert!(out[0].contains('\u{FFFD}'));
    }

    #[test]
    fn strict_invalid_utf8_should_fail() {
        let bad = [0xFFu8, 0x00];
        let bad_ptr = bad.as_ptr() as *const c_char;

        let ptrs = [bad_ptr, ptr::null()];
        let list = ptrs.as_ptr();

        assert!(list.to_vec_string().is_err());
    }
}
