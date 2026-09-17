use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
/// Rule that extracts a substring from the start of the input until a
/// specified delimiter character is encountered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntilChar<const C: char, const IS_ASCII: bool> {
    pub mode: super::UntilMode,
}
impl<const C: char, const IS_ASCII: bool> UntilChar<C, IS_ASCII> {
    const DELIM_LEN: usize = C.len_utf8();
}
impl<const C: char, const IS_ASCII: bool> IRule for UntilChar<C, IS_ASCII> {}

impl<const C: char, const IS_ASCII: bool> IFlowRule<IS_ASCII> for UntilChar<C, IS_ASCII> {
    type Output<'a> = &'a str;

    /// Applies the `UntilChar` rule to the input string.
    ///
    /// - Scans the input from the start until the delimiter `C` is found.
    /// - Returns a tuple `(prefix, rest)` split according to `self.mode`.
    /// - If the delimiter is not found, returns `(None, input)`.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        clerk::trace!(
            "{:?} rule: input='{:?}', char='{}', mode={:?}",
            self,
            input,
            C,
            self.mode
        );
        input.find(C).map_or_else(
            || {
                Err(RuleError {
                    reason: "input is empty or does not contain the expected character.".into(),
                })
            },
            |idx| Ok(self.mode.split_str(input, idx, Self::DELIM_LEN)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_rule;
    use crate::text::UntilMode;
    test_rule!(
        ascii_discard,
        "abc-def",
        UntilChar::<'-', true> {
            mode: UntilMode::Discard
        }
    );

    test_rule!(
        ascii_keep_left,
        "abc-def",
        UntilChar::<'-', true> {
            mode: UntilMode::KeepInOutput
        }
    );

    test_rule!(
        ascii_keep_right,
        "abc-def",
        UntilChar::<'-', true> {
            mode: UntilMode::KeepInRest
        }
    );

    test_rule!(
        ascii_delimiter_at_start,
        "-abcdef",
        UntilChar::<'-', true> {
            mode: UntilMode::Discard
        }
    );

    test_rule!(
        ascii_no_delimiter,
        "abcdef",
        UntilChar::<'-', true> {
            mode: UntilMode::Discard
        }
    );

    test_rule!(
        utf8_empty_input,
        "",
        UntilChar::<'-', false> {
            mode: UntilMode::Discard
        }
    );
}
