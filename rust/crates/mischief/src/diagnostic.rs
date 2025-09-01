extern crate alloc;
use alloc::string::String;
use core::fmt::Display;

pub struct Diagnostic {
    msg: String,
}
impl Diagnostic {
    pub(crate) fn new<D>(msg: D) -> Self
    where
        D: Display + Send + Sync + 'static,
    {
        Self {
            msg: msg.to_string(),
        }
    }
    pub(crate) fn msg<'a>(&'a self) -> &'a str { self.msg.as_str() }
}
