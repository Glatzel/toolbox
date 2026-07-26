extern crate alloc;

use super::IStrFlowRule;
use crate::error::RuleError;
use crate::string::filters::{CharSetFilter, IFilter};
use crate::string::rules::UntilMode;
use crate::string::{Decoder, IRule};
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
///   - [`UntilMode::KeepLeft`]: Include the first non-matching character at the
///     end of the prefix.
///   - [`UntilMode::KeepRight`]: Keep the first non-matching character at the
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
pub struct UntilNotInCharSet<'a, const N: usize> {
    pub filter: &'a CharSetFilter<N>,
    pub mode: UntilMode,
}

impl<'a, const N: usize> IRule for UntilNotInCharSet<'a, N> {}

impl<'a, const N: usize> IStrFlowRule<'a> for UntilNotInCharSet<'a, N> {
    type Output = &'a str;

    fn apply(&self, decoder: &mut Decoder<'a>) -> Result<Self::Output, RuleError> {
        let input = decoder.rest_str();
        if decoder.is_ascii() {
            for (i, &b) in input.as_bytes().iter().enumerate() {
                let c = b as char;

                if !self.filter.filter(&c) {
                    return Ok(self.mode.advance(decoder, input, i, 1));
                }
            }
            decoder.advance(input.len());
            return Ok(input);
        }

        // UTF-8 path
        for (i, c) in input.char_indices() {
            if !self.filter.filter(&c) {
                return Ok(self.mode.advance(decoder, input, i, c.len_utf8()));
            }
        }

        decoder.advance(input.len());
        return Ok(input);
    }
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    extern crate std;
    use std::format;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    use crate::string::filters::DIGITS;
    #[rstest::rstest]
    #[case("discard", "123abc", PhantomData::<UntilNotInCharSet<_>>, &DIGITS, UntilMode::Discard)]
    #[case("keep_left", "123abc", PhantomData::<UntilNotInCharSet<_>>, &DIGITS, UntilMode::KeepLeft)]
    #[case("keep_right", "123abc", PhantomData::<UntilNotInCharSet<_>>, &DIGITS, UntilMode::KeepRight)]
    #[case("all_in_set", "123456", PhantomData::<UntilNotInCharSet<_>>, &DIGITS, UntilMode::Discard)]
    #[case("first_char_not_in_set", "a123", PhantomData::<UntilNotInCharSet<_>>, &DIGITS, UntilMode::Discard)]
    #[case("empty_input", "", PhantomData::<UntilNotInCharSet<_>>, &DIGITS, UntilMode::Discard)]
    #[case("unicode", "你好世界", PhantomData::<UntilNotInCharSet<2>>, &CharSetFilter::new(['好', '你']), UntilMode::Discard)]
    fn test_until_not_in_char_set<const N: usize>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<UntilNotInCharSet<N>>,
        #[case] filter: &CharSetFilter<N>,
        #[case] mode: UntilMode,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = Decoder::new(input);
        let result = UntilNotInCharSet::<N> { filter, mode }
            .apply(&mut decoder)
            .map(|out| (out, decoder.rest_str()));
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
