use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{AsciiCharSetFilter, CharSetFilter, ICharSetFilter, IFilter};
use crate::text::rules::UntilMode;
/// Rule that extracts a prefix from the input string consisting of consecutive
/// characters that are in the provided character set, stopping at the first
/// character not in the set.
///
/// # Fields
///
/// - `filter`: A [`CharSetFilter`] that defines the allowed characters.
/// - `mode`: Determines how the first character *not* in the set is treated:
///   - [`UntilMode::Discard`]: Exclude the first non-matching character from
///     the prefix and remove it from the rest.
///   - [`UntilMode::KeepInOutput`]: Include the first non-matching character at
///     the end of the prefix.
///   - [`UntilMode::KeepInRest`]: Keep the first non-matching character at the
///     start of the rest.
///
/// # Behavior
///
/// - Returns `(Some(prefix), rest)` when a non-matching character is found,
///   split according to `mode`.
/// - Returns `(None, input)` if all characters in the input are in the set.
/// - Respects UTF-8 character boundaries.
/// - Logs debug information at each split or if all characters are in the set.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UntilNotInCharSet<'f, const IS_ASCII: bool, const N: usize, F: ICharSetFilter<N>> {
    pub filter: &'f F,
    pub mode: UntilMode,
}

impl<const N: usize, const IS_ASCII: bool, F: ICharSetFilter<N>> IRule
    for UntilNotInCharSet<'_, IS_ASCII, N, F>
{
}

impl<'f, const N: usize, F: ICharSetFilter<N>> IFlowRule<false>
    for UntilNotInCharSet<'f, false, N, F>
{
    type Output<'a> = &'a str;

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        for (i, c) in input.char_indices() {
            if !self.filter.filter(&c) {
                return Ok(self.mode.split_str(input, i, c.len_utf8()));
            }
        }

        Ok((input, input.len()))
    }
}
impl<'f, const N: usize> IFlowRule<true> for UntilNotInCharSet<'f, true, N, AsciiCharSetFilter<N>> {
    type Output<'a> = &'a str;

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        for (i, &b) in input.as_bytes().iter().enumerate() {
            if self.filter.mask() & (1_u128 << u32::from(b)) == 0 {
                return Ok(self.mode.split_str(input, i, 1));
            }
        }
        return Ok((input, input.len()));
    }
}
impl<'f, const N: usize> IFlowRule<true> for UntilNotInCharSet<'f, true, N, CharSetFilter<N>> {
    type Output<'a> = &'a str;

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        for (i, &b) in input.as_bytes().iter().enumerate() {
            let c = b as char;
            if !self.filter.filter(&c) {
                return Ok(self.mode.split_str(input, i, 1));
            }
        }
        return Ok((input, input.len()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_rule;
    use crate::text::filters::CHAR_SET_DIGITS;

    test_rule!(
        ascii_discard,
        "123abc",
        UntilNotInCharSet::<true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_keep_left,
        "123abc",
        UntilNotInCharSet::<true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::KeepInOutput,
        }
    );

    test_rule!(
        ascii_keep_right,
        "123abc",
        UntilNotInCharSet::<true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::KeepInRest,
        }
    );

    test_rule!(
        ascii_all_in_set,
        "123456",
        UntilNotInCharSet::<true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_first_char_not_in_set,
        "a123",
        UntilNotInCharSet::<true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_empty_input,
        "",
        UntilNotInCharSet::<true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_fallback_not_in_set,
        "123abc",
        UntilNotInCharSet::<true, _, _> {
            filter: &CharSetFilter::new(['0', '1', '2', '3', '你']),
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_fallback_all_in_set,
        "123",
        UntilNotInCharSet::<true, _, _> {
            filter: &CharSetFilter::new(['你', '0', '1', '2', '3']),
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        utf8_discard,
        "你好世界",
        UntilNotInCharSet::<false, _, _> {
            filter: &CharSetFilter::new(['好', '你']),
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        utf8_all_in_set,
        "你好世界",
        UntilNotInCharSet::<false, _, _> {
            filter: &CharSetFilter::new(['你', '好', '世', '界']),
            mode: UntilMode::Discard,
        }
    );
}
