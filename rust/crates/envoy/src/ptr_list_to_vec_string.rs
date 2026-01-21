use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;

use crate::{EnvoyError, PtrToString};

/// Converts a null-terminated list of C string pointers into a `Vec<String>`.
///
/// This trait is intended for FFI scenarios where a C API returns a pointer
/// to a null-terminated array of `char *` / `const char *`, such as:
///
/// ```c
/// char **argv;
/// const char * const *names;
/// ```
///
/// The list itself must be terminated by a null pointer (`NULL`), and each
/// entry must point to a valid, null-terminated C string encoded as UTF-8.
///
/// # UTF-8 Requirement
///
/// All C strings referenced by the pointer list **must** be valid UTF-8.
/// Invalid UTF-8 will cause a panic due to `unwrap()` in the current
/// implementation.
///
/// # Safety
///
/// Although the API is safe, all implementations rely on `unsafe` pointer
/// traversal. The caller must guarantee:
///
/// - The list pointer is either null or valid for reads
/// - The list is terminated by a null pointer
/// - Each non-null entry points to a valid, null-terminated C string
/// - All memory remains valid for the duration of the call
///
/// Violating any of these requirements results in **undefined behavior**.
///
/// # Typical Use Case
///
/// This trait is commonly used to convert C-style argument vectors
/// (`argv`-like structures) into owned Rust strings.
pub trait PtrListToVecString {
    /// Converts a null-terminated list of C string pointers into `Vec<String>`.
    ///
    /// # Returns
    ///
    /// - An empty vector if the list pointer itself is null
    /// - A vector containing one `String` per entry, stopping at the first null
    ///   pointer
    ///
    /// # Panics
    ///
    /// Panics if any referenced C string contains invalid UTF-8.
    fn to_vec_string(&self) -> Result<Vec<String>, EnvoyError>;
}

/// Implementation for `*mut *mut c_char` (e.g. `char **`).
///
/// Mutability is ignored; all data is treated as read-only.
///
/// # Safety
///
/// The pointer must reference a valid, null-terminated list of pointers,
/// where each pointer references a valid C string.
impl PtrListToVecString for *mut *mut c_char {
    fn to_vec_string(&self) -> Result<Vec<String>, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
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

        Ok(vec_str)
    }
}

/// Implementation for `*const *const c_char` (e.g. `const char * const *`).
///
/// # Safety
///
/// The pointer must reference a valid, null-terminated list of pointers,
/// where each pointer references a valid C string.
impl PtrListToVecString for *const *const c_char {
    fn to_vec_string(&self) -> Result<Vec<String>, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
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

        Ok(vec_str)
    }
}

#[cfg(test)]
mod tests {
    use alloc::ffi::CString;
    use alloc::vec;
    use core::ptr;

    use super::*;

    #[test]
    fn test_cstr_list_to_string() -> mischief::Result<()> {
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
                let result = ptr.cast_mut().to_vec_string()?;
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
                assert!(ptr.to_vec_string().is_err());
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
                let result = ptr.to_vec_string()?;
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
                assert!(ptr.to_vec_string().is_err());
            }
            Ok(())
        }
    }
}
