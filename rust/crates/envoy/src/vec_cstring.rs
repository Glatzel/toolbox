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
