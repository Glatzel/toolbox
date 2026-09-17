use core::str::FromStr;

use rax::error::RuleError;
use rax::text::{IGlobalRule, IRule};

use crate::common::Talker;

pub struct NmeaTalker;

impl IRule for NmeaTalker {}
impl IGlobalRule for NmeaTalker {
    type Output<'a> = Talker;

    fn apply<'a>(&self, input: &'a str) -> Result<Self::Output<'a>, RuleError> {
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
