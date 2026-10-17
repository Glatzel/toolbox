use rax::error::RuleError;
use rax::text::{IGlobalRule, IRule};

/// Reads the total number of GSV sentences in a multi-line GSV message.
///
/// The count is taken from the second comma-separated field of the first
/// line.
pub struct NmeaGsvLineCount;
impl IRule for NmeaGsvLineCount {}
impl IGlobalRule<true> for NmeaGsvLineCount {
    type Output<'a> = u8;

    /// Returns the GSV line count.
    ///
    /// # Errors
    ///
    /// Returns a [`RuleError`] if the second field is missing or is not a
    /// valid `u8`.
    fn apply<'a>(&self, input: &'a str) -> Result<Self::Output<'a>, RuleError> {
        let s = input.split(',').nth(1).ok_or_else(|| RuleError {
            reason: "missing second field".into(),
        })?;
        s.parse::<u8>().map_err(|_| RuleError {
            reason: "invalid second field".into(),
        })
    }
}
