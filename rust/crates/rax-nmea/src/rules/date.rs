extern crate alloc;
use core::fmt::Debug;

use jiff::civil::Date;
use rax::error::RuleError;
use rax::string::IRule;

use super::UNTIL_COMMA_DISCARD;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaDate;

impl IRule for NmeaDate {}

impl<'a> rax::string::IStrFlowRule<'a> for NmeaDate {
    type Output = Option<Date>;
    /// Applies the `NmeaUtc` rule to the input string.
    /// Parses the UTC time, converts to `DateTime<Utc>` using today's date, and
    /// returns the result and the rest of the string. Logs each step for
    /// debugging.
    fn apply(&self, input: &'a str, is_ascii: bool) -> Result<(Self::Output, usize), RuleError> {
        clerk::trace!("NmeaUtc rule: input='{}'", input);

        let (res, advanced) =
            UNTIL_COMMA_DISCARD
                .apply(input, is_ascii)
                .map_err(|_| RuleError {
                    reason: "Missing Date string.".into(),
                })?;
        if res.is_empty() {
            return Ok((None, advanced));
        }

        let Some(day) = res.get(0..2).and_then(|s| s.parse::<i8>().ok()) else {
            clerk::error!("{:?}: failed to parse day from '{}'", self, res);
            return Err(RuleError {
                reason: "Failed to parse day.".into(),
            });
        };
        let Some(month) = res.get(2..4).and_then(|s| s.parse::<i8>().ok()) else {
            clerk::error!("{:?}: failed to parse month from '{}'", self, res);
            return Err(RuleError {
                reason: "Failed to parse month.".into(),
            });
        };
        let Some(year) = res.get(4..6).and_then(|s| s.parse::<i16>().ok()) else {
            clerk::error!("{:?}: failed to parse year from '{}'", self, res);
            return Err(RuleError {
                reason: "Failed to parse year.".into(),
            });
        };
        let dt = match Date::new(year + 2000, month, day) {
            Ok(dt) => dt,
            Err(e) => {
                clerk::error!("{:?}: failed to parse date from '{}'", self, res);
                return Err(RuleError {
                    reason: alloc::format!("{:?}", e).into(),
                });
            }
        };
        Ok((Some(dt), advanced))
    }
}

#[cfg(test)]
mod tests {
    use rax::string::IStrFlowRule;

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
            .apply(input, true)
            .map(|(out, idx)| (out, input.get(idx..).unwrap()));
        insta::assert_debug_snapshot!(name, result)
    }
}
