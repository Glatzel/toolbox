use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::rules::UntilMode;

/// Rule that extracts a prefix from the input string up to the first
/// occurrence of a specified substring delimiter.
///
/// `UntilStr<IS_ASCII>` searches for the first occurrence of `pattern` in the
/// input string.
///
/// If the delimiter is found, the rule uses `mode` to determine how the input
/// is split and returns `Ok((prefix, consumed_bytes))`, where `prefix` is the
/// extracted substring and `consumed_bytes` is the number of bytes consumed
/// according to the selected mode.
///
/// If the delimiter is not found, the rule returns a [`RuleError`].
///
/// # Fields
///
/// - `pattern`: The delimiter substring to search for. An empty pattern matches
///   at the beginning of the input.
/// - `mode`: Determines how the delimiter is handled:
///   - [`UntilMode::Discard`]: Excludes the delimiter from the prefix and
///     consumes it.
///   - [`UntilMode::KeepInOutput`]: Includes the delimiter at the end of the
///     prefix.
///   - [`UntilMode::KeepInRest`]: Leaves the delimiter at the beginning of the
///     remainder.
///
/// # Type Parameters
///
/// - `IS_ASCII`: Whether the input is assumed to contain only ASCII characters.
///
/// Matching uses UTF-8 byte offsets returned by [`str::find`]. The delimiter
/// may contain multiple characters, including multi-byte UTF-8 characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntilStr<const IS_ASCII: bool> {
    pub pattern: &'static str,
    pub mode: UntilMode,
}

impl<const IS_ASCII: bool> IRule for UntilStr<IS_ASCII> {}

impl<const IS_ASCII: bool> IFlowRule<IS_ASCII> for UntilStr<IS_ASCII> {
    type Output<'a> = &'a str;

    /// Applies the `UntilStr` rule to the input string.
    ///
    /// # Returns
    ///
    /// - `Ok((prefix, consumed_bytes))` if the delimiter is found, with the
    ///   split determined by the selected mode.
    /// - `Err(RuleError)` if the delimiter is not found.
    ///
    /// An empty delimiter matches at the beginning of the input.
    /// The consumed byte count depends on the selected mode and the delimiter
    /// length.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        input.find(self.pattern).map_or_else(
            || {
                Err(RuleError {
                    reason: "no match found".into(),
                })
            },
            |idx| Ok(self.mode.split_str(input, idx, self.pattern.len())),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_rule;

    test_rule!(
        ascii_discard,
        "abc-def",
        UntilStr::<true> {
            pattern: "-",
            mode: super::UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_keep_left,
        "abc-def",
        UntilStr::<true> {
            pattern: "-",
            mode: super::UntilMode::KeepInOutput,
        }
    );

    test_rule!(
        ascii_keep_right,
        "abc-def",
        UntilStr::<true> {
            pattern: "-",
            mode: super::UntilMode::KeepInRest,
        }
    );

    test_rule!(
        ascii_no_delimiter,
        "abcdef",
        UntilStr::<true> {
            pattern: "-",
            mode: super::UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_delimiter_at_start,
        "-abcdef",
        UntilStr::<true> {
            pattern: "-",
            mode: super::UntilMode::Discard,
        }
    );

    test_rule!(
        ascii_empty_input,
        "",
        UntilStr::<true> {
            pattern: "-",
            mode: super::UntilMode::Discard,
        }
    );
}
