use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::filters::{CharSetFilter, IFilter};
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
pub struct UntilNotInCharSet<'f, const N: usize, const IS_ASCII: bool> {
    pub filter: &'f CharSetFilter<N>,
    pub mode: UntilMode,
}

impl<const N: usize, const IS_ASCII: bool> IRule for UntilNotInCharSet<'_, N, IS_ASCII> {}

impl<'f, const N: usize> IFlowRule<false> for UntilNotInCharSet<'f, N, false> {
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
impl<'f, const N: usize> IFlowRule<true> for UntilNotInCharSet<'f, N, true> {
    type Output<'a> = &'a str;

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        if let Some(mask) = self.filter.ascii_mask() {
            // Fast path: bitmask, no per-byte filter() dispatch
            for (i, &b) in input.as_bytes().iter().enumerate() {
                if mask & (1_u128 << u32::from(b)) == 0 {
                    return Ok(self.mode.split_str(input, i, 1));
                }
            }
            return Ok((input, input.len()));
        }
        // Fallback: table has non-ASCII entries
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
        UntilNotInCharSet::<10, true> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_keep_left,
        "123abc",
        UntilNotInCharSet::<10, true> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::KeepInOutput,
        }
    );

    test_rule!(
        ascii_keep_right,
        "123abc",
        UntilNotInCharSet::<10, true> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::KeepInRest,
        }
    );

    test_rule!(
        ascii_all_in_set,
        "123456",
        UntilNotInCharSet::<10, true> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_first_char_not_in_set,
        "a123",
        UntilNotInCharSet::<10, true> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_empty_input,
        "",
        UntilNotInCharSet::<10, true> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_fallback_not_in_set,
        "123abc",
        UntilNotInCharSet::<5, false> {
            filter: &CharSetFilter::new(['0', '1', '2', '3', '你']),
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_fallback_all_in_set,
        "123",
        UntilNotInCharSet::<5, false> {
            filter: &CharSetFilter::new(['0', '1', '2', '3', '你']),
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        utf8_discard,
        "你好世界",
        UntilNotInCharSet::<2, false> {
            filter: &CharSetFilter::new(['好', '你']),
            mode: UntilMode::Discard,
        }
    );

    test_rule!(
        utf8_all_in_set,
        "你好世界",
        UntilNotInCharSet::<4, false> {
            filter: &CharSetFilter::new(['你', '好', '世', '界']),
            mode: UntilMode::Discard,
        }
    );
}
