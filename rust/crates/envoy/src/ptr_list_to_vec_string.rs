use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ffi::c_char;

use crate::{EnvoyError, PtrAsStr, PtrToString};

/// Convert C string pointer arrays into `Vec<String>`.
///
/// Two traversal strategies:
///
/// - **NULL-terminated** (`_null_terminated` variants): walks until a NULL
///   sentinel.
/// - **Length-bounded** (plain variants): reads exactly `len` elements.
///
/// ## Structure requirements
///
/// The pointer must reference a contiguous array of pointers, either
/// terminated by a NULL pointer (null-end variants) or of at least `len`
/// elements (length-bounded variants).
///
/// ## Safety requirements
///
/// Implementations assume:
///
/// - `self` is either NULL or points to valid readable memory
/// - pointer array is contiguous
/// - for null-end variants: array ends with a NULL sentinel
/// - for length-bounded variants: array contains at least `len` valid elements
/// - each non-NULL element points to a valid NUL-terminated C string
/// - memory remains valid during traversal
///
/// Violating these requirements is undefined behavior.
///
/// ## Conversion modes
///
/// - strict (`to_vec_string*`): requires valid UTF-8, fails on first invalid
///   string
/// - lossy (`to_vec_string_lossy*`): replaces invalid UTF-8 with U+FFFD
///
/// Both variants fail if the outer pointer itself is NULL.
pub trait PtrListToVecString {
    /// Reads exactly `len` C string pointers starting at `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::CString;
    /// use std::ptr;
    ///
    /// use envoy::PtrListToVecString;
    ///
    /// let a = CString::new("x").unwrap();
    /// let b = CString::new("y").unwrap();
    ///
    /// let list = [a.as_ptr(), b.as_ptr()];
    /// let ptr = list.as_ptr();
    ///
    /// assert_eq!(ptr.to_vec_string(2).unwrap(), ["x", "y"]);
    /// ```
    fn to_vec_string(&self, len: usize) -> Result<Vec<String>, EnvoyError>;

    /// Reads exactly `len` C string pointers, replacing invalid UTF-8 with
    /// U+FFFD.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::ffi::c_char;
    ///
    /// use envoy::PtrListToVecString;
    ///
    /// let bad = [0xFFu8 as c_char, 0];
    /// let good = [b'x' as c_char, 0];
    /// let list = [bad.as_ptr(), good.as_ptr()];
    /// let ptr = list.as_ptr();
    ///
    /// let v = ptr.to_vec_string_lossy(2).unwrap();
    /// assert_eq!(v.len(), 2);
    /// ```
    fn to_vec_string_lossy(&self, len: usize) -> Result<Vec<String>, EnvoyError>;

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
    /// assert_eq!(ptr.to_vec_string_null_terminated().unwrap(), ["a", "b"]);
    /// ```
    fn to_vec_string_null_terminated(&self) -> Result<Vec<String>, EnvoyError>;

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
    /// let v = ptr.to_vec_string_lossy_null_terminated().unwrap();
    /// assert_eq!(v.len(), 1);
    /// ```
    fn to_vec_string_lossy_null_terminated(&self) -> Result<Vec<String>, EnvoyError>;
}

impl PtrListToVecString for *const *const c_char {
    fn to_vec_string(&self, len: usize) -> Result<Vec<String>, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }

        let mut vec_str = Vec::with_capacity(len);

        for offset in 0..len {
            let current_ptr = unsafe { self.add(offset).as_ref().ok_or(EnvoyError::NullPtr)? };
            vec_str.push(current_ptr.as_str()?.to_string());
        }

        Ok(vec_str)
    }

    fn to_vec_string_lossy(&self, len: usize) -> Result<Vec<String>, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }

        let mut vec_str = Vec::with_capacity(len);

        for offset in 0..len {
            let current_ptr = unsafe { self.add(offset).as_ref().ok_or(EnvoyError::NullPtr)? };
            vec_str.push(current_ptr.to_string_lossy()?);
        }

        Ok(vec_str)
    }

    fn to_vec_string_null_terminated(&self) -> Result<Vec<String>, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }

        let mut vec_str = Vec::new();
        let mut offset = 0;

        loop {
            let current_ptr = unsafe {
                match self.add(offset).as_ref() {
                    Some(ptr) => ptr,
                    None => break,
                }
            };
            if current_ptr.is_null() {
                break;
            }

            vec_str.push(current_ptr.as_str()?.to_string());
            offset += 1;
        }

        Ok(vec_str)
    }

    fn to_vec_string_lossy_null_terminated(&self) -> Result<Vec<String>, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }

        let mut vec_str = Vec::new();
        let mut offset = 0;

        loop {
            let current_ptr = unsafe { self.add(offset).as_ref().ok_or(EnvoyError::NullPtr)? };
            if current_ptr.is_null() {
                break;
            }

            vec_str.push(current_ptr.to_string_lossy()?);
            offset += 1;
        }

        Ok(vec_str)
    }
}

impl PtrListToVecString for *mut *mut c_char {
    fn to_vec_string(&self, len: usize) -> Result<Vec<String>, EnvoyError> {
        (*self as *const *const c_char).to_vec_string(len)
    }

    fn to_vec_string_lossy(&self, len: usize) -> Result<Vec<String>, EnvoyError> {
        (*self as *const *const c_char).to_vec_string_lossy(len)
    }

    fn to_vec_string_null_terminated(&self) -> Result<Vec<String>, EnvoyError> {
        (*self as *const *const c_char).to_vec_string_null_terminated()
    }

    fn to_vec_string_lossy_null_terminated(&self) -> Result<Vec<String>, EnvoyError> {
        (*self as *const *const c_char).to_vec_string_lossy_null_terminated()
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

    // --- null-end ---

    #[test]
    fn null_terminated_const_list_valid() {
        let (_keep_alive, ptrs) = make_list(&["a", "b", "c"]);
        let list = ptrs.as_ptr();
        let out = list.to_vec_string_null_terminated().unwrap();
        assert_eq!(out, ["a", "b", "c"]);
    }

    #[test]
    fn null_terminated_mut_list_valid() {
        let (_keep_alive, mut ptrs) = make_list(&["x", "y"]);
        let list = ptrs.as_mut_ptr() as *mut *mut c_char;
        let out = list.to_vec_string_null_terminated().unwrap();
        assert_eq!(out, ["x", "y"]);
    }

    #[test]
    fn null_terminated_empty_list() {
        let ptrs = [ptr::null::<c_char>()];
        let list = ptrs.as_ptr();
        let out = list.to_vec_string_null_terminated().unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn null_terminated_null_list_pointer() {
        let list: *const *const c_char = ptr::null();
        assert!(list.to_vec_string_null_terminated().is_err());
        assert!(list.to_vec_string_lossy_null_terminated().is_err());
    }

    #[test]
    fn null_terminated_lossy_invalid_utf8() {
        let bad = [0xFFu8, 0x00];
        let bad_ptr = bad.as_ptr() as *const c_char;
        let ptrs = [bad_ptr, ptr::null()];
        let list = ptrs.as_ptr();
        let out = list.to_vec_string_lossy_null_terminated().unwrap();
        assert_eq!(out.len(), 1);
        assert!(out[0].contains('\u{FFFD}'));
    }

    #[test]
    fn null_terminated_strict_invalid_utf8_should_fail() {
        let bad = [0xFFu8, 0x00];
        let bad_ptr = bad.as_ptr() as *const c_char;
        let ptrs = [bad_ptr, ptr::null()];
        let list = ptrs.as_ptr();
        assert!(list.to_vec_string_null_terminated().is_err());
    }

    // --- length-bounded ---

    #[test]
    fn len_const_list_valid() {
        let (_keep_alive, ptrs) = make_list(&["a", "b", "c"]);
        // ptrs has a trailing null but we only read 3
        let list = ptrs.as_ptr();
        let out = list.to_vec_string(3).unwrap();
        assert_eq!(out, ["a", "b", "c"]);
    }

    #[test]
    fn len_mut_list_valid() {
        let (_keep_alive, mut ptrs) = make_list(&["x", "y"]);
        let list = ptrs.as_mut_ptr() as *mut *mut c_char;
        let out = list.to_vec_string(2).unwrap();
        assert_eq!(out, ["x", "y"]);
    }

    #[test]
    fn len_zero() {
        let (_keep_alive, ptrs) = make_list(&["a"]);
        let list = ptrs.as_ptr();
        let out = list.to_vec_string(0).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn len_null_list_pointer() {
        let list: *const *const c_char = ptr::null();
        assert!(list.to_vec_string(1).is_err());
        assert!(list.to_vec_string_lossy(1).is_err());
    }

    #[test]
    fn len_lossy_invalid_utf8() {
        let bad = [0xFFu8, 0x00];
        let bad_ptr = bad.as_ptr() as *const c_char;
        let ptrs = [bad_ptr]; // no null sentinel needed
        let list = ptrs.as_ptr();
        let out = list.to_vec_string_lossy(1).unwrap();
        assert_eq!(out.len(), 1);
        assert!(out[0].contains('\u{FFFD}'));
    }

    #[test]
    fn len_strict_invalid_utf8_should_fail() {
        let bad = [0xFFu8, 0x00];
        let bad_ptr = bad.as_ptr() as *const c_char;
        let ptrs = [bad_ptr];
        let list = ptrs.as_ptr();
        assert!(list.to_vec_string(1).is_err());
    }
}
