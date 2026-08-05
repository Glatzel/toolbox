use super::IStrFlowRule;
use crate::error::RuleError;
use crate::string::IRule;
use crate::string::filters::{CharSetFilter, IFilter};
use crate::string::rules::UntilMode;
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
pub struct UntilOneInCharSet<'a, const N: usize> {
    pub filter: &'a CharSetFilter<N>,
    pub mode: UntilMode,
}

impl<const N: usize> IRule for UntilOneInCharSet<'_, N> {}

impl<'a, const N: usize> IStrFlowRule<'a> for UntilOneInCharSet<'a, N> {
    type Output = &'a str;

    fn apply(&self, input: &'a str, is_ascii: bool) -> Result<(Self::Output, usize), RuleError> {
        if is_ascii {
            if let Some(mask) = self.filter.ascii_mask() {
                return input
                    .as_bytes()
                    .iter()
                    .position(|&b| mask & (1_u128 << u32::from(b)) != 0)
                    .map_or_else(
                        || {
                            Err(RuleError {
                                reason: "no match found".into(),
                            })
                        },
                        |i| Ok(self.mode.split_str(input, i, 1)),
                    );
            }
            // Fallback: table has non-ASCII entries
            for (i, &b) in input.as_bytes().iter().enumerate() {
                if self.filter.filter(&(b as char)) {
                    return Ok(self.mode.split_str(input, i, 1));
                }
            }
            return Err(RuleError {
                reason: "no match found".into(),
            });
        }

        // UTF-8 path
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

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    use clerk::{LevelFilter, init_log_with_level};
    extern crate std;
    use std::format;

    use super::*;
    use crate::string::filters::{CHAR_SET_ASCII_LETTERS, CHAR_SET_DIGITS};
    #[rstest::rstest]
    #[case("ascii_discard", "abc1def", PhantomData::<UntilOneInCharSet<_>>, &CHAR_SET_DIGITS, UntilMode::Discard)]
    #[case("ascii_keep_left", "abc1def", PhantomData::<UntilOneInCharSet<_>>, &CHAR_SET_DIGITS, UntilMode::KeepInOutput)]
    #[case("ascii_keep_right_first_char", "a123", PhantomData::<UntilOneInCharSet<_>>, &CHAR_SET_ASCII_LETTERS, UntilMode::KeepInRest)]
    #[case("ascii_keep_right_not_first_char", "abc1def", PhantomData::<UntilOneInCharSet<_>>, &CHAR_SET_DIGITS, UntilMode::KeepInRest)]
    #[case("ascii_no_match", "abcdef", PhantomData::<UntilOneInCharSet<_>>, &CHAR_SET_DIGITS , UntilMode::Discard)]
    #[case("ascii_empty_input", "", PhantomData::<UntilOneInCharSet<_>>, &CHAR_SET_DIGITS, UntilMode::Discard)]
    fn test_until_one_in_char_set<const N: usize>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<UntilOneInCharSet<N>>,
        #[case] filter: &CharSetFilter<N>,
        #[case] mode: UntilMode,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let result = UntilOneInCharSet::<N> { filter, mode }
            .apply(input, input.is_ascii())
            .map(|(out, idx)| (out, input.get(idx..).unwrap()));
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
