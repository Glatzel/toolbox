extern crate alloc;
use alloc::format;

use jiff::civil::Time;
use rax::error::RuleError;
use rax::text::IRule;

use super::UNTIL_COMMA_DISCARD;
fn parse_field(res: &str, range: core::ops::Range<usize>, label: &str) -> Result<i8, RuleError> {
    let s = res.get(range).ok_or_else(|| RuleError {
        reason: format!("Missing {label} field.").into(),
    })?;

    s.parse::<i8>().map_err(|_| RuleError {
        reason: format!("Failed to parse {label} field.").into(),
    })
}
/// Rule to parse an NMEA UTC time string in the format "hhmmss.sss,...".
///
/// Converts the time to a `DateTime<Utc>` using today's date.
/// Returns a tuple of (`DateTime<Utc>`, `rest_of_input`) if successful,
/// otherwise None.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaTime;

impl IRule for NmeaTime {}

impl rax::text::IFlowRule<true> for NmeaTime {
    type Output<'a> = Option<Time>;
    /// Applies the `NmeaUtc` rule to the input string.
    /// Parses the UTC time, converts to `DateTime<Utc>` using today's date, and
    /// returns the result and the rest of the string. Logs each step for
    /// debugging.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        let Ok((res, advanced)) = UNTIL_COMMA_DISCARD.apply(input) else {
            return Err(RuleError {
                reason: "Missing time string.".into(),
            });
        };
        if res.is_empty() {
            return Ok((None, advanced));
        }

        let nanos = match res.get(7..) {
            Some(frac) => {
                if frac.len() > 9 {
                    return Err(RuleError {
                        reason: "Nano field has too many digits.".into(),
                    });
                }
                let digits = u32::try_from(frac.len()).map_err(|_| RuleError {
                    reason: "Nano field has too many digits.".into(),
                })?;
                if let Ok(frac) = frac.parse::<i32>() {
                    frac * (1_000_000_000 / 10_i32.pow(digits))
                } else {
                    return Err(RuleError {
                        reason: "Failed to parse nano field.".into(),
                    });
                }
            }
            None => 0,
        };

        let hour = parse_field(res, 0..2, "hour")?;
        let min = parse_field(res, 2..4, "minute")?;
        let sec = parse_field(res, 4..6, "second")?;

        let t = match Time::new(hour, min, sec, nanos) {
            Ok(t) => t,
            Err(e) => {
                return Err(RuleError {
                    reason: format!("Failed to parse time field: {e}").into(),
                });
            }
        };
        Ok((Some(t), advanced))
    }
}

#[cfg(test)]
mod tests {

    use rax::text::IFlowRule;

    use super::*;
    #[rstest::rstest]
    #[case("valid", "123456.789,foo,bar")]
    #[case("no_fraction", "235959,foo,bar")]
    #[case("invalid_fraction", "235959.12x,foo,bar")]
    #[case("invalid_time", "235961,foo,bar")]
    #[case("invalid_hour", "xx0000,foo,bar")]
    #[case("invalid_minute", "12xx00,foo,bar")]
    #[case("invalid_second", "1236xx,foo,bar")]
    #[case("invalid_second_range", "12345,foo,bar")]
    #[case("empty", ",foo,bar")]
    #[case("no_comma", "123456")]
    fn test_nmea_time(#[case] name: &str, #[case] input: &str) {
        let result = NmeaTime
            .apply(input)
            .map(|(out, idx)| (out, input.get(idx..).unwrap()));
        insta::assert_debug_snapshot!(name, result)
    }
}
