use super::IFlowRule;
use crate::error::RuleError;
use crate::text::IRule;
use crate::text::rules::UntilMode;

/// Rule that extracts a prefix from the input string up to the first occurrence
/// of a specified substring delimiter.
///
/// # Fields
///
/// - `pattern`: The delimiter substring to search for.
/// - `mode`: Determines how the delimiter is treated:
///   - [`UntilMode::Discard`]: Exclude the delimiter from the prefix and remove
///     it from the rest.
///   - [`UntilMode::KeepInOutput`]: Include the delimiter in the prefix.
///   - [`UntilMode::KeepInRest`]: Keep the delimiter at the start of the rest.
///
/// # Behavior
///
/// - Returns `(Some(prefix), rest)` if the delimiter is found, split according
///   to `mode`.
/// - Returns `(None, input)` if the delimiter is not found.
/// - Logs debug information for each split or when no match is found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntilStr<const IS_ASCII: bool> {
    pub pattern: &'static str,
    pub mode: UntilMode,
}

impl<const IS_ASCII: bool> IRule for UntilStr<IS_ASCII> {}
impl<const IS_ASCII: bool> IFlowRule<IS_ASCII> for UntilStr<IS_ASCII> {
    type Output<'a> = &'a str;
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        clerk::trace!(
            "{:?}: input='{}', delimiter='{}', mode={:?}",
            self,
            input,
            self.pattern,
            self.mode
        );

        input.find(self.pattern).map_or_else(
            || {
                clerk::debug!(
                    "{:?}: delimiter '{}' not found, returning None",
                    self,
                    self.pattern
                );
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
