use alloc::string::{String, ToString};
use core::ffi::{CStr, c_char};

use crate::EnvoyError;

/// Converts C-style strings (`char *`) into owned Rust [`String`] values.
///
/// This trait is intended for **FFI boundaries**, where string data is
/// represented as null-terminated C strings. Implementations perform
/// null checks where applicable and convert the underlying bytes into
/// a Rust-owned [`String`].
///
/// ## UTF-8 handling
///
/// All implementations use **lossy UTF-8 conversion**
/// ([`CStr::to_string_lossy`]):
///
/// * Invalid UTF-8 sequences are replaced with `U+FFFD` (`�`)
/// * Conversion **never fails due to encoding**
///
/// This design avoids platform-specific UTF-8 failures commonly observed
/// on Linux and macOS when consuming foreign C APIs.
///
/// ## Errors
///
/// Implementations that operate on raw pointers return:
///
/// * [`EnvoyError::NullPtr`] if the pointer is null
///
/// ## Safety
///
/// Implementations using raw pointers assume:
///
/// * The pointer is valid
/// * The memory is null-terminated
/// * The memory remains valid for the duration of the call
///
/// Violating these assumptions results in **undefined behavior**.
pub trait PtrToString {
    /// Converts the underlying C string into an owned [`String`].
    ///
    /// # Errors
    ///
    /// Returns [`EnvoyError::NullPtr`] if the underlying pointer is null
    /// (for pointer-based implementations).
    fn to_string(&self) -> Result<String, EnvoyError>;
}

/// Conversion from `*const c_char`.
///
/// This implementation:
///
/// * Checks for null
/// * Treats the pointer as a null-terminated C string
/// * Performs lossy UTF-8 conversion
impl PtrToString for *const c_char {
    fn to_string(&self) -> Result<String, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }

        // SAFETY:
        // - `self` is non-null (checked above)
        // - Caller guarantees a valid, null-terminated C string
        unsafe { Ok(CStr::from_ptr(*self).to_string_lossy().to_string()) }
    }
}

/// Conversion from `*mut c_char`.
///
/// Semantically identical to the `*const c_char` implementation.
/// Mutability is ignored; the data is treated as read-only.
///
/// This exists for convenience when consuming C APIs that return
/// mutable string pointers.
impl PtrToString for *mut c_char {
    fn to_string(&self) -> Result<String, EnvoyError> {
        if self.is_null() {
            return Err(EnvoyError::NullPtr);
        }

        // SAFETY:
        // - `self` is non-null
        // - Caller guarantees valid, null-terminated memory
        unsafe { Ok(CStr::from_ptr(*self).to_string_lossy().to_string()) }
    }
}

/// Conversion from a C character slice (`[c_char]`).
///
/// The slice **must** contain a null terminator (`\0`).
/// The conversion reads from the slice's pointer until the first
/// null byte is encountered.
///
/// ## Safety
///
/// This implementation assumes:
///
/// * The slice points to valid memory
/// * The slice is null-terminated
///
/// Failure to meet these conditions results in undefined behavior.
///
/// ## Note
///
/// No null-pointer check is performed because slices are always non-null.
impl PtrToString for [c_char] {
    fn to_string(&self) -> Result<String, EnvoyError> {
        // SAFETY:
        // - Slice pointer is non-null by construction
        // - Caller guarantees a terminating NUL byte
        unsafe { Ok(CStr::from_ptr(self.as_ptr()).to_string_lossy().to_string()) }
    }
}

#[cfg(test)]
mod tests {

    use alloc::ffi::CString;
    use core::ptr;

    use super::*;

    #[test]
    fn const_ptr_to_string() -> mischief::Result<()> {
        let s = CString::new("hello").unwrap();
        let ptr = s.as_ptr();

        let out = ptr.to_string()?;
        assert_eq!(out, "hello");
        Ok(())
    }

    #[test]
    fn mut_ptr_to_string() -> mischief::Result<()> {
        let s = CString::new("world").unwrap();
        let ptr = s.as_ptr().cast_mut();

        let out = ptr.to_string()?;
        assert_eq!(out, "world");
        Ok(())
    }

    #[test]
    fn null_ptr_returns_none() {
        let ptr: *const c_char = ptr::null();
        assert!(ptr.to_string().is_err());
    }

    #[test]
    fn slice_c_char_to_string() -> mischief::Result<()> {
        let bytes = b"slice-owned\0";
        let slice: &[c_char] =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const c_char, bytes.len()) };

        let out = slice.to_string()?;
        assert_eq!(out, "slice-owned");
        Ok(())
    }

    #[test]
    fn returned_string_is_owned() {
        let s = CString::new("owned").unwrap();
        let ptr = s.as_ptr();

        let out = ptr.to_string().unwrap();
        drop(s);

        // The String must remain valid after the source is dropped
        assert_eq!(out, "owned");
    }
}
