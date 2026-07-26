extern crate alloc;

use super::IStrFlowRule;
use crate::error::RuleError;
use crate::string::{Decoder, IRule};
/// Rule that extracts a substring from the start of the input until a
/// specified delimiter character is encountered.
///
/// `UntilChar<C>` searches the input string for the first occurrence of
/// character `C` and splits the input according to the selected [`UntilMode`].
///
/// # Fields
///
/// - `mode`: Determines how the delimiter is treated in the output:
///   - [`UntilMode::Discard`]: The delimiter is removed from the prefix and
///     rest.
///   - [`UntilMode::KeepLeft`]: The delimiter is included at the end of the
///     prefix.
///   - [`UntilMode::KeepRight`]: The delimiter is included at the start of the
///     rest.
///
/// # Type Parameters
///
/// - `C`: The delimiter character to search for.
///
/// # Behavior
///
/// - Returns `(Some(prefix), rest)` if the delimiter is found.
/// - Returns `(None, input)` if the delimiter is not present in the input.
///
/// This rule respects UTF-8 character boundaries and logs trace/debug
/// information for each operation.
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
    fn apply(&self, decoder: &mut Decoder<'a>) -> Result<Self::Output, RuleError> {
        let input = decoder.rest_str();
        clerk::trace!(
            "{:?} rule: input='{:?}', char='{}', mode={:?}",
            self,
            input,
            C,
            self.mode
        );

        let pos = if decoder.is_ascii() {
            input.as_bytes().iter().position(|&b| b == C as u8)
        } else {
            input
                .char_indices()
                .find_map(|(i, c)| (c == C).then_some(i))
        };

        let i = pos.ok_or_else(|| RuleError {
            reason: "delimiter not found".into(),
        })?;

        Ok(self.mode.advance(decoder, input, i,  C.len_utf8()))
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
    #[case("discard","abc-def", PhantomData::<UntilChar<'-'>>, UntilMode::Discard)]
    #[case("keep_left","abc-def", PhantomData::<UntilChar<'-'>>, UntilMode::KeepLeft)]
    #[case("keep_right","abc-def", PhantomData::<UntilChar<'-'>>, UntilMode::KeepRight)]
    #[case("delimiter_at_start","-abcdef", PhantomData::<UntilChar<'-'>>, UntilMode::Discard)]
    #[case("no_delimiter","abcdef", PhantomData::<UntilChar<'-'>>, UntilMode::Discard)]
    #[case("empty_input","", PhantomData::<UntilChar<'-'>>, UntilMode::Discard)]
    fn test_until_char<const C: char>(
        #[case] name: &str,
        #[case] input: &str,
        #[case] _rule: PhantomData<UntilChar<C>>,
        #[case] mode: UntilMode,
    ) {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = Decoder::new(input);
        let result = UntilChar::<C> { mode }
            .apply(&mut decoder)
            .map(|out| (out, decoder.rest_str()));
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
