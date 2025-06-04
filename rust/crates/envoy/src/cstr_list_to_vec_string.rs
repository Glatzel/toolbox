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
