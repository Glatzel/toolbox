use alloc::ffi::CString;
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

impl<T: ToCString> From<&[T]> for VecCString {
    /// Converts a slice of Rust string-like types into a `VecCString`.
    ///
    /// Each element is converted to a [`CString`] via [`ToCString`].
    fn from(value: &[T]) -> Self {
        VecCString {
            content: value
                .iter()
                .map(|s| s.to_cstring())
                .collect::<Vec<CString>>(),
        }
    }
}

impl<T: ToCString> From<Vec<T>> for VecCString {
    /// Converts a vector of Rust string-like types into a `VecCString`.
    ///
    /// Each element is converted to a [`CString`] via [`ToCString`].
    fn from(value: Vec<T>) -> Self {
        VecCString {
            content: value
                .iter()
                .map(|s| s.to_cstring())
                .collect::<Vec<CString>>(),
        }
    }
}

impl<T: ToCString> From<Option<Vec<T>>> for VecCString {
    /// Converts an `Option<Vec<T>>` into a `VecCString`.
    ///
    /// - If `Some(v)`, each element is converted into a [`CString`].
    /// - If `None`, returns an empty `VecCString`.
    fn from(value: Option<Vec<T>>) -> Self { value.as_deref().map_or_else(Self::new, Self::from) }
}

impl<T: ToCString> From<Option<&[T]>> for VecCString {
    /// Converts an `Option<&[T]>` into a `VecCString`.
    ///
    /// - If `Some(slice)`, each element is converted into a [`CString`].
    /// - If `None`, returns an empty `VecCString`.
    fn from(value: Option<&[T]>) -> Self { value.map_or_else(Self::new, Self::from) }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec;

    use super::*;

    #[test]
    fn test_vec_cstring_from_slice() {
        let arr = ["foo", "bar"];
        let vec_cstring = VecCString::from(&arr[..]);
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
    }

    #[test]
    fn test_vec_cstring_from_vec() {
        let arr = vec![String::from("foo"), String::from("bar")];
        let vec_cstring = VecCString::from(arr.clone());
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
    }

    #[test]
    fn test_vec_cstring_from_option_vec_some() {
        let arr = Some(vec!["foo", "bar"]);
        let vec_cstring = VecCString::from(arr);
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
    }

    #[test]
    fn test_vec_cstring_from_option_vec_none() {
        let arr: Option<Vec<&str>> = None;
        let vec_cstring = VecCString::from(arr);
        assert_eq!(vec_cstring.content.len(), 0);
    }

    #[test]
    fn test_vec_cstring_from_option_slice_some() {
        let arr = ["foo", "bar"];
        let arr_opt = Some(&arr[..]);
        let vec_cstring = VecCString::from(arr_opt);
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
    }

    #[test]
    fn test_vec_cstring_from_option_slice_none() {
        let arr: Option<&[&str]> = None;
        let vec_cstring = VecCString::from(arr);
        assert_eq!(vec_cstring.content.len(), 0);
    }

    #[test]
    fn test_as_vec_ptr_null_terminated() {
        let arr = ["foo", "bar"];
        let vec_cstring = VecCString::from(&arr[..]);
        let ptrs = vec_cstring.as_vec_ptr();
        assert_eq!(ptrs.len(), 3); // 2 + null
        assert!(!ptrs[0].is_null());
        assert!(!ptrs[1].is_null());
        assert!(ptrs[2].is_null());
    }
}
