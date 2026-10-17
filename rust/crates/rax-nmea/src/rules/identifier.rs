use core::str::FromStr;

use rax::error::RuleError;
use rax::text::{IGlobalRule, IRule};

use crate::common::Identifier;

/// Extracts the three-character sentence identifier.
///
/// The identifier is read from bytes 3 through 5 of the sentence, after the
/// `$` delimiter and the two-character talker ID.
pub struct NmeaIdentifier;

impl IRule for NmeaIdentifier {}
impl IGlobalRule<true> for NmeaIdentifier {
    type Output<'a> = Identifier;

    /// Returns the parsed [`Identifier`].
    ///
    /// # Errors
    ///
    /// Returns a [`RuleError`] if the input is too short or the identifier is
    /// not recognized.
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
