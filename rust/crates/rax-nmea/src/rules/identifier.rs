use core::str::FromStr;

use rax::error::RuleError;
use rax::text::{IGlobalRule, IRule};

use crate::common::Identifier;

pub struct NmeaIdentifier;

impl IRule for NmeaIdentifier {}
impl IGlobalRule<true> for NmeaIdentifier {
    type Output<'a> = Identifier;

    fn apply<'a>(&self, input: &'a str) -> Result<Self::Output<'a>, RuleError> {
        let s = input.get(3..6).ok_or_else(|| RuleError {
            reason: "missing identifier".into(),
        })?;
        Identifier::from_str(s).map_or_else(
            |_| {
                Err(RuleError {
                    reason: "unknown identifier".into(),
                })
            },
            Ok,
        )
    }
}
