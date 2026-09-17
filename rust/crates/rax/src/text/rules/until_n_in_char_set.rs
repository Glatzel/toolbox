use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{AsciiCharSetFilter, CharSetFilter, ICharSetFilter, IFilter};
use crate::text::rules::UntilMode;
/// Rule that extracts a prefix from the input string until the N-th character
/// matching a given character set is reached.
///
/// `UntilNInCharSet<N, M>` scans the input string from the start, counting
/// how many characters belong to the specified character set (defined by
/// `filter`).
///
/// # Fields
///
/// - `filter`: The [`CharSetFilter`] that defines the set of valid characters.
/// - `mode`: Determines how the N-th matched character is treated:
///   - [`UntilMode::Discard`]: The N-th character is excluded from the prefix
///     and removed from the rest.
///   - [`UntilMode::KeepInOutput`]: The N-th character is included at the end
///     of the prefix.
///   - [`UntilMode::KeepInRest`]: The N-th character is included at the start
///     of the rest.
///
/// # Type Parameters
///
/// - `N`: The number of matches required to stop scanning.
/// - `M`: The size of the character set (`CharSetFilter<M>`).
///
/// # Behavior
///
/// - Returns `(Some(prefix), rest)` when N characters in the set have been
///   seen, split according to `mode`.
/// - Returns `(None, input)` if fewer than N characters in the set are found.
/// - Respects UTF-8 character boundaries and logs trace/debug information.
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

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            clerk::warn!("N is 0, returning empty string");
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

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if N == 0 {
            clerk::warn!("N is 0, returning empty string");
            return Ok(("", 0));
        }

        // UTF-8 path
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
