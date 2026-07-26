extern crate alloc;

use core::fmt::Debug;

use super::IStrFlowRule;
use crate::error::RuleError;
use crate::string::Decoder;
use crate::string::rules::IRule;

/// Rule that matches a specific character at the start of the input string.
///
/// `Char<C>` checks if the first character of the input string is equal to the
/// expected character `C`. If the first character matches, it returns a tuple:
/// `(Some(C), rest)` where `rest` is the remainder of the input after the
/// matched character. Otherwise, it returns `(None, input)`.
///
/// This rule respects UTF-8 character boundaries and only examines the first
/// character of the input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Char<const C: char>;

impl<const C: char> IRule for Char<C> {}

impl<'a, const C: char> IStrFlowRule<'a> for Char<C> {
    type Output = char;

    /// Applies the `Char` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `(Some(C), rest)` if the first character of the input matches `C`.
    /// - `(None, input)` if the first character does not match `C` or the input
    ///   is empty.
    ///
    /// # Logging
    ///
    /// - Trace-level logs show the input and the expected character.
    /// - Debug-level logs show whether a match occurred and the resulting rest
    ///   of the input.
    fn apply(&self, decoder: &mut Decoder<'a>) -> Result<Self::Output, RuleError> {
        let input = decoder.rest_str();
        clerk::trace!("{:?}: input='{:?}', expected='{:?}'", self, input, C);
        if input.is_empty() {
            return Err(RuleError {
                reason: "input is empty.".into(),
            });
        }

        // Fast path: ASCII comparison
        if decoder.is_ascii() {
            let expected = C as u8;

            if input.as_bytes()[0] == expected {
                clerk::debug!("{:?} matched ASCII: '{:?}'", self, C);
                decoder.advance(1);
                return Ok(C);
            }

            return Err(RuleError {
                reason: "first character does not match.".into(),
            });
        }

        // Unicode fallback
        let mut chars = input.chars();

        let first_char = chars.next().unwrap();

        if first_char == C {
            clerk::debug!("{:?} matched: '{:?}'", self, first_char);
            decoder.advance(first_char.len_utf8());
            Ok(first_char)
        } else {
            clerk::debug!(
                "{:?} did not match: found '{:?}', expected '{:?}'",
                self,
                first_char,
                C
            );

            Err(RuleError {
                reason: "first character does not match.".into(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use core::marker::PhantomData;
    use std::format;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[rstest::rstest]
    #[case("match","a123", PhantomData::<Char<'a'>>)]
    #[case("no_match","abc", PhantomData::<Char<'d'>>)]
    #[case("empty_input","", PhantomData::<Char<'a'>>)]
    #[case("unicode","你好", PhantomData::<Char<'你'>>)]
    fn test_byte_count<const C: char>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<Char<C>>,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = Decoder::new(input);
        let result = Char::<C>
            .apply(&mut decoder)
            .map(|out| (out, decoder.rest_str()));
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
