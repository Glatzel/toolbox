use alloc::ffi::CString;
use alloc::vec;
use alloc::vec::Vec;
use core::ffi::c_char;
use core::ptr;

use crate::{EnvoyError, ToCString};

/// A helper trait for exposing a [`VecCString`] as a null-terminated raw
/// pointer array (`*const *const c_char`).
///
/// The returned pointers always form a **null-terminated** array, which is the
/// convention expected by C APIs such as `execve`'s `argv` and `envp`.
///
/// # Safety
///
/// - The returned pointers are only valid as long as the [`VecCString`] is
///   alive and unmodified.
/// - Any mutation of the [`VecCString`] invalidates previously returned
///   pointers.
pub trait AsVecPtr {
    /// Returns a `*const *const c_char` pointing into the internal pointer
    /// buffer. The array is always null-terminated; its length equals the
    /// number of strings plus one.
    ///
    /// # Safety
    ///
    /// The pointer is valid only as long as the [`VecCString`] is alive and
    /// unmodified.
    fn as_ptr(&self) -> *const *const c_char;

    /// Returns a `*mut *mut c_char` pointing into the internal pointer buffer.
    /// The array is always null-terminated.
    ///
    /// # Safety
    ///
    /// The pointer is valid only as long as the [`VecCString`] is alive and
    /// unmodified. Mutating through the returned pointer must not write a
    /// pointer that outlives the underlying [`CString`].
    fn as_mut_ptr(&mut self) -> *mut *mut c_char;
}

/// A wrapper around a vector of [`CString`] that keeps a synchronised
/// null-terminated pointer buffer for cheap FFI hand-off.
///
/// `content` is private so that every mutation goes through methods that
/// rebuild `ptr_buf`, keeping the two fields in sync at all times.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VecCString {
    buffer: Vec<CString>,
    /// Null-terminated array of pointers into `content`.
    /// Length is always `content.len() + 1`; the last element is `ptr::null()`.
    ptr_buffer: Vec<*const c_char>,
}

// SAFETY: VecCString owns its CStrings; the raw pointers in ptr_buf are
// derived from those owned values and are never exposed beyond the lifetime
// of &self / &mut self.
unsafe impl Send for VecCString {}
unsafe impl Sync for VecCString {}

impl Clone for VecCString {
    fn clone(&self) -> Self {
        // Clone content into a fresh allocation, then rebuild ptr_buf from
        // the new addresses. Simply copying ptr_buf would leave dangling
        // pointers into the original's CString heap buffers.
        let content = self.buffer.clone();
        let ptr_buf = Self::build_ptr_buf(&content);
        Self {
            buffer: content,
            ptr_buffer: ptr_buf,
        }
    }
}

impl Default for VecCString {
    fn default() -> Self { Self::new() }
}

impl VecCString {
    /// Creates a new, empty `VecCString`.
    ///
    /// # Examples
    ///
    /// ```
    /// use envoy::VecCString;
    ///
    /// let v = VecCString::new();
    /// assert!(v.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            ptr_buffer: vec![ptr::null()],
        }
    }

    /// Returns the number of strings (excluding the null terminator).
    pub fn len(&self) -> usize { self.buffer.len() }

    /// Returns `true` if there are no strings.
    pub fn is_empty(&self) -> bool { self.buffer.is_empty() }

    /// Appends a [`CString`] and rebuilds the pointer buffer.
    pub fn push(&mut self, s: CString) {
        self.buffer.push(s);
        self.rebuild_ptr_buf();
    }

    /// Extends from an iterator of [`CString`]s and rebuilds the pointer
    /// buffer once at the end.
    pub fn extend_from_cstrings(&mut self, iter: impl IntoIterator<Item = CString>) {
        self.buffer.extend(iter);
        self.rebuild_ptr_buf();
    }

    /// Returns a shared reference to the underlying [`CString`] slice.
    pub fn as_slice(&self) -> &[CString] { &self.buffer }

    /// Builds a null-terminated pointer buffer from a [`CString`] slice.
    fn build_ptr_buf(content: &[CString]) -> Vec<*const c_char> {
        content
            .iter()
            .map(|s| s.as_ptr())
            .chain(core::iter::once(ptr::null()))
            .collect()
    }

    /// Rebuilds `ptr_buf` in place from the current `content`.
    ///
    /// Must be called after every operation that moves or reallocates
    /// `content`, because a reallocation changes the addresses of the
    /// `CString` heap buffers.
    fn rebuild_ptr_buf(&mut self) { self.ptr_buffer = Self::build_ptr_buf(&self.buffer); }
}

impl AsVecPtr for VecCString {
    /// # Examples
    ///
    /// ```
    /// use envoy::{AsVecPtr, ToVecCString};
    ///
    /// let v = ["a", "b"].to_vec_cstring().unwrap();
    /// let p = v.as_ptr();
    /// assert!(!p.is_null());
    /// // The element at index 2 (0-based) is the null terminator.
    /// assert!(unsafe { (*p.add(2)).is_null() });
    /// ```
    fn as_ptr(&self) -> *const *const c_char { self.ptr_buffer.as_ptr() }

    fn as_mut_ptr(&mut self) -> *mut *mut c_char { self.ptr_buffer.as_mut_ptr().cast() }
}

/// Converts a collection of Rust string-like values into a [`VecCString`].
///
/// This trait is primarily intended for FFI use cases where a C API expects
/// an array of `const char *`, typically constructed from multiple Rust
/// strings.
pub trait ToVecCString {
    /// Converts the value into a [`VecCString`].
    ///
    /// # Errors
    ///
    /// Returns [`EnvoyError::NulError`] if any element contains an interior
    /// NUL byte.
    fn to_vec_cstring(&self) -> Result<VecCString, EnvoyError>;
}

impl<T: ToCString> ToVecCString for [T] {
    /// # Examples
    ///
    /// ```
    /// use envoy::ToVecCString;
    ///
    /// let v = ["foo", "bar"].to_vec_cstring().unwrap();
    /// assert_eq!(v.len(), 2);
    /// ```
    fn to_vec_cstring(&self) -> Result<VecCString, EnvoyError> {
        let content = self
            .iter()
            .map(|s| s.to_cstring())
            .collect::<Result<Vec<CString>, EnvoyError>>()?;

        let ptr_buf = VecCString::build_ptr_buf(&content);
        Ok(VecCString {
            buffer: content,
            ptr_buffer: ptr_buf,
        })
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec;

    use super::*;

    #[test]
    fn test_vec_cstring_from_slice() -> mischief::Result<()> {
        let v = ["foo", "bar"].to_vec_cstring()?;
        assert_eq!(v.len(), 2);
        assert_eq!(v.as_slice()[0].to_str().unwrap(), "foo");
        assert_eq!(v.as_slice()[1].to_str().unwrap(), "bar");
        Ok(())
    }

    #[test]
    fn test_vec_cstring_from_vec() -> mischief::Result<()> {
        let v = vec![String::from("foo"), String::from("bar")].to_vec_cstring()?;
        assert_eq!(v.len(), 2);
        assert_eq!(v.as_slice()[0].to_str().unwrap(), "foo");
        assert_eq!(v.as_slice()[1].to_str().unwrap(), "bar");
        Ok(())
    }

    #[test]
    fn test_ptr_buf_null_terminated() -> mischief::Result<()> {
        let v = ["foo", "bar"].to_vec_cstring()?;
        assert_eq!(v.ptr_buffer.len(), 3);
        assert!(!v.ptr_buffer[0].is_null());
        assert!(!v.ptr_buffer[1].is_null());
        assert!(v.ptr_buffer[2].is_null());
        Ok(())
    }

    #[test]
    fn test_as_ptr_null_terminated() -> mischief::Result<()> {
        let v = ["a", "b"].to_vec_cstring()?;
        let p = v.as_ptr();
        assert!(!p.is_null());
        assert!(unsafe { (*p.add(2)).is_null() });
        Ok(())
    }

    #[test]
    fn test_push_rebuilds_ptr_buf() -> mischief::Result<()> {
        let mut v = ["foo"].to_vec_cstring()?;
        v.push(CString::new("bar").unwrap());
        assert_eq!(v.len(), 2);
        assert_eq!(v.ptr_buffer.len(), 3);
        assert!(v.ptr_buffer[2].is_null());
        Ok(())
    }

    #[test]
    fn test_clone_has_valid_ptrs() -> mischief::Result<()> {
        let original = ["foo", "bar"].to_vec_cstring()?;
        let cloned = original.clone();
        // Cloned ptr_buf must point into the clone's own content, not the
        // original's. Verify by checking the pointers are distinct.
        assert_ne!(cloned.ptr_buffer[0], original.ptr_buffer[0]);
        assert_ne!(cloned.ptr_buffer[1], original.ptr_buffer[1]);
        // And still null-terminated.
        assert!(cloned.ptr_buffer[2].is_null());
        Ok(())
    }
}
