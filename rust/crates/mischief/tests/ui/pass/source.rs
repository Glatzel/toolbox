use core::fmt;

use mischief::mischief;

#[derive(Debug)]
struct SourceError;

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("source error") }
}

impl core::error::Error for SourceError {}

fn main() {
    let _foreign = mischief!("foreign wrapper", source = SourceError, code = "E_SOURCE");
    let _mischief = mischief!("mischief wrapper", source = mischief!("inner"));
}
