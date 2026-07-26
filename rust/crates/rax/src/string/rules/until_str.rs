extern crate alloc;

use alloc::string::ToString;

use super::IStrFlowRule;
use crate::error::RuleError;
use crate::string::IRule;
use crate::string::rules::UntilMode;

/// Rule that extracts a prefix from the input string up to the first occurrence
/// of a specified substring delimiter.
///
/// # Fields
///
/// - `pattern`: The delimiter substring to search for.
/// - `mode`: Determines how the delimiter is treated:
///   - [`UntilMode::Discard`]: Exclude the delimiter from the prefix and remove
///     it from the rest.
///   - [`UntilMode::KeepLeft`]: Include the delimiter in the prefix.
///   - [`UntilMode::KeepRight`]: Keep the delimiter at the start of the rest.
///
/// # Behavior
///
/// - Returns `(Some(prefix), rest)` if the delimiter is found, split according
///   to `mode`.
/// - Returns `(None, input)` if the delimiter is not found.
/// - Logs debug information for each split or when no match is found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntilStr {
    pub pattern: &'static str,
    pub mode: UntilMode,
}

impl IRule for UntilStr {}
impl<'a> IStrFlowRule<'a> for UntilStr {
    type Output = &'a str;
    fn apply(&self, input: &'a str, _is_ascii: bool) -> Result<(Self::Output, &'a str), RuleError> {
        clerk::trace!(
            "{:?}: input='{}', delimiter='{}', mode={:?}",
            self,
            input,
            self.pattern,
            self.mode
        );

        match input.find(self.pattern) {
            Some(idx) => Ok(self.mode.split_str(input, idx, self.pattern.len())),
            None => {
                clerk::debug!(
                    "{:?}: delimiter '{}' not found, returning None",
                    self,
                    self.pattern
                );
                Err(RuleError {
                    reason: "no match found".to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::format;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[rstest::rstest]
    #[case("ascii_discard", "abc-def", UntilStr { pattern: "-", mode: super::UntilMode::Discard })]
    #[case("ascii_keep_left", "abc-def", UntilStr { pattern: "-", mode: super::UntilMode::KeepLeft })]
    #[case("ascii_keep_right", "abc-def", UntilStr { pattern: "-", mode: super::UntilMode::KeepRight })]
    #[case("ascii_no_delimiter", "abcdef", UntilStr { pattern: "-", mode: super::UntilMode::Discard })]
    #[case("ascii_delimiter_at_start", "-abcdef", UntilStr { pattern: "-", mode: super::UntilMode::Discard })]
    #[case("ascii_empty_input", "", UntilStr { pattern: "-", mode: super::UntilMode::Discard })]
    fn test_until_str(#[case] name: &str, #[case] input: &str, #[case] rule: UntilStr) {
        init_log_with_level(LevelFilter::TRACE);
        let result = rule.apply(input, input.is_ascii());
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
