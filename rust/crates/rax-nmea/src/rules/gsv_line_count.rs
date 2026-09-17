use rax::error::RuleError;
use rax::text::{IGlobalRule, IRule};

pub struct NmeaGsvLineCount;
impl IRule for NmeaGsvLineCount {}
impl IGlobalRule<true> for NmeaGsvLineCount {
    type Output<'a> = u8;

    fn apply<'a>(&self, input: &'a str) -> Result<Self::Output<'a>, RuleError> {
        let s = input.split(',').nth(1).ok_or_else(|| RuleError {
            reason: "missing second field".into(),
        })?;
        s.parse::<u8>().map_err(|_| RuleError {
            reason: "invalid second field".into(),
        })
    }
}
