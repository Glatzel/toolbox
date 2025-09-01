extern crate alloc;
use alloc::string::{String, ToString};
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
    pub(crate) fn msg(&self) -> &str { self.msg.as_str() }
}
