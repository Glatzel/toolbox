use std::ffi::CString;
use std::ptr;

use crate::ToCStr;

pub trait ToVecCStr {
    fn to_vec_cstring(&self) -> Vec<CString>;
    fn to_vec_cstr(&self) -> Vec<*const i8>;
}

impl<T: ToCStr> ToVecCStr for [T] {
    fn to_vec_cstring(&self) -> Vec<CString> { self.iter().map(|s| s.to_cstring()).collect() }
    fn to_vec_cstr(&self) -> Vec<*const i8> {
        let mut ptrs: Vec<_> = self.iter().map(|s| s.to_cstr()).collect();
        ptrs.push(ptr::null()); // Null-terminate
        ptrs
    }
}
impl<T: ToCStr> ToVecCStr for Option<Vec<T>> {
    fn to_vec_cstring(&self) -> Vec<CString> {
        self.as_deref()
            .map_or_else(Vec::new, |s| s.to_vec_cstring())
    }
    fn to_vec_cstr(&self) -> Vec<*const i8> {
        match self {
            Some(s) => s.to_vec_cstr(),
            None => vec![ptr::null()],
        }
    }
}

impl<T: ToCStr> ToVecCStr for Option<&[T]> {
    fn to_vec_cstring(&self) -> Vec<CString> {
        self.as_deref()
            .map_or_else(Vec::new, |s| s.to_vec_cstring())
    }
    fn to_vec_cstr(&self) -> Vec<*const i8> {
        match self {
            Some(s) => s.to_vec_cstr(),
            None => vec![ptr::null()],
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
            let cstr_list = arr.to_vec_cstr();
            assert_eq!(cstr_list.len(), 4);
            for (i, s) in arr.iter().enumerate() {
                assert_eq!(cstr_list[i].to_string().unwrap(), s.to_string());
                assert!(!cstr_list[i].is_null());
            }
        }
        // Vec<String>
        {
            let arr = ["foo".to_string(), "bar".to_string()];
            let cstr_list = arr.to_vec_cstr();
            assert_eq!(cstr_list.len(), 3);
            for (i, s) in arr.iter().enumerate() {
                assert_eq!(cstr_list[i].to_string().unwrap(), s.to_string());
                assert!(!cstr_list[i].is_null());
            }
        }
        // Option<Vec<&str>>
        {
            let arr = Some(vec!["foo", "bar"]);
            let cstr_list = arr.to_vec_cstr();
            assert_eq!(cstr_list.len(), 3);
            assert_eq!(cstr_list[0].to_string().unwrap(), "foo");
            assert_eq!(cstr_list[1].to_string().unwrap(), "bar");
            for (i, s) in arr.unwrap().iter().enumerate() {
                assert_eq!(cstr_list[i].to_string().unwrap(), s.to_string());
                assert!(!cstr_list[i].is_null());
            }
            let none: Option<Vec<&str>> = None;
            let cstr_list = none.to_vec_cstr();
            assert_eq!(cstr_list, vec![ptr::null()]);
        }
        // Option<Vec<String>>
        {
            let arr = Some(vec!["foo".to_string(), "bar".to_string()]);
            let cstr_list = arr.to_vec_cstr();
            assert_eq!(cstr_list.len(), 3);
            assert_eq!(cstr_list[0].to_string().unwrap(), "foo");
            assert_eq!(cstr_list[1].to_string().unwrap(), "bar");
            for (i, s) in arr.unwrap().iter().enumerate() {
                assert_eq!(cstr_list[i].to_string().unwrap(), s.to_string());
                assert!(!cstr_list[i].is_null());
            }
            let none: Option<Vec<String>> = None;
            let cstr_list = none.to_vec_cstr();
            assert_eq!(cstr_list, vec![ptr::null()]);
        }
        // Option<[String]>
        {
            let src = vec!["foo".to_string(), "bar".to_string()];
            let arr = Some(src.as_slice());
            let cstr_list = arr.to_vec_cstr();
            assert_eq!(cstr_list.len(), 3);
            assert_eq!(cstr_list[0].to_string().unwrap(), "foo");
            assert_eq!(cstr_list[1].to_string().unwrap(), "bar");
            for (i, s) in arr.unwrap().iter().enumerate() {
                assert_eq!(cstr_list[i].to_string().unwrap(), s.to_string());
                assert!(!cstr_list[i].is_null());
            }
            let none: Option<Vec<String>> = None;
            let cstr_list = none.to_vec_cstr();
            assert_eq!(cstr_list, vec![ptr::null()]);
        }
        // Empty slices/vectors
        {
            let arr: [&str; 0] = [];
            assert_eq!(arr.to_vec_cstr(), vec![ptr::null()]);
            let arr: Vec<String> = vec![];
            assert_eq!(arr.to_vec_cstr(), vec![ptr::null()]);
            let arr: Option<Vec<&str>> = Some(vec![]);
            assert_eq!(arr.to_vec_cstr(), vec![ptr::null()]);
            let arr: Option<Vec<String>> = Some(vec![]);
            assert_eq!(arr.to_vec_cstr(), vec![ptr::null()]);
        }
    }
}
