/// Extension trait to provide convenient parsing for `Option<&str>`
///
/// Allows attempting to parse the inner string into a specified type,
/// returning `None` if the option is `None` or parsing fails.
pub trait ParseOptExt<T> {
    /// Attempt to parse the inner value into type `U`.
    ///
    /// Returns:
    /// - `Some(U)` if the option contains a string and parsing succeeds.
    /// - `None` if the option is `None` or parsing fails.
    fn parse_opt<U: std::str::FromStr>(self) -> Option<U>;
}

impl<'a> ParseOptExt<&'a str> for Option<&'a str> {
    fn parse_opt<U: std::str::FromStr>(self) -> Option<U> {
        self.and_then(|s| s.parse::<U>().ok())
    }
}
