use clap::error::ErrorKind;

pub fn parse_generic<T>(value: &str) -> Result<T, clap::Error>
where
    T: core::str::FromStr<Err = mischief::Report>,
{
    value
        .parse::<T>()
        .map_err(|e| clap::Error::raw(ErrorKind::ValueValidation, e.to_string()))
}
