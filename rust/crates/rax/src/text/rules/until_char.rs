use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
/// Rule that extracts a substring from the start of the input until a
/// specified delimiter character is encountered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntilChar<const C: char, const IS_ASCII: bool> {
    pub mode: super::UntilMode,
}

impl<const C: char, const IS_ASCII: bool> IRule for UntilChar<C, IS_ASCII> {}

// ASCII fast path: byte scan, delimiter is always 1 byte.
impl<const C: char> IFlowRule<true> for UntilChar<C, true> {
    type Output<'a> = &'a str;

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        clerk::trace!(
            "{:?} rule: input='{:?}', char='{}', mode={:?}",
            self,
            input,
            C,
            self.mode
        );

        let target = C as u8;
        input
            .as_bytes()
            .iter()
            .position(|&b| b == target)
            .map_or_else(
                || {
                    Err(RuleError {
                        reason: "input is empty or does not contain the expected character.".into(),
                    })
                },
                |idx| Ok(self.mode.split_str(input, idx, 1)),
            )
    }
}

// Non-ASCII path: char scan, delimiter length depends on the char.
impl<const C: char> IFlowRule<false> for UntilChar<C, false> {
    type Output<'a> = &'a str;

    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        clerk::trace!(
            "{:?} rule: input='{:?}', char='{}', mode={:?}",
            self,
            input,
            C,
            self.mode
        );

        for (idx, ch) in input.char_indices() {
            if ch == C {
                return Ok(self.mode.split_str(input, idx, ch.len_utf8()));
            }
        }

        Err(RuleError {
            reason: "input is empty or does not contain the expected character.".into(),
        })
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
        utf8_discard,
        "你好世界",
        UntilChar::<'好', false> {
            mode: UntilMode::Discard
        }
    );

    test_rule!(
        utf8_keep_left,
        "你好世界",
        UntilChar::<'好', false> {
            mode: UntilMode::KeepInOutput
        }
    );

    test_rule!(
        utf8_keep_right,
        "你好世界",
        UntilChar::<'好', false> {
            mode: UntilMode::KeepInRest
        }
    );

    test_rule!(
        utf8_delimiter_at_start,
        "你好世界",
        UntilChar::<'你', false> {
            mode: UntilMode::Discard
        }
    );

    test_rule!(
        utf8_no_delimiter,
        "你好世界",
        UntilChar::<'们', false> {
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
