extern crate alloc;

use core::fmt::Debug;

use super::IStrFlowRule;
use crate::error::RuleError;
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
    fn apply(&self, input: &'a str, _is_ascii: bool) -> Result<(Self::Output, &'a str), RuleError> {
        clerk::trace!("{:?}: input='{:?}', expected='{:?}'", self, input, C);
        match input.find(C) {
            Some(idx) => Ok((C, &input[idx + C.len_utf8()..])),
            None => Err(RuleError {
                reason: "input is empty or does not contain the expected character.".into(),
            }),
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
    #[case("ascii_match","a123", PhantomData::<Char<'a'>>)]
    #[case("ascii_no_match","abc", PhantomData::<Char<'d'>>)]
    #[case("ascii_empty_input","", PhantomData::<Char<'a'>>)]
    #[case("utf8_match","你好", PhantomData::<Char<'你'>>)]
    fn test_byte_count<const C: char>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<Char<C>>,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let result = Char::<C>.apply(input, input.is_ascii());
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
