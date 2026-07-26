extern crate alloc;

use super::IStrFlowRule;
use crate::error::RuleError;
use crate::string::IRule;
/// Rule that extracts a substring from the start of the input until a
/// specified delimiter character is encountered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntilChar<const C: char> {
    pub mode: super::UntilMode,
}
impl<const C: char> UntilChar<C> {
    const DELIM_LEN: usize = C.len_utf8();
}
impl<const C: char> IRule for UntilChar<C> {}

impl<'a, const C: char> IStrFlowRule<'a> for UntilChar<C> {
    type Output = &'a str;

    /// Applies the `UntilChar` rule to the input string.
    ///
    /// - Scans the input from the start until the delimiter `C` is found.
    /// - Returns a tuple `(prefix, rest)` split according to `self.mode`.
    /// - If the delimiter is not found, returns `(None, input)`.
    fn apply(&self, input: &'a str, _is_ascii: bool) -> Result<(Self::Output, &'a str), RuleError> {
        clerk::trace!(
            "{:?} rule: input='{:?}', char='{}', mode={:?}",
            self,
            input,
            C,
            self.mode
        );
        match input.find(C) {
            Some(idx) => Ok(self.mode.split_str(input, idx, Self::DELIM_LEN)),
            None => Err(RuleError {
                reason: "input is empty or does not contain the expected character.".into(),
            }),
        }
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
    #[case("ascii_keep_left","abc-def", PhantomData::<UntilChar<'-'>>, UntilMode::KeepInOutput)]
    #[case("ascii_keep_right","abc-def", PhantomData::<UntilChar<'-'>>, UntilMode::KeepInRest)]
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
