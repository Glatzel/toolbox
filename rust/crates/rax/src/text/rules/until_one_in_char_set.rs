use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{AsciiCharSetFilter, CharSetFilter, ICharSetFilter, IFilter};
use crate::text::rules::UntilMode;
/// Rule that extracts a prefix from the input string up to the first occurrence
/// of any character in the provided character set.
///
/// # Fields
///
/// - `filter`: A [`CharSetFilter`] defining the set of characters to stop at.
/// - `mode`: Determines how the matched character is treated:
///   - [`UntilMode::Discard`]: Exclude the matched character from the prefix
///     and remove it from the rest.
///   - [`UntilMode::KeepInOutput`]: Include the matched character in the
///     prefix.
///   - [`UntilMode::KeepInRest`]: Keep the matched character at the start of
///     the rest.
///
/// # Behavior
///
/// - Returns `(Some(prefix), rest)` when a character from the set is found,
///   split according to `mode`.
/// - Returns `(None, input)` if no character from the set is found.
/// - Respects UTF-8 character boundaries.
/// - Logs debug information for each split or if no match is found.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UntilOneInCharSet<'f, const IS_ASCII: bool, const N: usize, F: ICharSetFilter<N>> {
    pub filter: &'f F,
    pub mode: UntilMode,
}

impl<const IS_ASCII: bool, const N: usize, F: ICharSetFilter<N>> IRule
    for UntilOneInCharSet<'_, IS_ASCII, N, F>
{
}

impl<const N: usize> IFlowRule<false> for UntilOneInCharSet<'_, false, N, CharSetFilter<N>> {
    type Output<'a> = &'a str;

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        for (i, c) in input.char_indices() {
            if self.filter.filter(&c) {
                return Ok(self.mode.split_str(input, i, c.len_utf8()));
            }
        }

        Err(RuleError {
            reason: "no match found".into(),
        })
    }
}
impl<const N: usize> IFlowRule<true> for UntilOneInCharSet<'_, true, N, AsciiCharSetFilter<N>> {
    type Output<'a> = &'a str;

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        input
            .as_bytes()
            .iter()
            .position(|&b| self.filter.mask() & (1_u128 << u32::from(b)) != 0)
            .map_or_else(
                || {
                    Err(RuleError {
                        reason: "no match found".into(),
                    })
                },
                |i| Ok(self.mode.split_str(input, i, 1)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_rule;
    use crate::text::filters::{CHAR_SET_ASCII_LETTERS, CHAR_SET_DIGITS};

    test_rule!(
        ascii_discard,
        "abc1def",
        UntilOneInCharSet::<true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_keep_left,
        "abc1def",
        UntilOneInCharSet::<true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::KeepInOutput,
        }
    );

    test_rule!(
        ascii_keep_right_first_char,
        "a123",
        UntilOneInCharSet::<true, _, _> {
            filter: &CHAR_SET_ASCII_LETTERS,
            mode: UntilMode::KeepInRest,
        }
    );

    test_rule!(
        ascii_keep_right_not_first_char,
        "abc1def",
        UntilOneInCharSet::<true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::KeepInRest,
        }
    );

    test_rule!(
        ascii_no_match,
        "abcdef",
        UntilOneInCharSet::<true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_empty_input,
        "",
        UntilOneInCharSet::<true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );
}
