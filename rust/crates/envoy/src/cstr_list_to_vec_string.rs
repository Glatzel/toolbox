use crate::CStrToString;

pub trait CStrListToVecString {
    fn to_vec_string(&self) -> Vec<String>;
}

impl CStrListToVecString for *mut *mut i8 {
    fn to_vec_string(&self) -> Vec<String> {
        if self.is_null() {
            return Vec::new();
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
        vec_str
    }
}
impl CStrListToVecString for *const *const i8 {
    fn to_vec_string(&self) -> Vec<String> {
        if self.is_null() {
            return Vec::new();
        }
        let mut vec_str = Vec::new();
        let mut offset = 0;

        loop {
            let current_ptr = unsafe { self.offset(offset).as_ref().unwrap() };
            if current_ptr.is_null() {
                break;
            }
            vec_str.push(current_ptr.to_string().unwrap());
            offset += 1;
        }
        vec_str
    }
}
#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::ptr;

    use super::*;

    #[test]
    fn test_cstr_list_to_string() {
        //*mut *mut i8
        {
            // not null
            {
                let s1 = CString::new("foo").unwrap();
                let s2 = CString::new("bar").unwrap();
                let s3 = CString::new("baz").unwrap();
                let arr: [*mut i8; 4] = [
                    s1.as_ptr() as *mut i8,
                    s2.as_ptr() as *mut i8,
                    s3.as_ptr() as *mut i8,
                    std::ptr::null_mut(),
                ];
                let ptr: *const *mut i8 = arr.as_ptr();
                let result = ptr.cast_mut().to_vec_string();
                assert_eq!(
                    result,
                    vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]
                );
            }
            // null
            {
                let ptr: *mut *mut i8 = ptr::null_mut();
                assert!(ptr.is_null());
                assert!(ptr.to_vec_string().is_empty());
            }
        }
        //*const *const i8
        {
            // not null
            {
                let s1 = CString::new("foo").unwrap();
                let s2 = CString::new("bar").unwrap();
                let s3 = CString::new("baz").unwrap();
                let arr: [*const i8; 4] = [
                    s1.as_ptr() as *const i8,
                    s2.as_ptr() as *const i8,
                    s3.as_ptr() as *const i8,
                    std::ptr::null_mut(),
                ];
                let ptr: *const *const i8 = arr.as_ptr();
                let result = ptr.to_vec_string();
                assert_eq!(
                    result,
                    vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]
                );
            }
            // null
            {
                let ptr: *const *const i8 = ptr::null();
                assert!(ptr.is_null());
                assert!(ptr.to_vec_string().is_empty());
            }
        }
    }
}
