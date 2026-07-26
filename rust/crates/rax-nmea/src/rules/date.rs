extern crate alloc;

use alloc::string::ToString;
use core::fmt::Debug;

use chrono::NaiveDate;
use rax::error::RuleError;
use rax::string::{Decoder, IRule};

use super::UNTIL_COMMA_DISCARD;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaDate;

impl IRule for NmeaDate {}

impl<'a> rax::string::IStrFlowRule<'a> for NmeaDate {
    type Output = Option<NaiveDate>;
    /// Applies the NmeaUtc rule to the input string.
    /// Parses the UTC time, converts to `DateTime<Utc>` using today's date, and
    /// returns the result and the rest of the string. Logs each step for
    /// debugging.
    fn apply(&self, decoder: &mut Decoder<'a>) -> Result<Self::Output, RuleError> {
        clerk::trace!("NmeaUtc rule: input='{}'", decoder.rest_str());

        let res = UNTIL_COMMA_DISCARD.apply(decoder).map_err(|_| RuleError {
            reason: "Missing Date string.".to_string(),
        })?;
        if res.is_empty() {
            return Ok(None);
        }

        let day = match res.get(0..2).and_then(|s| s.parse::<u32>().ok()) {
            Some(d) => d,
            None => {
                clerk::error!("{:?}: failed to parse day from '{}'", self, res);
                return Err(RuleError {
                    reason: "Failed to parse day.".to_string(),
                });
            }
        };
        let month = match res.get(2..4).and_then(|s| s.parse::<u32>().ok()) {
            Some(m) => m,
            None => {
                clerk::error!("{:?}: failed to parse month from '{}'", self, res);
                return Err(RuleError {
                    reason: "Failed to parse month.".to_string(),
                });
            }
        };
        let year = match res.get(4..6).and_then(|s| s.parse::<i32>().ok()) {
            Some(y) => y,
            None => {
                clerk::error!("{:?}: failed to parse year from '{}'", self, res);
                return Err(RuleError {
                    reason: "Failed to parse year.".to_string(),
                });
            }
        };
        let dt = match NaiveDate::from_ymd_opt(year + 2000, month, day) {
            Some(date) => {
                clerk::debug!("{:?}: parsed date: {}", self, date);
                date
            }
            None => {
                clerk::error!(
                    "{:?}: invalid date: y={}, m={}, d={}",
                    self,
                    year + 2000,
                    month,
                    day
                );
                return Err(RuleError {
                    reason: "Invalid date.".to_string(),
                });
            }
        };
        Ok(Some(dt))
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
        let mut decoder = Decoder::new(input);
        let result = NmeaDate
            .apply(&mut decoder)
            .map(|out| (out, decoder.rest_str()));
        insta::assert_debug_snapshot!(name, result)
    }
}
