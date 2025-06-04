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
