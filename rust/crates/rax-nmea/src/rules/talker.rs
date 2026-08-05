use core::str::FromStr;

use rax::error::RuleError;
use rax::string::{IGlobalRule, IRule};

use crate::common::Talker;

pub struct NmeaTalker;

impl IRule for NmeaTalker {}
impl<'a> IGlobalRule<'a> for NmeaTalker {
    type Output = Talker;

    fn apply(&self, input: &'a str) -> Result<Self::Output, RuleError> {
        let s = input.get(1..3).ok_or_else(|| RuleError {
            reason: "missing talker".into(),
        })?;
        Talker::from_str(s).map_or_else(
            |_| {
                Err(RuleError {
                    reason: "unknown talker".into(),
                })
            },
            Ok,
        )
    }
}
