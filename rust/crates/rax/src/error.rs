use thiserror::Error;
extern crate alloc;
use alloc::string::String;
#[derive(Error, Debug)]
pub enum RaxError {
    #[error("VerbError(verb: {verb}, rule: {rule})")]
    VerbError { verb: String, rule: &'static str },
    #[error("FilterError: {0}")]
    FilterError(String),
}
