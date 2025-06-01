use std::ffi::{CStr, CString};
use std::ptr;
const CSTRING_NEW_EXPECTION: &str = "Failed to create CString";
/// Trait for converting C string pointers and slices to Rust `String`.
pub trait CStrToString {
    /// Converts the C string to a Rust `String`.
    /// Returns `None` if the pointer is null.
    fn to_string(&self) -> Option<String>;
}

impl CStrToString for *const i8 {
    /// Converts a raw C string pointer to a Rust `String`.
    /// Returns `None` if the pointer is null.
    fn to_string(&self) -> Option<String> {
        if self.is_null() {
            return None;
        }
        Some(
            unsafe { CStr::from_ptr(*self) }
                .to_string_lossy()
                .to_string(),
        )
    }
}
impl CStrToString for *mut i8 {
    /// Converts a slice of C string bytes to a Rust `String`.
    fn to_string(&self) -> Option<String> {
        Some(
            unsafe { CStr::from_ptr(*self) }
                .to_string_lossy()
                .to_string(),
        )
    }
}
impl CStrToString for [i8] {
    /// Converts a slice of C string bytes to a Rust `String`.
    fn to_string(&self) -> Option<String> {
        Some(
            unsafe { CStr::from_ptr(self.as_ptr()) }
                .to_string_lossy()
                .to_string(),
        )
    }
}

/// Trait for converting a null-terminated list of C string pointers to a
/// `Vec<String>`.
pub trait CStrListToVecString {
    /// Converts the list to a vector of Rust `String`.
    /// Returns `None` if the pointer is null.
    fn to_vec_string(&self) -> Option<Vec<String>>;
}

impl CStrListToVecString for *mut *mut i8 {
    /// Converts a null-terminated array of C string pointers to a vector of
    /// Rust `String`.
    fn to_vec_string(&self) -> Option<Vec<String>> {
        if self.is_null() {
            return None;
        }
        let mut vec_str = Vec::new();
        let mut offset = 0;

        loop {
            let current_ptr = unsafe { self.offset(offset).as_ref().unwrap() };
            if current_ptr.is_null() {
                break;
            }
            vec_str.push(current_ptr.cast_const().to_string().unwrap());
            offset += 1;
        }
        Some(vec_str)
    }
}

/// Trait for converting Rust strings to `CString`.
pub trait ToCStr {
    fn to_cstring(&self) -> CString;
    fn to_cstr(&self) -> *const i8;
}

impl ToCStr for &str {
    fn to_cstring(&self) -> CString { CString::new(*self).expect(CSTRING_NEW_EXPECTION) }
    fn to_cstr(&self) -> *const i8 { self.to_cstring().into_raw() }
}

impl ToCStr for String {
    fn to_cstring(&self) -> CString { CString::new(self.as_str()).expect(CSTRING_NEW_EXPECTION) }
    fn to_cstr(&self) -> *const i8 {
        CString::new(self.as_str())
            .expect(CSTRING_NEW_EXPECTION)
            .into_raw()
    }
}

impl ToCStr for Option<&str> {
    fn to_cstring(&self) -> CString {
        CString::new(self.unwrap_or_default()).expect(CSTRING_NEW_EXPECTION)
    }
    fn to_cstr(&self) -> *const i8 {
        match self {
            Some(s) => CString::new(*s).expect(CSTRING_NEW_EXPECTION).into_raw(),
            None => ptr::null(),
        }
    }
}

impl ToCStr for Option<String> {
    fn to_cstring(&self) -> CString {
        match self {
            Some(s) => CString::new(s.as_str()).expect(CSTRING_NEW_EXPECTION),
            None => CString::default(),
        }
    }
    fn to_cstr(&self) -> *const i8 {
        match self {
            Some(s) => CString::new(s.as_str())
                .expect(CSTRING_NEW_EXPECTION)
                .into_raw(),
            None => ptr::null(),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
