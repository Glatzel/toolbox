use core::fmt::Debug;

use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{CharSetFilter, IFilter};

/// Rule that matches the first character of the input string if it belongs to
/// a specified character set.
///
/// `OneOfCharSet<'a, N>` takes a reference to a [`CharSetFilter<N>`] and checks
/// the first character of the input. If the first character is in the set, it
/// returns a tuple `(Some(matched), rest)` where `matched` is the character and
/// `rest` is the remainder of the input. Otherwise, it returns `(None, input)`.
///
/// This rule respects UTF-8 boundaries and stops immediately on the first
/// character if it is not in the set, or if the input is empty.
///
/// # Type Parameters
///
/// - `'a`: Lifetime of the character set reference.
/// - `N`: Size of the character set (length of the `CharSetFilter`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OneOfCharSet<'f, const N: usize, const IS_ASCII: bool>(pub &'f CharSetFilter<N>);

impl<const N: usize, const IS_ASCII: bool> IRule for OneOfCharSet<'_, N, IS_ASCII> {}

impl<'f, const N: usize> IFlowRule<true> for OneOfCharSet<'f, N, true> {
    type Output<'a> = char;
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        clerk::trace!("OneOfCharSet rule: input='{}'", input);

        let b = input.as_bytes().first().ok_or_else(|| RuleError {
            reason: "empty input".into(),
        })?;

        if !self.0.filter(&(*b as char)) {
            return Err(RuleError {
                reason: "character not in set".into(),
            });
        }
        return Ok((*b as char, 1));
    }
}
impl<'f, const N: usize> IFlowRule<false> for OneOfCharSet<'f, N, false> {
    type Output<'a> = char;
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        clerk::trace!("OneOfCharSet rule: input='{}'", input);

        let c = input.chars().next().ok_or_else(|| unreachable!())?;

        if !self.0.filter(&c) {
            return Err(RuleError {
                reason: "character not in set".into(),
            });
        }
        Ok((c, c.len_utf8()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_rule;
    use crate::text::filters::{CHAR_SET_ASCII_LETTERS_DIGITS, CHAR_SET_DIGITS};

    test_rule!(
        ascii_match,
        "a123",
        OneOfCharSet::<62, true>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        ascii_no_match,
        "abc",
        OneOfCharSet::<10, true>(&CHAR_SET_DIGITS)
    );

    test_rule!(
        ascii_empty_input,
        "",
        OneOfCharSet::<62, true>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        utf8_match,
        "你好世界",
        OneOfCharSet::<1, false>(&CharSetFilter::new(['你']))
    );

    test_rule!(
        utf8_no_match,
        "你好世界",
        OneOfCharSet::<10, false>(&CHAR_SET_DIGITS)
    );
}
