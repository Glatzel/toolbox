extern crate alloc;
use alloc::string::ToString;

use rax::string::{IGlobalRule, IRule};

use crate::RaxNmeaError;

pub struct NmeaGsvLineCount;
impl IRule for NmeaGsvLineCount {}
impl<'a> IGlobalRule<'a> for NmeaGsvLineCount {
    type Output = u8;
    type Error = RaxNmeaError;

    fn apply(&self, input: &'a str) -> Result<Self::Output, Self::Error> {
        let s = input
            .split(',')
            .nth(1)
            .ok_or_else(|| RaxNmeaError::InvalidSentence(input.to_string()))?;
        s.parse::<u8>()
            .map_err(|_| RaxNmeaError::InvalidSentence(input.to_string()))
    }
}
