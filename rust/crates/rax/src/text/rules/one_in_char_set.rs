use core::fmt::Debug;

use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::ICharSetFilter;

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
pub struct OneOfCharSet<'f, const N: usize, F: ICharSetFilter<N>, const IS_ASCII: bool>(pub &'f F);

impl<'f, const N: usize, F: ICharSetFilter<N>, const IS_ASCII: bool> IRule
    for OneOfCharSet<'f, N, F, IS_ASCII>
{
}

impl<'f, const N: usize, F: ICharSetFilter<N>> IFlowRule<true> for OneOfCharSet<'f, N, F, true> {
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
impl<'f, const N: usize, F: ICharSetFilter<N>> IFlowRule<false> for OneOfCharSet<'f, N, F, false> {
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
    use crate::text::filters::{CHAR_SET_ASCII_LETTERS_DIGITS, CHAR_SET_DIGITS, CharSetFilter};

    test_rule!(
        ascii_match,
        "a123",
        OneOfCharSet::<_, _, true>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        ascii_no_match,
        "abc",
        OneOfCharSet::<_, _, true>(&CHAR_SET_DIGITS)
    );

    test_rule!(
        ascii_empty_input,
        "",
        OneOfCharSet::<_, _, true>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        utf8_match,
        "你好世界",
        OneOfCharSet::<_, _, false>(&CharSetFilter::new(['你']))
    );

    test_rule!(
        utf8_no_match,
        "你好世界",
        OneOfCharSet::<_, _, false>(&CHAR_SET_DIGITS)
    );
}
