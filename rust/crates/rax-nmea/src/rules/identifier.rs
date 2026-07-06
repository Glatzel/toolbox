extern crate alloc;
use alloc::string::ToString;
use core::str::FromStr;

use rax::string::{IGlobalRule, IRule};

use crate::RaxNmeaError;
use crate::common::Identifier;

pub struct NmeaIdentifier;

impl IRule for NmeaIdentifier {}
impl<'a> IGlobalRule<'a> for NmeaIdentifier {
    type Output = Identifier;


    fn apply(&self, input: &'a str) -> Result<Self::Output, Self::Error> {
        let s = input
            .get(3..6)
            .ok_or(RaxNmeaError::InvalidSentenceLength(input.len()))?;
        match Identifier::from_str(s) {
            Ok(ident) => Ok(ident),
            Err(_) => Err(RaxNmeaError::UnknownIdentifier(s.to_string())),
        }
    }
}
