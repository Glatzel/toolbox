use alloc::string::{String, ToString};
use core::ffi::{CStr, c_char};
use core::str::Utf8Error;

/// Converts C-style character pointers or buffers into a Rust `String`.
///
/// This trait provides a convenience method for converting null-terminated
/// C strings (`char *`, `const char *`, or raw buffers) into owned Rust
/// `String` values.
///
/// # UTF-8 Requirement
///
/// The underlying C string **must** be valid UTF-8. If it is not, the
/// conversion will fail with [`Utf8Error`].
///
/// # Safety
///
/// Although this trait exposes a safe API, all implementations internally
/// rely on `unsafe` operations. The caller must ensure:
///
/// - The pointer is non-null
/// - The pointer is valid for reads
/// - The data is null-terminated
/// - The memory remains valid for the duration of the call
///
/// Violating any of these requirements results in **undefined behavior**.
///
/// # Typical Use Case
///
/// This trait is intended for FFI bindings where C APIs return raw `char *`
/// pointers that represent UTF-8 encoded strings.
pub trait PtrToString {
    /// Converts the underlying C string into a Rust `String`.
    ///
    /// # Errors
    ///
    /// Returns [`Utf8Error`] if the C string contains invalid UTF-8.
    fn to_string(&self) -> Result<String, Utf8Error>;
}

/// Implementation for `*const c_char` (e.g. `const char *`).
///
/// # Safety
///
/// The pointer must reference a valid, null-terminated C string.
impl PtrToString for *const c_char {
    fn to_string(&self) -> Result<String, Utf8Error> {
        unsafe { Ok(CStr::from_ptr(*self).to_str()?.to_string()) }
    }
}

/// Implementation for `*mut c_char` (e.g. `char *`).
///
/// # Safety
///
/// The pointer must reference a valid, null-terminated C string.
/// Mutability is ignored; the data is read-only.
impl PtrToString for *mut c_char {
    fn to_string(&self) -> Result<String, Utf8Error> {
        unsafe { Ok(CStr::from_ptr(*self).to_str()?.to_string()) }
    }
}

/// Implementation for a C character slice.
///
/// The slice is assumed to contain a null-terminated C string.
/// The terminator does **not** need to be within bounds of the slice,
/// but the underlying memory must be valid until a `'\0'` is found.
///
/// # Safety
///
/// - `self.as_ptr()` must point to a valid C string
/// - The string must be null-terminated
impl PtrToString for [c_char] {
    fn to_string(&self) -> Result<String, Utf8Error> {
        unsafe { Ok(CStr::from_ptr(self.as_ptr()).to_str()?.to_string()) }
    }
}

#[cfg(test)]
mod tests {
    use alloc::ffi::CString;

    use super::*;
    #[test]
    fn test_cstr_to_string() -> mischief::Result<()> {
        let s = CString::new("foo").unwrap();

        //*const i8
        {
            let ptr: *const c_char = s.as_ptr();
            assert_eq!(ptr.to_string().unwrap(), "foo");
        }
        //*mut i8
        {
            let ptr: *mut c_char = s.as_ptr().cast_mut();
            assert_eq!(ptr.to_string().unwrap(), "foo");
        }
        Ok(())
    }
}
