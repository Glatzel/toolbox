use std::ffi::CString;
use std::ptr;

use crate::ToCString;
pub trait AsVecPtr {
    fn as_vec_ptr(&self) -> Vec<*const i8>;
}
pub struct VecCString {
    content: Vec<CString>,
}
impl VecCString {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }
}
impl AsVecPtr for VecCString {
    fn as_vec_ptr(&self) -> Vec<*const i8> {
        let mut vec_ptr = self
            .content
            .iter()
            .map(|s| s.as_ptr())
            .collect::<Vec<*const i8>>();
        vec_ptr.push(ptr::null());
        vec_ptr
    }
}
impl<T: ToCString> From<&[T]> for VecCString {
    fn from(value: &[T]) -> Self {
        VecCString {
            content: value
                .iter()
                .map(|s| s.to_cstring())
                .collect::<Vec<CString>>(),
        }
    }
}
impl<T: ToCString> From<Vec<T>> for VecCString {
    fn from(value: Vec<T>) -> Self {
        VecCString {
            content: value
                .iter()
                .map(|s| s.to_cstring())
                .collect::<Vec<CString>>(),
        }
    }
}
impl<T: ToCString> From<Option<Vec<T>>> for VecCString {
    fn from(value: Option<Vec<T>>) -> Self {
        value.as_deref().map_or_else(Self::new, |s| Self::from(s))
    }
}

impl<T: ToCString> From<Option<&[T]>> for VecCString {
    fn from(value: Option<&[T]>) -> Self {
        value.as_deref().map_or_else(Self::new, |s| Self::from(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_cstring_from_slice() {
        let arr = ["foo", "bar"];
        let vec_cstring = VecCString::from(&arr[..]);
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
    }

    #[test]
    fn test_vec_cstring_from_vec() {
        let arr = vec!["foo".to_string(), "bar".to_string()];
        let vec_cstring = VecCString::from(arr.clone());
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
    }

    #[test]
    fn test_vec_cstring_from_option_vec_some() {
        let arr = Some(vec!["foo", "bar"]);
        let vec_cstring = VecCString::from(arr);
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
    }

    #[test]
    fn test_vec_cstring_from_option_vec_none() {
        let arr: Option<Vec<&str>> = None;
        let vec_cstring = VecCString::from(arr);
        assert_eq!(vec_cstring.content.len(), 0);
    }

    #[test]
    fn test_vec_cstring_from_option_slice_some() {
        let arr = ["foo", "bar"];
        let arr_opt = Some(&arr[..]);
        let vec_cstring = VecCString::from(arr_opt);
        assert_eq!(vec_cstring.content.len(), 2);
        assert_eq!(vec_cstring.content[0].to_str().unwrap(), "foo");
        assert_eq!(vec_cstring.content[1].to_str().unwrap(), "bar");
    }

    #[test]
    fn test_vec_cstring_from_option_slice_none() {
        let arr: Option<&[&str]> = None;
        let vec_cstring = VecCString::from(arr);
        assert_eq!(vec_cstring.content.len(), 0);
    }

    #[test]
    fn test_as_vec_ptr_null_terminated() {
        let arr = ["foo", "bar"];
        let vec_cstring = VecCString::from(&arr[..]);
        let ptrs = vec_cstring.as_vec_ptr();
        assert_eq!(ptrs.len(), 3); // 2 + null
        assert!(!ptrs[0].is_null());
        assert!(!ptrs[1].is_null());
        assert!(ptrs[2].is_null());
    }
}
