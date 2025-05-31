use std::ffi::{CStr, CString};

use miette::IntoDiagnostic;
pub trait CstrToString {
    fn to_string(&self) -> Option<String>;
}
impl CstrToString for *const i8 {
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
impl CstrToString for [i8] {
    fn to_string(&self) -> Option<String> {
        Some(
            unsafe { CStr::from_ptr(self.as_ptr()) }
                .to_string_lossy()
                .to_string(),
        )
    }
}

pub trait CstrListToVecString {
    fn to_vec_string(&self) -> Option<Vec<String>>;
}
impl CstrListToVecString for *mut *mut i8 {
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

pub trait ToCString {
    fn to_cstring(&self) -> miette::Result<CString>;
}
impl ToCString for &str {
    fn to_cstring(&self) -> miette::Result<CString> { CString::new(*self).into_diagnostic() }
}
impl ToCString for String {
    fn to_cstring(&self) -> miette::Result<CString> {
        CString::new(self.as_str()).into_diagnostic()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cstr_to_string_ptr() {
        let s = CString::new("hello").unwrap();
        let ptr = s.as_ptr();
        assert_eq!(ptr.to_string().as_deref(), Some("hello"));
    }

    #[test]
    fn test_cstr_to_string_ptr_null() {
        let ptr: *const i8 = std::ptr::null();
        assert_eq!(ptr.to_string(), None);
    }

    #[test]
    fn test_cstr_to_string_slice() {
        let s = CString::new("world").unwrap();
        let bytes = s.as_bytes_with_nul();
        let mut arr = [0i8; 6];
        for (i, b) in bytes.iter().enumerate() {
            arr[i] = *b as i8;
        }
        assert_eq!(arr[..].to_string().as_deref(), Some("world"));
    }

    #[test]
    fn test_cstr_list_to_vec_string() {
        let s1 = CString::new("foo").unwrap();
        let s2 = CString::new("bar").unwrap();
        let s3 = CString::new("baz").unwrap();
        let arr = vec![
            s1.as_ptr() as *mut i8,
            s2.as_ptr() as *mut i8,
            s3.as_ptr() as *mut i8,
            std::ptr::null_mut(),
        ];
        let ptr = arr.as_ptr();
        let result = ptr.cast_mut().to_vec_string();
        assert_eq!(
            result,
            Some(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string()
            ])
        );
    }

    #[test]
    fn test_cstr_list_to_vec_string_null() {
        let ptr: *mut *mut i8 = std::ptr::null_mut();
        assert_eq!(ptr.to_vec_string(), None);
    }

    #[test]
    fn test_to_cstring_str() {
        let s = "abc";
        let cstr = s.to_cstring().unwrap();
        assert_eq!(cstr.to_str().unwrap(), "abc");
    }

    #[test]
    fn test_to_cstring_string() {
        let s = String::from("xyz");
        let cstr = s.to_cstring().unwrap();
        assert_eq!(cstr.to_str().unwrap(), "xyz");
    }

    #[test]
    fn test_to_cstring_str_with_nul() {
        let s = "a\0b";
        assert!(s.to_cstring().is_err());
    }

    #[test]
    fn test_cstr_list_to_vec_string_empty() {
        let arr: Vec<*mut i8> = vec![std::ptr::null_mut()];
        let ptr = arr.as_ptr().cast_mut();
        let result = ptr.to_vec_string();
        assert_eq!(result, Some(vec![]));
    }

    #[test]
    fn test_cstr_to_string_slice_empty() {
        let arr = [0i8; 1];
        assert_eq!(arr[..].to_string().as_deref(), Some(""));
    }
}
