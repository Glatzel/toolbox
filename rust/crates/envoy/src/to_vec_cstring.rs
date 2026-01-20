use alloc::ffi::{CString, NulError};
use alloc::vec::Vec;
use core::ffi::c_char;
use core::ptr;

use crate::ToCString;

/// A helper trait for exposing collections of [`CString`]s as raw pointer
/// arrays (`*const c_char`).
///
/// Implementors of this trait produce a **null-terminated** vector of pointers,
/// which is the convention expected by many C APIs (e.g. `argv` style).
///
/// # Safety
///
/// - The returned pointers are only valid as long as the underlying
///   [`CString`]s are alive.
/// - Dropping the container or modifying it may invalidate the pointers.
pub trait AsVecPtr {
    /// Returns a **null-terminated** vector of `*const c_char` pointers
    /// referencing the internal [`CString`]s.
    ///
    /// The last element is always a `NULL` pointer (`ptr::null()`).
    fn as_vec_ptr(&self) -> Vec<*const c_char>;
}

/// A wrapper around a vector of [`CString`], with ergonomic conversions from
/// standard Rust string types.
///
/// This type is especially useful for preparing argument lists (`argv`) or
/// environment variable lists (`envp`) for C APIs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecCString {
    /// The owned collection of [`CString`]s.
    pub content: Vec<CString>,
}

impl Default for VecCString {
    fn default() -> Self { Self::new() }
}

impl VecCString {
    /// Creates a new, empty `VecCString`.
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }
}
impl AsVecPtr for VecCString {
    /// Converts the internal vector into a list of raw pointers, terminated by
    /// a `NULL` pointer.
    ///
    /// # Safety
    ///
    /// - The returned pointers must not outlive the `VecCString` itself.
    /// - Dropping `VecCString` invalidates the returned pointers.
    fn as_vec_ptr(&self) -> Vec<*const c_char> {
        let mut vec_ptr = self
            .content
            .iter()
            .map(|s| s.as_ptr())
            .collect::<Vec<*const c_char>>();
        vec_ptr.push(ptr::null());
        vec_ptr
    }
}

/// Converts a collection of Rust string-like values into a vector of
/// [`CString`] values.
///
/// This trait is primarily intended for FFI use cases where a C API expects
/// an array of `char *` / `const char *`, typically constructed from multiple
/// Rust strings.
///
/// All elements are converted eagerly. If any element fails conversion,
/// the entire operation fails.
pub trait ToVecCString {
    /// Converts the value into a [`VecCString`].
    ///
    /// # Errors
    ///
    /// Returns [`NulError`] if **any** element contains an interior NUL
    /// (`'\0'`) byte.
    ///
    /// # Notes
    ///
    /// Conversion stops at the first error; no partial results are returned.
    fn to_vec_cstring(&self) -> Result<VecCString, NulError>;
}

/// Implementation for slices of values implementing [`ToCString`].
///
/// This allows ergonomic conversion from common Rust collections such as
/// `&[String]`, `&[&str]`, or mixed types, as long as each element implements
/// `ToCString`.
impl<T: ToCString> ToVecCString for [T] {
    fn to_vec_cstring(&self) -> Result<VecCString, NulError> {
        let content = self
            .iter()
            .map(|s| s.to_cstring())
            .collect::<Result<Vec<CString>, NulError>>()?;

        Ok(VecCString { content })
    }
}
#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec;

    use super::*;

    #[test]
    fn test_vec_cstring_from_slice() -> mischief::Result<()> {
        let arr = ["foo", "bar"];
        let vec_cstring = &arr.to_vec_cstring()?;
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
        Ok(())
    }

    #[test]
    fn test_vec_cstring_from_vec() -> mischief::Result<()> {
        let arr = vec![String::from("foo"), String::from("bar")];
        let vec_cstring = arr.to_vec_cstring()?;
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
        Ok(())
    }

    #[test]
    fn test_as_vec_ptr_null_terminated() -> mischief::Result<()> {
        let arr = ["foo", "bar"];
        let vec_cstring = arr[..].to_vec_cstring()?;
        let ptrs = vec_cstring.as_vec_ptr();
        assert_eq!(ptrs.len(), 3); // 2 + null
        assert!(!ptrs[0].is_null());
        assert!(!ptrs[1].is_null());
        assert!(ptrs[2].is_null());
        Ok(())
    }
}
