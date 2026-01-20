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
/// Conversion trait for producing a vector of owned C strings.
///
/// This trait is intended for **FFI use cases** where a C API expects
/// an array of `const char*` (typically `char**`) and requires all
/// strings to be:
///
/// - NUL-terminated
/// - Free of interior NUL bytes
/// - Valid for the duration of the FFI call
///
/// ## Allocation Behavior
///
/// This conversion **always allocates**:
///
/// - Each element is converted into a [`CString`]
/// - The resulting `Vec<CString>` owns all string buffers
///
/// This is required to satisfy C string invariants and lifetime rules.
/// Zero-copy conversion from Rust strings to C strings is not possible.
///
/// ## Typical Use Case
///
/// ```c
/// void c_api(const char** args, size_t len);
/// ```
///
/// ```rust
/// let args = ["foo", "bar", "baz"];
/// let c_args = args.to_vec_cstring();
///
/// unsafe {
///     c_api(c_args.as_ptr(), c_args.len());
/// }
/// ```
///
/// ## Safety and Lifetime
///
/// The returned [`VecCString`] owns all underlying `CString` values,
/// ensuring that the pointers passed to C remain valid as long as
/// the `VecCString` is kept alive.
///
/// ## Errors
///
/// If any element contains an interior NUL byte, conversion will fail
/// at the [`ToCString`] level.
///
/// ## See Also
///
/// - [`CString`] for individual C string ownership
/// - [`ToCString`] for single-value conversion
pub trait ToVecCString {
    /// Converts a slice into a vector of owned C strings.
    fn to_vec_cstring(&self) -> VecCString;
}

impl<T: ToCString> ToVecCString for [T] {
    fn to_vec_cstring(&self) -> VecCString {
        VecCString {
            content: self
                .iter()
                .map(|s| s.to_cstring())
                .collect::<Vec<CString>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec;

    use super::*;

    #[test]
    fn test_vec_cstring_from_slice() {
        let arr = ["foo", "bar"];
        let vec_cstring = &arr.to_vec_cstring();
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
    }

    #[test]
    fn test_vec_cstring_from_vec() {
        let arr = vec![String::from("foo"), String::from("bar")];
        let vec_cstring = arr.to_vec_cstring();
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
    }

    #[test]
    fn test_as_vec_ptr_null_terminated() {
        let arr = ["foo", "bar"];
        let vec_cstring = arr[..].to_vec_cstring();
        let ptrs = vec_cstring.as_vec_ptr();
        assert_eq!(ptrs.len(), 3); // 2 + null
        assert!(!ptrs[0].is_null());
        assert!(!ptrs[1].is_null());
        assert!(ptrs[2].is_null());
    }
}
