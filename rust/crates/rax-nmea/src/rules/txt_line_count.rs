use rax::error::RuleError;
use rax::text::{IGlobalRule, IRule};

pub struct NmeaTxtLineCount;
impl IRule for NmeaTxtLineCount {}
impl IGlobalRule<true> for NmeaTxtLineCount {
    type Output<'a> = u8;

    fn apply<'a>(&self, input: &'a str) -> Result<Self::Output<'a>, RuleError> {
        let s = input.split(',').nth(1).ok_or_else(|| RuleError {
            reason: "missing line count".into(),
        })?;
        s.parse::<u8>().map_err(|_| RuleError {
            reason: "invalid line count".into(),
        })
    }
}
