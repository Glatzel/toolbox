use super::IStrFlowRule;
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
    type Error = &'static str;
    fn apply(&self, input: &'a str) -> Result<(Self::Output, &'a str), Self::Error> {
        clerk::trace!(
            "{:?}: input='{}', delimiter='{}', mode={:?}",
            self,
            input,
            self.pattern,
            self.mode
        );

        match input.find(self.pattern) {
            Some(idx) => {
                let end = idx + self.pattern.len();
                let (prefix, rest) = match self.mode {
                    UntilMode::Discard => (&input[..idx], &input[end..]),
                    UntilMode::KeepLeft => (&input[..end], &input[end..]),
                    UntilMode::KeepRight => (&input[..idx], &input[idx..]),
                };
                clerk::debug!("{:?} matched: prefix='{}', rest='{}'", self, prefix, rest);
                Ok((prefix, rest))
            }
            None => {
                clerk::debug!(
                    "{:?}: delimiter '{}' not found, returning None",
                    self,
                    self.pattern
                );
                Err("no match found")
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
    #[case("discard", "abc-def", UntilStr { pattern: "-", mode: super::UntilMode::Discard })]
    #[case("keep_left", "abc-def", UntilStr { pattern: "-", mode: super::UntilMode::KeepLeft })]
    #[case("keep_right", "abc-def", UntilStr { pattern: "-", mode: super::UntilMode::KeepRight })]
    #[case("no_delimiter", "abcdef", UntilStr { pattern: "-", mode: super::UntilMode::Discard })]
    #[case("delimiter_at_start", "-abcdef", UntilStr { pattern: "-", mode: super::UntilMode::Discard })]
    #[case("empty_input", "", UntilStr { pattern: "-", mode: super::UntilMode::Discard })]
    fn test_until_str(#[case] name: &str, #[case] input: &str, #[case] rule: UntilStr) {
        init_log_with_level(LevelFilter::TRACE);
        let result = rule.apply(input);
        insta::assert_debug_snapshot!(format!("{}", name), result);
    }
}
