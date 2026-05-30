use alloc::ffi::CString;
use alloc::vec::Vec;
use core::ffi::c_char;
use core::ops::Deref;
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
    /// The pointer buffer is built lazily on the first call and cached until
    /// the [`VecCString`] is mutated.
    ///
    /// # Safety
    ///
    /// The pointer is valid only as long as the [`VecCString`] is alive and
    /// unmodified.
    fn as_ptr(&mut self) -> *const *const c_char;

    /// Returns a `*mut *mut c_char` pointing into the internal pointer buffer.
    /// The array is always null-terminated.
    ///
    /// The pointer buffer is built lazily on the first call and cached until
    /// the [`VecCString`] is mutated.
    ///
    /// # Safety
    ///
    /// The pointer is valid only as long as the [`VecCString`] is alive and
    /// unmodified. Mutating through the returned pointer must not write a
    /// pointer that outlives the underlying [`CString`].
    fn as_mut_ptr(&mut self) -> *mut *mut c_char;
}

/// A wrapper around a vector of [`CString`] that lazily builds a synchronised
/// null-terminated pointer buffer for cheap FFI hand-off.
///
/// `buffer` is private so that every mutation goes through methods that
/// invalidate `ptr_buffer`, keeping the two fields in sync at all times.
///
/// `ptr_buffer` is `None` until the first call to [`as_ptr`] or
/// [`as_mut_ptr`], after which it is cached. Any mutation sets it back to
/// `None`.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VecCString {
    buffer: Vec<CString>,
    /// Lazily-built null-terminated array of pointers into `buffer`.
    /// `None` means the cache is invalid and must be rebuilt on next access.
    /// When `Some`, length is always `buffer.len() + 1`; the last element is
    /// `ptr::null()`.
    ptr_buffer: Option<Vec<*const c_char>>,
}

// SAFETY: VecCString owns its CStrings; the raw pointers in ptr_buffer are
// derived from those owned values and are never exposed beyond the lifetime
// of &mut self.
unsafe impl Send for VecCString {}
unsafe impl Sync for VecCString {}

impl Clone for VecCString {
    fn clone(&self) -> Self {
        // Clone buffer into a fresh allocation. Do NOT copy ptr_buffer —
        // those pointers point into the original's CString heap buffers and
        // would be immediately dangling. Start with None; the clone will
        // build its own ptr_buffer lazily on first use.
        Self {
            buffer: self.buffer.clone(),
            ptr_buffer: None,
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
            ptr_buffer: None,
        }
    }

    /// Appends a [`CString`], invalidating the pointer cache.
    pub fn push(&mut self, s: CString) {
        self.ptr_buffer = None;
        self.buffer.push(s);
    }
}
impl Deref for VecCString {
    type Target = [CString];
    fn deref(&self) -> &[CString] { &self.buffer }
}

// No DerefMut — would allow silent ptr_buffer invalidation via index writes.

impl Extend<CString> for VecCString {
    fn extend<I: IntoIterator<Item = CString>>(&mut self, iter: I) {
        self.ptr_buffer = None;
        self.buffer.extend(iter);
    }
}

impl FromIterator<CString> for VecCString {
    fn from_iter<I: IntoIterator<Item = CString>>(iter: I) -> Self {
        Self {
            buffer: iter.into_iter().collect(),
            ptr_buffer: None,
        }
    }
}

impl<'a> IntoIterator for &'a VecCString {
    type Item = &'a CString;
    type IntoIter = core::slice::Iter<'a, CString>;
    fn into_iter(self) -> Self::IntoIter { self.buffer.iter() }
}

impl IntoIterator for VecCString {
    type Item = CString;
    type IntoIter = alloc::vec::IntoIter<CString>;
    fn into_iter(self) -> Self::IntoIter { self.buffer.into_iter() }
}
impl AsVecPtr for VecCString {
    /// # Examples
    ///
    /// ```
    /// use envoy::{AsVecPtr, ToVecCString};
    ///
    /// let mut v = ["a", "b"].to_vec_cstring().unwrap();
    /// let p = v.as_ptr();
    /// assert!(!p.is_null());
    /// // The element at index 2 (0-based) is the null terminator.
    /// assert!(unsafe { (*p.add(2)).is_null() });
    /// ```
    fn as_ptr(&mut self) -> *const *const c_char {
        self.ptr_buffer
            .get_or_insert_with(|| {
                self.buffer
                    .iter()
                    .map(|s| s.as_ptr())
                    .chain(core::iter::once(ptr::null()))
                    .collect()
            })
            .as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut *mut c_char {
        self.ptr_buffer
            .get_or_insert_with(|| {
                self.buffer
                    .iter()
                    .map(|s| s.as_ptr())
                    .chain(core::iter::once(ptr::null()))
                    .collect()
            })
            .as_mut_ptr()
            .cast()
    }
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
        let buffer = self
            .iter()
            .map(|s| s.to_cstring())
            .collect::<Result<Vec<CString>, EnvoyError>>()?;

        Ok(VecCString {
            buffer,
            ptr_buffer: None,
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
        assert_eq!(v[0].to_str().unwrap(), "foo");
        assert_eq!(v[1].to_str().unwrap(), "bar");
        Ok(())
    }

    #[test]
    fn test_vec_cstring_from_vec() -> mischief::Result<()> {
        let v = vec![String::from("foo"), String::from("bar")].to_vec_cstring()?;
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].to_str().unwrap(), "foo");
        assert_eq!(v[1].to_str().unwrap(), "bar");
        Ok(())
    }

    #[test]
    fn test_ptr_buf_null_terminated() -> mischief::Result<()> {
        let mut v = ["foo", "bar"].to_vec_cstring()?;
        v.as_ptr(); // trigger lazy build
        let buf = v.ptr_buffer.as_ref().unwrap();
        assert_eq!(buf.len(), 3);
        assert!(!buf[0].is_null());
        assert!(!buf[1].is_null());
        assert!(buf[2].is_null());
        Ok(())
    }

    #[test]
    fn test_as_ptr_null_terminated() -> mischief::Result<()> {
        let mut v = ["a", "b"].to_vec_cstring()?;
        let p = v.as_ptr();
        assert!(!p.is_null());
        assert!(unsafe { (*p.add(2)).is_null() });
        Ok(())
    }

    #[test]
    fn test_push_invalidates_and_rebuilds() -> mischief::Result<()> {
        let mut v = ["foo"].to_vec_cstring()?;
        v.as_ptr(); // populate cache
        v.push(CString::new("bar").unwrap()); // must invalidate
        assert!(v.ptr_buffer.is_none()); // cache cleared
        v.as_ptr(); // rebuild
        let buf = v.ptr_buffer.as_ref().unwrap();
        assert_eq!(buf.len(), 3);
        assert!(buf[2].is_null());
        Ok(())
    }

    #[test]
    fn test_clone_ptr_buffer_is_none() -> mischief::Result<()> {
        let mut original = ["foo", "bar"].to_vec_cstring()?;
        original.as_ptr(); // populate cache in original
        let cloned = original.clone();
        // Clone must not carry over the original's pointers.
        assert!(cloned.ptr_buffer.is_none());
        Ok(())
    }

    #[test]
    fn test_clone_builds_own_ptrs() -> mischief::Result<()> {
        let mut original = ["foo", "bar"].to_vec_cstring()?;
        original.as_ptr();
        let mut cloned = original.clone();
        cloned.as_ptr(); // trigger build in clone
        let orig_buf = original.ptr_buffer.as_ref().unwrap();
        let clone_buf = cloned.ptr_buffer.as_ref().unwrap();
        // Pointers must be into different heap allocations.
        assert_ne!(clone_buf[0], orig_buf[0]);
        assert_ne!(clone_buf[1], orig_buf[1]);
        assert!(clone_buf[2].is_null());
        Ok(())
    }
}
