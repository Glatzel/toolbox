use super::IStrFlowRule;
use crate::error::RuleError;
use crate::string::IRule;
use crate::string::filters::{CharSetFilter, IFilter};
use crate::string::rules::UntilMode;
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
pub struct UntilNInCharSet<'a, const N: usize, const M: usize> {
    pub filter: &'a CharSetFilter<M>,
    pub mode: UntilMode,
}

impl<const N: usize, const M: usize> IRule for UntilNInCharSet<'_, N, M> {}

impl<'a, const N: usize, const M: usize> IStrFlowRule<'a> for UntilNInCharSet<'a, N, M> {
    type Output = &'a str;

    fn apply(&self, input: &'a str, is_ascii: bool) -> Result<(Self::Output, usize), RuleError> {
        if N == 0 {
            clerk::warn!("N is 0, returning empty string");
            return Ok(("", 0));
        }

        if is_ascii {
            let mut remaining = N;
            if let Some(mask) = self.filter.ascii_mask() {
                // Fast path: bitmask lookup, no per-char filter() call
                for (idx, &b) in input.as_bytes().iter().enumerate() {
                    if mask & (1_u128 << u32::from(b)) != 0 {
                        remaining -= 1;
                        if remaining == 0 {
                            return Ok(self.mode.split_str(input, idx, 1));
                        }
                    }
                }
            } else {
                // Fallback: table has non-ASCII entries, use original filter
                for (idx, &b) in input.as_bytes().iter().enumerate() {
                    let ch = b as char;
                    if self.filter.filter(&ch) {
                        remaining -= 1;
                        if remaining == 0 {
                            return Ok(self.mode.split_str(input, idx, 1));
                        }
                    }
                }
            }
            return Err(RuleError {
                reason: "fewer than N matches found".into(),
            });
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
    use core::marker::PhantomData;

    extern crate std;
    use std::format;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    use crate::string::filters::CHAR_SET_DIGITS;
    #[rstest::rstest]
    #[case(
        "zero_n",
        "a1b2c3",
        PhantomData::<UntilNInCharSet<0, _>>,
        &CHAR_SET_DIGITS,
        UntilMode::Discard
    )]
    #[rstest::rstest]
    #[case(
        "ascii_discard",
        "a1b2c3",
        PhantomData::<UntilNInCharSet<2, _>>,
        &CHAR_SET_DIGITS,
        UntilMode::Discard
    )]
    #[case(
        "ascii_keep_left",
        "a1b2c3",
        PhantomData::<UntilNInCharSet<2, _>>,
        &CHAR_SET_DIGITS,
        UntilMode::KeepInOutput,

    )]
    #[case(
        "ascii_keep_right",
        "a1b2c3",
        PhantomData::<UntilNInCharSet<2, _>>,
        &CHAR_SET_DIGITS,
        UntilMode::KeepInRest,

    )]
    #[case(
        "ascii_not_enough_matches",
        "a1b2c3",
        PhantomData::<UntilNInCharSet<4, _>>,
        &CHAR_SET_DIGITS,
        UntilMode::Discard,

    )]
    #[case(
        "ascii_empty_input",
        "",
        PhantomData::<UntilNInCharSet<1, _>>,
        &CHAR_SET_DIGITS,
        UntilMode::Discard,

    )]
    #[case(
        "ascii_fallback_match",
        "a1b2c3",
        PhantomData::<UntilNInCharSet<2, _>>,
        &CharSetFilter::new(['0', '1', '2', '你']),
        UntilMode::KeepInOutput,
    )]
    #[case(
        "ascii_fallback_not_enough_matches",
        "abc",
        PhantomData::<UntilNInCharSet<1, _>>,
        &CharSetFilter::new(['1', '2', '你']),
        UntilMode::KeepInOutput,
    )]
    #[case(
        "utf8_unicode_keep_left",
        "你好世界",
        PhantomData::<UntilNInCharSet<2, 3>>,
        &CharSetFilter::new(['你', '世', '好']),
        UntilMode::KeepInOutput,
    )]
    #[case(
        "utf8_not_enough_matches",
        "你好世界",
        PhantomData::<UntilNInCharSet<4, 3>>,
        &CharSetFilter::new(['你', '世', '好']),
        UntilMode::KeepInOutput,
    )]
    fn test_until_n_in_char_set<const N: usize, const M: usize>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<UntilNInCharSet<N, M>>,
        #[case] filter: &CharSetFilter<M>,
        #[case] mode: UntilMode,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let result = UntilNInCharSet::<N, M> { filter, mode }
            .apply(input, input.is_ascii())
            .map(|(out, idx)| (out, input.get(idx..).unwrap()));
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
