extern crate alloc;
use alloc::string::ToString;

use rax::error::RuleError;
use rax::string::{IGlobalRule, IRule};

pub struct NmeaGsvLineCount;
impl IRule for NmeaGsvLineCount {}
impl<'a> IGlobalRule<'a> for NmeaGsvLineCount {
    type Output = u8;

    fn apply(&self, input: &'a str) -> Result<Self::Output, RuleError> {
        let s = input.split(',').nth(1).ok_or_else(|| RuleError {
            reason: "missing second field".to_string(),
        })?;
        s.parse::<u8>().map_err(|_| RuleError {
            reason: "invalid second field".to_string(),
        })
    }
}
