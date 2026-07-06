extern crate alloc;
use alloc::string::ToString;
use core::str::FromStr;

use rax::error::RuleError;
use rax::string::{IGlobalRule, IRule};

use crate::common::Talker;

pub struct NmeaTalker;

impl IRule for NmeaTalker {}
impl<'a> IGlobalRule<'a> for NmeaTalker {
    type Output = Talker;

    fn apply(&self, input: &'a str) -> Result<Self::Output, RuleError> {
        let s = input.get(1..3).ok_or(RuleError {
            reason: "missing talker".to_string(),
        })?;
        match Talker::from_str(s) {
            Ok(talker) => Ok(talker),
            Err(_) => Err(RuleError {
                reason: "unknown talker".to_string(),
            }),
        }
    }
}
