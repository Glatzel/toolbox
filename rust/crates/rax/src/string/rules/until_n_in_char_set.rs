use super::IStrFlowRule;
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
///   - [`UntilMode::KeepLeft`]: The N-th character is included at the end of
///     the prefix.
///   - [`UntilMode::KeepRight`]: The N-th character is included at the start of
///     the rest.
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

impl<'a, const N: usize, const M: usize> IRule for UntilNInCharSet<'a, N, M> {}

impl<'a, const N: usize, const M: usize> IStrFlowRule<'a> for UntilNInCharSet<'a, N, M> {
    type Output = &'a str;
    type Error = &'static str;
    fn apply(&self, input: &'a str) -> Result<(Self::Output, &'a str), Self::Error> {
        let mut remaining = N;

        for (idx, ch) in input.char_indices() {
            if self.filter.filter(&ch) {
                remaining -= 1;
                if remaining == 0 {
                    let after = idx + ch.len_utf8();
                    let (prefix, rest) = match self.mode {
                        UntilMode::Discard => (&input[..idx], &input[after..]),
                        UntilMode::KeepLeft => (&input[..after], &input[after..]),
                        UntilMode::KeepRight => (&input[..idx], &input[idx..]),
                    };
                    clerk::debug!(
                        "UntilNInCharSet: mode={:?}, prefix='{}', rest='{}', idx={}, after={}, N={}",
                        self.mode,
                        prefix,
                        rest,
                        idx,
                        after,
                        N
                    );
                    return Ok((prefix, rest));
                }
            }
        }

        clerk::debug!(
            "{:?}: fewer than {} matches found, returning None, input='{}'",
            self,
            N,
            input
        );
        Err("fewer than N matches found")
    }
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;
    use std::str::FromStr;
    extern crate std;
    use std::format;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    use crate::string::filters::DIGITS;

    #[rstest::rstest]
    #[case(
        "discard",
        "a1b2c3",
        PhantomData::<UntilNInCharSet<2, _>>,
        &DIGITS,
        UntilMode::Discard
    )]
    #[case(
        "keep_left",
        "a1b2c3",
        PhantomData::<UntilNInCharSet<2, _>>,
        &DIGITS,
        UntilMode::KeepLeft,

    )]
    #[case(
        "keep_right",
        "a1b2c3",
        PhantomData::<UntilNInCharSet<2, _>>,
        &DIGITS,
        UntilMode::KeepRight,

    )]
    #[case(
        "not_enough_matches",
        "a1b2c3",
        PhantomData::<UntilNInCharSet<4, _>>,
        &DIGITS,
        UntilMode::Discard,

    )]
    #[case(
        "empty_input",
        "",
        PhantomData::<UntilNInCharSet<1, _>>,
        &DIGITS,
        UntilMode::Discard,

    )]
    #[case(
        "unicode_keep_left",
        "",
        PhantomData::<UntilNInCharSet<2, 3>>,
        &CharSetFilter::from_str("你世好").unwrap(),
        UntilMode::KeepLeft,
    )]
    fn test_until_n_in_char_set<const N: usize, const M: usize>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<UntilNInCharSet<N, M>>,
        #[case] filter: &CharSetFilter<M>,
        #[case] mode: UntilMode,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let result = UntilNInCharSet::<N, M> { filter, mode }.apply(input);
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
