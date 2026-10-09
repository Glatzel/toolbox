use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{AsciiCharSetFilter, CharSetFilter, ICharSetFilter, IFilter};
use crate::text::rules::UntilMode;

/// Rule that extracts a prefix from the input string up to the first
/// occurrence of a character belonging to the specified character set.
///
/// `UntilOneInCharSet` scans the input from the start and searches for the
/// first character accepted by `filter`.
///
/// When a matching character is found, the rule uses `mode` to determine
/// how the input is split and returns `Ok((prefix, consumed_bytes))`, where
/// `prefix` is the extracted substring and `consumed_bytes` is the number
/// of bytes consumed according to the selected mode.
///
/// If no matching character is found, the rule returns a [`RuleError`].
///
/// # Fields
///
/// - `filter`: The character set filter that defines which characters match.
/// - `mode`: Determines how the first matching character is handled:
///   - [`UntilMode::Discard`]: Excludes the matched character from the prefix
///     and consumes it.
///   - [`UntilMode::KeepInOutput`]: Includes the matched character at the end
///     of the prefix.
///   - [`UntilMode::KeepInRest`]: Leaves the matched character at the beginning
///     of the remainder.
///
/// # Type Parameters
///
/// - `'f`: Lifetime of the character set filter reference.
/// - `IS_ASCII`: Whether the input is assumed to contain only ASCII characters.
/// - `N`: Capacity or size parameter of the character set filter.
/// - `F`: Character set filter implementing [`ICharSetFilter`].
///
/// The Unicode implementation scans Unicode scalar values and respects UTF-8
/// character boundaries. The ASCII implementation scans bytes using a bitmask
/// and assumes that the input contains only ASCII characters.
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

    /// Applies the rule using Unicode character scanning.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if a matching character is found, with
    ///   the split determined by the selected mode.
    /// - `Err(RuleError)` if no matching character is found.
    ///
    /// The consumed byte count depends on the selected mode and the UTF-8
    /// length of the matched character.
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

    /// Applies the rule using byte scanning and an ASCII character set.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if a matching byte is found, with the
    ///   split determined by the selected mode.
    /// - `Err(RuleError)` if no matching byte is found.
    ///
    /// The input must contain only ASCII characters for byte-based matching
    /// to correspond to character-based matching.
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
