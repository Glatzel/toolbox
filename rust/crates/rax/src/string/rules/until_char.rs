extern crate alloc;

use super::IStrFlowRule;
use crate::error::RuleError;
use crate::string::IRule;
/// Rule that extracts a substring from the start of the input until a
/// specified delimiter character is encountered.
///
/// `UntilChar<C>` searches the input string for the first occurrence of
/// character `C` and splits the input according to the selected [`UntilMode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntilChar<const C: char> {
    pub mode: super::UntilMode,
}

impl<const C: char> IRule for UntilChar<C> {}

impl<'a, const C: char> IStrFlowRule<'a> for UntilChar<C> {
    type Output = &'a str;

    /// Applies the `UntilChar` rule to the input string.
    ///
    /// - Scans the input from the start until the delimiter `C` is found.
    /// - Returns a tuple `(prefix, rest)` split according to `self.mode`.
    /// - If the delimiter is not found, returns `(None, input)`.
    fn apply(&self, input: &'a str, is_ascii: bool) -> Result<(Self::Output, &'a str), RuleError> {
        clerk::trace!(
            "{:?} rule: input='{:?}', char='{}', mode={:?}",
            self,
            input,
            C,
            self.mode
        );

        let pos = if is_ascii {
            input.as_bytes().iter().position(|&b| b == C as u8)
        } else {
            input
                .char_indices()
                .find_map(|(i, c)| (c == C).then_some(i))
        };

        let i = pos.ok_or_else(|| RuleError {
            reason: "delimiter not found".into(),
        })?;

        Ok(self.mode.split_str(input, i, C.len_utf8()))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use core::marker::PhantomData;
    use std::format;

    use clerk::{LevelFilter, init_log_with_level};
    extern crate std;
    use super::*;
    use crate::string::UntilMode;

    #[rstest::rstest]
    #[case("ascii_discard","abc-def", PhantomData::<UntilChar<'-'>>, UntilMode::Discard)]
    #[case("ascii_keep_left","abc-def", PhantomData::<UntilChar<'-'>>, UntilMode::KeepLeft)]
    #[case("ascii_keep_right","abc-def", PhantomData::<UntilChar<'-'>>, UntilMode::KeepRight)]
    #[case("ascii_delimiter_at_start","-abcdef", PhantomData::<UntilChar<'-'>>, UntilMode::Discard)]
    #[case("ascii_no_delimiter","abcdef", PhantomData::<UntilChar<'-'>>, UntilMode::Discard)]
    #[case("utf8_empty_input","", PhantomData::<UntilChar<'-'>>, UntilMode::Discard)]
    fn test_until_char<const C: char>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<UntilChar<C>>,
        #[case] mode: UntilMode,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let result = UntilChar::<C> { mode }.apply(input, input.is_ascii());
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
