use core::str::FromStr;

pub trait ParseOptionPrimitive<T>
where
    T: FromStr,
{
    fn parse_option(&self) -> Result<Option<T>, T::Err>;
}

impl<T> ParseOptionPrimitive<T> for str
where
    T: FromStr,
{
    fn parse_option(&self) -> Result<Option<T>, T::Err> {
        if self.is_empty() {
            return Ok(None);
        }
        let parsed = self.parse::<T>()?;
        Ok(Some(parsed))
    }
}
