extern crate alloc;
use alloc::string::ToString;
use core::str::FromStr;

use rax::error::RuleError;
use rax::string::{IGlobalRule, IRule};

use crate::common::Identifier;

pub struct NmeaIdentifier;

impl IRule for NmeaIdentifier {}
impl<'a> IGlobalRule<'a> for NmeaIdentifier {
    type Output = Identifier;

    fn apply(&self, input: &'a str) -> Result<Self::Output, RuleError> {
        let s = input.get(3..6).ok_or(RuleError {
            reason: "missing identifier".to_string(),
        })?;
        match Identifier::from_str(s) {
            Ok(ident) => Ok(ident),
            Err(_) => Err(RuleError {
                reason: "unknown identifier".to_string(),
            }),
        }
    }
}
