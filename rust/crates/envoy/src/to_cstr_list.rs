use std::ffi::CString;

use crate::ToCStr;

pub trait ToCStrList {
    fn to_cstring_list(&self) -> Vec<CString>;
    fn to_cstr_list(&self) -> Vec<*const i8>;
}

impl ToCStrList for [&str] {
    fn to_cstring_list(&self) -> Vec<CString> {
        if self.is_empty() {
            return Vec::new();
        }
        self.iter().map(|s| s.to_cstring()).collect()
    }
    fn to_cstr_list(&self) -> Vec<*const i8> {
        self.iter().map(|s| s.to_cstr()).collect::<Vec<*const i8>>()
    }
}

impl ToCStrList for Vec<String> {
    fn to_cstring_list(&self) -> Vec<CString> {
        if self.is_empty() {
            return Vec::new();
        }
        self.iter().map(|s| s.to_cstring()).collect()
    }
    fn to_cstr_list(&self) -> Vec<*const i8> {
        self.iter().map(|s| s.to_cstr()).collect::<Vec<*const i8>>()
    }
}

impl ToCStrList for Option<Vec<&str>> {
    fn to_cstring_list(&self) -> Vec<CString> {
        match self {
            Some(s) => {
                if s.is_empty() {
                    return Vec::new();
                }
                s.iter().map(|l| l.to_cstring()).collect()
            }
            None => Vec::new(),
        }
    }
    fn to_cstr_list(&self) -> Vec<*const i8> {
        match self {
            Some(s) => {
                if s.is_empty() {
                    return Vec::new();
                }
                s.iter().map(|l| l.to_cstr()).collect()
            }
            None => Vec::new(),
        }
    }
}
impl ToCStrList for Option<Vec<String>> {
    fn to_cstring_list(&self) -> Vec<CString> {
        match self {
            Some(s) => {
                if s.is_empty() {
                    return Vec::new();
                }
                s.iter().map(|l| l.to_cstring()).collect()
            }
            None => Vec::new(),
        }
    }
    fn to_cstr_list(&self) -> Vec<*const i8> {
        match self {
            Some(s) => {
                if s.is_empty() {
                    return Vec::new();
                }
                s.iter().map(|l| l.to_cstr()).collect()
            }
            None => Vec::new(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::CStrToString;

    #[test]
    fn test_to_cstr_list() {
        // [&str]
        {
            let arr = ["foo", "bar", "baz"];
            let cstr_list = arr.to_cstr_list();
            assert_eq!(cstr_list.len(), 3);
            for (i, ptr) in cstr_list.iter().enumerate() {
                assert_eq!(ptr.to_string().unwrap(), arr[i]);
                assert!(!ptr.is_null());
                // SAFETY: ptr was allocated by CString::into_raw, so we must reclaim it
                unsafe {
                    let _ = CString::from_raw(*ptr as *mut i8);
                }
            }
        }
        // Vec<String>
        {
            let vec = vec!["foo".to_string(), "bar".to_string()];
            let cstr_list = vec.to_cstr_list();
            assert_eq!(cstr_list.len(), 2);
            for (i, ptr) in cstr_list.iter().enumerate() {
                assert_eq!(ptr.to_string().unwrap(), vec[i]);
                assert!(!ptr.is_null());
                unsafe {
                    let _ = CString::from_raw(*ptr as *mut i8);
                }
            }
        }
        // Option<Vec<&str>>
        {
            let opt = Some(vec!["foo", "bar"]);
            let cstr_list = opt.to_cstr_list();
            assert_eq!(cstr_list.len(), 2);
            assert_eq!(cstr_list[0].to_string().unwrap(), "foo");
            assert_eq!(cstr_list[1].to_string().unwrap(), "bar");
            for ptr in cstr_list {
                assert!(!ptr.is_null());
                unsafe {
                    let _ = CString::from_raw(ptr as *mut i8);
                }
            }
            let none: Option<Vec<&str>> = None;
            let cstr_list = none.to_cstr_list();
            assert!(cstr_list.is_empty());
        }
        // Option<Vec<String>>
        {
            let opt = Some(vec!["foo".to_string(), "bar".to_string()]);
            let cstr_list = opt.to_cstr_list();
            assert_eq!(cstr_list.len(), 2);
            assert_eq!(cstr_list[0].to_string().unwrap(), "foo");
            assert_eq!(cstr_list[1].to_string().unwrap(), "bar");
            for ptr in cstr_list {
                assert!(!ptr.is_null());
                unsafe {
                    let _ = CString::from_raw(ptr as *mut i8);
                }
            }
            let none: Option<Vec<String>> = None;
            let cstr_list = none.to_cstr_list();
            assert!(cstr_list.is_empty());
        }
        // Empty slices/vectors
        {
            let arr: [&str; 0] = [];
            assert!(arr.to_cstr_list().is_empty());
            let vec: Vec<String> = vec![];
            assert!(vec.to_cstr_list().is_empty());
            let opt: Option<Vec<&str>> = Some(vec![]);
            assert!(opt.to_cstr_list().is_empty());
            let opt: Option<Vec<String>> = Some(vec![]);
            assert!(opt.to_cstr_list().is_empty());
        }
    }
}
