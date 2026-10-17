extern crate alloc;
use core::fmt::Debug;

use jiff::civil::Date;
use rax::error::RuleError;
use rax::text::IRule;

use super::UNTIL_COMMA_DISCARD;

/// Parses an NMEA date in the `ddmmyy` format.
///
/// The two-digit year is interpreted as a year between 2000 and 2099. An
/// empty field produces [`None`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaDate;

impl IRule for NmeaDate {}

impl rax::text::IFlowRule<true> for NmeaDate {
    type Output<'a> = Option<Date>;
    /// Parses the date field and returns it together with the number of bytes
    /// consumed.
    ///
    /// # Errors
    ///
    /// Returns a [`RuleError`] if the field is missing, if the day, month, or
    /// year components cannot be parsed, or if they do not form a valid
    /// calendar date.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        let (res, advanced) = UNTIL_COMMA_DISCARD.apply(input).map_err(|_| RuleError {
            reason: "Missing Date string.".into(),
        })?;
        if res.is_empty() {
            return Ok((None, advanced));
        }

        let Some(day) = res.get(0..2).and_then(|s| s.parse::<i8>().ok()) else {
            return Err(RuleError {
                reason: "Failed to parse day.".into(),
            });
        };
        let Some(month) = res.get(2..4).and_then(|s| s.parse::<i8>().ok()) else {
            return Err(RuleError {
                reason: "Failed to parse month.".into(),
            });
        };
        let Some(year) = res.get(4..6).and_then(|s| s.parse::<i16>().ok()) else {
            return Err(RuleError {
                reason: "Failed to parse year.".into(),
            });
        };
        let dt = match Date::new(year + 2000, month, day) {
            Ok(dt) => dt,
            Err(e) => {
                return Err(RuleError {
                    reason: alloc::format!("{e:?}").into(),
                });
            }
        };
        Ok((Some(dt), advanced))
    }
}

#[cfg(test)]
mod tests {
    use rax::text::IFlowRule;

    use super::*;

    #[rstest::rstest]
    #[case("valid", "110324,foo,bar")]
    #[case("invalid_day", "xx0324,foo,bar")]
    #[case("invalid_month", "11xx24,foo,bar")]
    #[case("invalid_year", "1103xx,foo,bar")]
    #[case("no_comma", "110324")]
    #[case("invalid_date", "320224,foo,bar")]
    #[case("empty", ",foo,bar")]
    fn test_nmea_date_valid(#[case] name: &str, #[case] input: &str) {
        let result = NmeaDate
            .apply(input)
            .map(|(out, idx)| (out, input.get(idx..).unwrap()));
        insta::assert_debug_snapshot!(name, result)
    }
}
