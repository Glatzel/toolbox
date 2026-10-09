use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{AsciiCharSetFilter, CharSetFilter, ICharSetFilter, IFilter};
use crate::text::rules::UntilMode;

/// Rule that extracts a prefix from the input string until the N-th character
/// matching a specified character set is encountered.
///
/// `UntilNInCharSet<N, IS_ASCII>` scans the input from the start and counts
/// characters that belong to the character set defined by `filter`.
///
/// When the N-th matching character is found, the rule uses `mode` to
/// determine how the input is split and returns
/// `Ok((prefix, consumed_bytes))`, where `prefix` is the extracted substring
/// and `consumed_bytes` is the number of bytes consumed according to the
/// selected mode.
///
/// If fewer than `N` matching characters are found, the rule returns a
/// [`RuleError`]. When `N` is zero, the rule succeeds with an empty prefix
/// and consumes zero bytes.
///
/// # Fields
///
/// - `filter`: The character set filter that defines which characters match.
/// - `mode`: Determines how the N-th matching character is handled:
///   - [`UntilMode::Discard`]: Excludes the matching character from the prefix
///     and consumes it.
///   - [`UntilMode::KeepInOutput`]: Includes the matching character at the end
///     of the prefix.
///   - [`UntilMode::KeepInRest`]: Leaves the matching character at the
///     beginning of the remainder.
///
/// # Type Parameters
///
/// - `'f`: Lifetime of the character set filter reference.
/// - `N`: Number of matching characters required to stop scanning.
/// - `IS_ASCII`: Whether the input is assumed to contain only ASCII characters.
/// - `F`: Character set filter implementing [`ICharSetFilter`].
/// - `N_CHAR_SET`: Capacity or size parameter of the character set filter.
///
/// The ASCII implementation scans bytes using a bitmask, while the Unicode
/// implementation scans Unicode scalar values and respects UTF-8 boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UntilNInCharSet<
    'f,
    const N: usize,
    const IS_ASCII: bool,
    F: ICharSetFilter<N_CHAR_SET>,
    const N_CHAR_SET: usize,
> {
    pub filter: &'f F,
    pub mode: UntilMode,
}

impl<const N: usize, F: ICharSetFilter<N_CHAR_SET>, const N_CHAR_SET: usize, const IS_ASCII: bool>
    IRule for UntilNInCharSet<'_, N, IS_ASCII, F, N_CHAR_SET>
{
}

impl<const N: usize, const N_CHAR_SET: usize> IFlowRule<true>
    for UntilNInCharSet<'_, N, true, AsciiCharSetFilter<N_CHAR_SET>, N_CHAR_SET>
{
    type Output<'a> = &'a str;

    /// Applies the rule using byte-based scanning and an ASCII character set.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if the N-th matching byte is found.
    /// - `Err(RuleError)` if fewer than `N` matching bytes are found.
    ///
    /// For `N == 0`, returns an empty prefix and consumes zero bytes.
    ///
    /// The input must contain only ASCII characters for byte-based matching
    /// to correspond to character-based matching.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            return Ok(("", 0));
        }

        let mut remaining = N;

        for (idx, &b) in input.as_bytes().iter().enumerate() {
            if self.filter.mask() & (1_u128 << u32::from(b)) != 0 {
                remaining -= 1;

                if remaining == 0 {
                    return Ok(self.mode.split_str(input, idx, 1));
                }
            }
        }

        Err(RuleError {
            reason: "fewer than N matches found".into(),
        })
    }
}

impl<const N: usize, const N_CHAR_SET: usize> IFlowRule<false>
    for UntilNInCharSet<'_, N, false, CharSetFilter<N_CHAR_SET>, N_CHAR_SET>
{
    type Output<'a> = &'a str;

    /// Applies the rule using Unicode character scanning.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if the N-th matching character is
    ///   found.
    /// - `Err(RuleError)` if fewer than `N` matching characters are found.
    ///
    /// For `N == 0`, returns an empty prefix and consumes zero bytes.
    ///
    /// Matching is performed on Unicode scalar values rather than grapheme
    /// clusters. The consumed byte count depends on the selected mode and
    /// the UTF-8 length of the N-th matching character.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            return Ok(("", 0));
        }

        let mut remaining = N;

        for (idx, ch) in input.char_indices() {
            if self.filter.filter(&ch) {
                remaining -= 1;

                if remaining == 0 {
                    return Ok(self.mode.split_str(input, idx, ch.len_utf8()));
                }
            }
        }

        Err(RuleError {
            reason: "fewer than N matches found".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_rule;
    use crate::text::filters::CHAR_SET_DIGITS;
    test_rule!(
        zero_n,
        "a1b2c3",
        UntilNInCharSet::<0, true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_discard,
        "a1b2c3",
        UntilNInCharSet::<2, true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_keep_left,
        "a1b2c3",
        UntilNInCharSet::<2, true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::KeepInOutput,
        }
    );

    test_rule!(
        ascii_keep_right,
        "a1b2c3",
        UntilNInCharSet::<2, true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::KeepInRest,
        }
    );

    test_rule!(
        ascii_not_enough_matches,
        "a1b2c3",
        UntilNInCharSet::<4, true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_empty_input,
        "",
        UntilNInCharSet::<1, true, _, _> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        utf8_unicode_keep_left,
        "你好世界",
        UntilNInCharSet::<2, false, _, _> {
            filter: &CharSetFilter::new(['你', '世', '好']),
            mode: UntilMode::KeepInOutput,
        }
    );

    test_rule!(
        utf8_not_enough_matches,
        "你好世界",
        UntilNInCharSet::<4, false, _, _> {
            filter: &CharSetFilter::new(['你', '世', '好']),
            mode: UntilMode::KeepInOutput,
        }
    );
}
