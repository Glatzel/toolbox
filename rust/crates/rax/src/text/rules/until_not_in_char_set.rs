use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{AsciiCharSetFilter, CharSetFilter, ICharSetFilter, IFilter};
use crate::text::rules::UntilMode;

/// Rule that extracts a prefix from the input string until the first
/// character that does not belong to the specified character set.
///
/// `UntilNotInCharSet` scans the input from the start, consuming consecutive
/// characters that belong to the set defined by `filter`.
///
/// When the first non-matching character is found, the rule uses `mode` to
/// determine how the input is split and returns
/// `Ok((prefix, consumed_bytes))`, where `prefix` is the extracted substring
/// and `consumed_bytes` is the number of bytes consumed according to the
/// selected mode.
///
/// If every character belongs to the set, the rule returns
/// `Ok((input, input.len()))`, consuming the entire input. An empty input
/// also succeeds, returning an empty prefix and consuming zero bytes.
///
/// # Fields
///
/// - `filter`: The character set filter that defines which characters match.
/// - `mode`: Determines how the first non-matching character is handled:
///   - [`UntilMode::Discard`]: Excludes the character from the prefix and
///     consumes it.
///   - [`UntilMode::KeepInOutput`]: Includes the character at the end of the
///     prefix.
///   - [`UntilMode::KeepInRest`]: Leaves the character at the beginning of the
///     remainder.
///
/// # Type Parameters
///
/// - `'f`: Lifetime of the character set filter reference.
/// - `IS_ASCII`: Whether the input is assumed to contain only ASCII characters.
/// - `N`: Capacity or size parameter of the character set filter.
/// - `F`: Character set filter implementing [`ICharSetFilter`].
///
/// The Unicode implementation scans Unicode scalar values and respects UTF-8
/// character boundaries. The ASCII implementations scan bytes for efficiency
/// and assume that the input contains only ASCII characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UntilNotInCharSet<'f, const IS_ASCII: bool, const N: usize, F: ICharSetFilter<N>> {
    pub filter: &'f F,
    pub mode: UntilMode,
}

impl<const N: usize, const IS_ASCII: bool, F: ICharSetFilter<N>> IRule
    for UntilNotInCharSet<'_, IS_ASCII, N, F>
{
}

impl<const N: usize, F: ICharSetFilter<N>> IFlowRule<false> for UntilNotInCharSet<'_, false, N, F> {
    type Output<'a> = &'a str;

    /// Applies the rule using Unicode character scanning.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if a non-matching character is found,
    ///   with the split determined by the selected mode.
    /// - `Ok((input, input.len()))` if every character belongs to the set.
    ///
    /// An empty input succeeds with an empty prefix and consumes zero bytes.
    /// The consumed byte count depends on the selected mode and the UTF-8
    /// length of the first non-matching character.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        for (i, c) in input.char_indices() {
            if !self.filter.filter(&c) {
                return Ok(self.mode.split_str(input, i, c.len_utf8()));
            }
        }

        Ok((input, input.len()))
    }
}

impl<const N: usize> IFlowRule<true> for UntilNotInCharSet<'_, true, N, AsciiCharSetFilter<N>> {
    type Output<'a> = &'a str;

    /// Applies the rule using byte scanning and an ASCII character set.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if a non-matching byte is found, with
    ///   the split determined by the selected mode.
    /// - `Ok((input, input.len()))` if every byte belongs to the set.
    ///
    /// The input must contain only ASCII characters for byte-based matching
    /// to correspond to character-based matching.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        for (i, &b) in input.as_bytes().iter().enumerate() {
            if self.filter.mask() & (1_u128 << u32::from(b)) == 0 {
                return Ok(self.mode.split_str(input, i, 1));
            }
        }

        Ok((input, input.len()))
    }
}

impl<const N: usize> IFlowRule<true> for UntilNotInCharSet<'_, true, N, CharSetFilter<N>> {
    type Output<'a> = &'a str;

    /// Applies the rule using byte scanning with a general character set
    /// filter.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if a non-matching byte is found, with
    ///   the split determined by the selected mode.
    /// - `Ok((input, input.len()))` if every byte belongs to the set.
    ///
    /// The input must contain only ASCII characters because each byte is
    /// converted directly to a `char` before filtering.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        for (i, &b) in input.as_bytes().iter().enumerate() {
            let c = b as char;

            if !self.filter.filter(&c) {
                return Ok(self.mode.split_str(input, i, 1));
            }
        }

        Ok((input, input.len()))
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
