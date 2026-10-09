use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;

/// Rule that extracts a substring from the start of the input until a
/// specified delimiter character is encountered.
///
/// The `UntilChar<C, IS_ASCII>` rule searches for the first occurrence of
/// the delimiter `C` in the input string.
///
/// If the delimiter is found, the rule uses [`UntilMode`] to determine how
/// the input is split and returns `Ok((prefix, consumed_bytes))`, where
/// `prefix` is the extracted substring and `consumed_bytes` is the number
/// of bytes consumed, as determined by the selected mode.
///
/// For `UntilChar<C, true>`, the input is scanned byte by byte. This fast
/// path assumes that the input and delimiter are ASCII characters.
///
/// For `UntilChar<C, false>`, the input is scanned by Unicode scalar value,
/// so multi-byte UTF-8 delimiters are handled correctly.
///
/// If the delimiter is not found, the rule returns a [`RuleError`].
///
/// # Type Parameters
///
/// - `C`: Delimiter character to search for.
/// - `IS_ASCII`: Whether the input is assumed to contain only ASCII characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntilChar<const C: char, const IS_ASCII: bool> {
    /// Determines how the input is split around the delimiter.
    pub mode: super::UntilMode,
}

impl<const C: char, const IS_ASCII: bool> IRule for UntilChar<C, IS_ASCII> {}

// ASCII fast path: scan bytes because each ASCII character occupies one byte.
impl<const C: char> IFlowRule<true> for UntilChar<C, true> {
    type Output<'a> = &'a str;

    /// Applies the `UntilChar` rule using byte-based scanning.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if the delimiter is found and the
    ///   selected mode can split the input.
    /// - `Err(RuleError)` if the delimiter is not found.
    ///
    /// The input and delimiter must be ASCII for byte-based matching to
    /// correspond to character-based matching.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
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

// Unicode path: scan character boundaries because the delimiter may occupy
// multiple bytes in UTF-8.
impl<const C: char> IFlowRule<false> for UntilChar<C, false> {
    type Output<'a> = &'a str;

    /// Applies the `UntilChar` rule using Unicode character scanning.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if the delimiter is found and the
    ///   selected mode can split the input.
    /// - `Err(RuleError)` if the delimiter is not found.
    ///
    /// The consumed byte count depends on the selected [`UntilMode`] and the
    /// UTF-8 length of the delimiter.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
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
