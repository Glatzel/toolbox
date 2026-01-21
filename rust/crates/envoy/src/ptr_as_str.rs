use core::ffi::{CStr, c_char};

/// Extension trait for converting C-style string pointers or buffers
/// into Rust `&str`.
///
/// # Semantics
///
/// - Returns `None` if the pointer is null (for raw pointers)
/// - Returns `None` if the C string is not valid UTF-8
/// - The returned `&str` borrows the original memory; no allocation occurs
///
/// # Safety
///
/// For raw pointers:
/// - The pointer must be valid for reads
/// - The pointed-to memory must be NUL-terminated
/// - The memory must outlive the returned `&str`
///
/// This trait does **not** take ownership of the underlying C string.
pub trait PtrAsStr {
    /// Convert the underlying C string representation to `&str`.
    ///
    /// Returns `None` if the pointer is null or if UTF-8 validation fails.
    fn as_str(&self) -> Option<&str>;
}

impl PtrAsStr for *const c_char {
    fn as_str(&self) -> Option<&str> {
        unsafe {
            self.as_ref()
                .and_then(|_| CStr::from_ptr(*self).to_str().ok())
        }
    }
}

impl PtrAsStr for *mut c_char {
    fn as_str(&self) -> Option<&str> {
        unsafe {
            self.as_ref()
                .and_then(|_| CStr::from_ptr(*self).to_str().ok())
        }
    }
}

impl PtrAsStr for [c_char] {
    fn as_str(&self) -> Option<&str> { unsafe { CStr::from_ptr(self.as_ptr()).to_str().ok() } }
}
#[cfg(test)]
mod tests {
    use alloc::ffi::CString;
    use core::ptr;

    use super::*;

    #[test]
    fn const_ptr_valid_utf8() {
        let s = CString::new("hello").unwrap();
        let ptr = s.as_ptr();

        let out = ptr.as_str();
        assert_eq!(out, Some("hello"));
    }

    #[test]
    fn const_ptr_null() {
        let ptr: *const c_char = ptr::null();
        assert_eq!(ptr.as_str(), None);
    }

    #[test]
    fn mut_ptr_valid_utf8() {
        let s = CString::new("world").unwrap();
        let ptr = s.as_ptr().cast_mut();

        let out = ptr.as_str();
        assert_eq!(out, Some("world"));
    }

    #[test]
    fn invalid_utf8_returns_none() {
        // 0xFF is invalid UTF-8
        let bytes = [0xFFu8 as c_char, 0];
        let ptr = bytes.as_ptr();

        let binding = ptr as *const c_char;
        let out = binding.as_str();
        assert_eq!(out, None);
    }

    #[test]
    fn slice_c_char_valid() {
        let bytes = b"slice-test\0";
        let slice: &[c_char] =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const c_char, bytes.len()) };

        let out = slice.as_str();
        assert_eq!(out, Some("slice-test"));
    }
}
