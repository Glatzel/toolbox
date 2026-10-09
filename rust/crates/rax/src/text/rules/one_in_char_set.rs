use core::fmt::Debug;

use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::ICharSetFilter;

/// Rule that matches the first character of the input string if it belongs to
/// a specified character set.
///
/// `OneOfCharSet` takes a reference to an [`ICharSetFilter`] and checks
/// whether the first character of the input belongs to the set.
///
/// For `OneOfCharSet<..., true, ...>`, the first byte is interpreted as an
/// ASCII character. The input is assumed to contain only ASCII characters.
///
/// For `OneOfCharSet<..., false, ...>`, the first Unicode scalar value is
/// checked, correctly handling multi-byte UTF-8 characters.
///
/// If the first character belongs to the set, the rule returns
/// `Ok((matched, consumed_bytes))`, where:
/// - `matched` is the matched character,
/// - `consumed_bytes` is the number of bytes consumed.
///
/// If the input is empty or the first character does not belong to the set,
/// the rule returns a [`RuleError`].
///
/// # Type Parameters
///
/// - `'f`: Lifetime of the character set filter reference.
/// - `IS_ASCII`: Whether the input is assumed to contain only ASCII characters.
/// - `N`: Capacity or size parameter of the character set filter.
/// - `F`: Character set filter implementing [`ICharSetFilter`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OneOfCharSet<'f, const IS_ASCII: bool, const N: usize, F: ICharSetFilter<N>>(pub &'f F);

impl<const N: usize, F: ICharSetFilter<N>, const IS_ASCII: bool> IRule
    for OneOfCharSet<'_, IS_ASCII, N, F>
{
}

impl<const N: usize, F: ICharSetFilter<N>> IFlowRule<true> for OneOfCharSet<'_, true, N, F> {
    type Output<'a> = char;

    /// Applies the `OneOfCharSet` rule to ASCII input.
    ///
    /// # Returns
    ///
    /// - `Ok((matched, 1))` if the first byte belongs to the character set.
    /// - `Err(RuleError)` if the input is empty or the first byte does not
    ///   belong to the character set.
    ///
    /// The input must contain only ASCII characters for the consumed byte
    /// count to correspond to one character.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        let b = input.as_bytes().first().ok_or_else(|| RuleError {
            reason: "empty input".into(),
        })?;

        if !self.0.filter(&(*b as char)) {
            return Err(RuleError {
                reason: "character not in set".into(),
            });
        }

        Ok((*b as char, 1))
    }
}

impl<const N: usize, F: ICharSetFilter<N>> IFlowRule<false> for OneOfCharSet<'_, false, N, F> {
    type Output<'a> = char;

    /// Applies the `OneOfCharSet` rule to Unicode input.
    ///
    /// # Returns
    ///
    /// - `Ok((matched, consumed_bytes))` if the first character belongs to the
    ///   character set.
    /// - `Err(RuleError)` if the input is empty or the first character does not
    ///   belong to the character set.
    ///
    /// `consumed_bytes` is the UTF-8 byte length of the matched character.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        let c = input.chars().next().ok_or_else(|| RuleError {
            reason: "empty input".into(),
        })?;

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
        OneOfCharSet::<true, _, _>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        ascii_no_match,
        "abc",
        OneOfCharSet::<true, _, _>(&CHAR_SET_DIGITS)
    );

    test_rule!(
        ascii_empty_input,
        "",
        OneOfCharSet::<true, _, _>(&CHAR_SET_ASCII_LETTERS_DIGITS)
    );

    test_rule!(
        utf8_match,
        "你好世界",
        OneOfCharSet::<false, _, _>(&CharSetFilter::new(['你']))
    );

    test_rule!(
        utf8_no_match,
        "你好世界",
        OneOfCharSet::<true, _, _>(&CHAR_SET_DIGITS)
    );
}
