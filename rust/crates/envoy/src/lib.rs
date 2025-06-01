use std::ffi::{CStr, CString};
use std::ptr;

use miette::IntoDiagnostic;
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
    /// Converts the Rust string to a `CString`.
    /// Returns an error if the string contains interior null bytes.
    fn to_cstr(&self) -> miette::Result<*const i8>;
}

impl ToCStr for &str {
    fn to_cstr(&self) -> miette::Result<*const i8> {
        Ok(CString::new(*self).into_diagnostic()?.into_raw())
    }
}

impl ToCStr for String {
    fn to_cstr(&self) -> miette::Result<*const i8> {
        Ok(CString::new(self.as_str()).into_diagnostic()?.into_raw())
    }
}

impl ToCStr for Option<&str> {
    fn to_cstr(&self) -> miette::Result<*const i8> {
        match self {
            Some(s) => Ok(CString::new(*s).into_diagnostic()?.into_raw()),
            None => Ok(ptr::null()),
        }
    }
}

impl ToCStr for Option<String> {
    fn to_cstr(&self) -> miette::Result<*const i8> {
        match self {
            Some(s) => Ok(CString::new(s.as_str()).into_diagnostic()?.into_raw()),
            None => Ok(ptr::null()),
        }
    }
}
/// Trait for converting Rust strings to `CString`.
pub trait ToCString {
    /// Converts the Rust string to a `CString`.
    /// Returns an error if the string contains interior null bytes.
    fn to_cstring(&self) -> miette::Result<CString>;
}

impl ToCString for &str {
    fn to_cstring(&self) -> miette::Result<CString> { CString::new(*self).into_diagnostic() }
}

impl ToCString for String {
    fn to_cstring(&self) -> miette::Result<CString> { CString::new(self.as_str()).into_diagnostic() }
}

impl ToCString for Option<&str> {
    fn to_cstring(&self) -> miette::Result<CString> {
        CString::new(self.unwrap_or_default()).into_diagnostic()
    }
}

impl ToCString for Option<String> {
    fn to_cstring(&self) -> miette::Result<CString> {
        match self {
            Some(s) => CString::new(s.as_str()).into_diagnostic(),
            None => Ok(CString::default()),
        }
    }
}
#[cfg(test)]
mod tests {
    // use super::*;
}
