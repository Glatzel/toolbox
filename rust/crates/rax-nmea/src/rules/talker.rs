extern crate alloc;
use alloc::string::ToString;
use core::str::FromStr;

use rax::string::{IGlobalRule, IRule};

use crate::RaxNmeaError;
use crate::data::Talker;

pub struct NmeaTalker;

impl IRule for NmeaTalker {}
impl<'a> IGlobalRule<'a> for NmeaTalker {
    type Output = Result<Talker, RaxNmeaError>;
    fn apply(&self, input: &'a str) -> Self::Output {
        let s = input
            .get(1..3)
            .ok_or(RaxNmeaError::InvalidSentenceLength(input.len()))?;
        match Talker::from_str(s) {
            Ok(talker) => Ok(talker),
            Err(_) => Err(RaxNmeaError::UnknownIdentifier(s.to_string())),
        }
    }
}
