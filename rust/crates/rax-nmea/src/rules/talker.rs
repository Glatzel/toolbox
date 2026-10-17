use core::str::FromStr;

use rax::error::RuleError;
use rax::text::{IGlobalRule, IRule};

use crate::common::Talker;

/// Extracts the two-character talker ID.
///
/// The talker is read from bytes 1 through 2 of the sentence, immediately
/// after the `$` delimiter.
pub struct NmeaTalker;

impl IRule for NmeaTalker {}
impl IGlobalRule<true> for NmeaTalker {
    type Output<'a> = Talker;

    /// Returns the parsed [`Talker`].
    ///
    /// # Errors
    ///
    /// Returns a [`RuleError`] if the input is too short or the talker is not
    /// recognized.
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
