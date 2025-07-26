pub trait ParseOptExt<T> {
    fn parse_opt<U: std::str::FromStr>(self) -> Option<U>;
}

impl<'a> ParseOptExt<&'a str> for Option<&'a str> {
    fn parse_opt<U: std::str::FromStr>(self) -> Option<U> { self.and_then(|s| s.parse::<U>().ok()) }
}
