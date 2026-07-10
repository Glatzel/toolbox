extern crate alloc;
use alloc::format;
use alloc::string::ToString;

use chrono::NaiveTime;
use rax::error::RuleError;
use rax::string::IRule;

use super::UNTIL_COMMA_DISCARD;
fn parse_field(
    res: &str,
    range: core::ops::Range<usize>,
    label: &str,
    parser: &impl core::fmt::Debug,
    input: &str,
) -> Result<u32, RuleError> {
    let s = res.get(range).ok_or_else(|| {
        clerk::error!("{:?}: missing {}, input='{:?}'", parser, label, input);
        RuleError {
            reason: format!("Missing {} field.", label),
        }
    })?;

    s.parse::<u32>().map_err(|_| {
        clerk::error!(
            "{:?}: failed to parse {}, value='{}', input={:?}",
            parser,
            label,
            s,
            input
        );
        RuleError {
            reason: format!("Failed to parse {} field.", label),
        }
    })
}
/// Rule to parse an NMEA UTC time string in the format "hhmmss.sss,...".
/// Converts the time to a `DateTime<Utc>` using today's date.
/// Returns a tuple of (`DateTime<Utc>`, rest_of_input) if successful, otherwise
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaTime;

impl IRule for NmeaTime {}

impl<'a> rax::string::IStrFlowRule<'a> for NmeaTime {
    type Output = Option<NaiveTime>;
    /// Applies the NmeaUtc rule to the input string.
    /// Parses the UTC time, converts to `DateTime<Utc>` using today's date, and
    /// returns the result and the rest of the string. Logs each step for
    /// debugging.
    fn apply(&self, input: &'a str) -> Result<(Option<NaiveTime>, &'a str), RuleError> {
        clerk::trace!("{:?}: input='{}'", self, input);

        let (res, rest) = match UNTIL_COMMA_DISCARD.apply(input) {
            Ok(result) => result,
            Err(_) => {
                return Err(RuleError {
                    reason: "Missing time string.".to_string(),
                });
            }
        };
        if res.is_empty() {
            return Ok((None, rest));
        }

        let nanos = match res.get(7..) {
            Some(frac) => {
                if frac.len() > 9 {
                    return Err(RuleError {
                        reason: "Nano field has too many digits.".into(),
                    });
                }
                let digits = frac.len() as u32;
                match frac.parse::<u64>() {
                    Ok(frac) => frac * (1_000_000_000 / 10_u64.pow(digits)),
                    Err(_) => {
                        clerk::error!("Can not parse nano:{}", frac);
                        return Err(RuleError {
                            reason: "Failed to parse nano field.".into(),
                        });
                    }
                }
            }
            None => 0,
        };

        let hour = parse_field(res, 0..2, "hour", self, input)?;
        let min = parse_field(res, 2..4, "minute", self, input)?;
        let sec = parse_field(res, 4..6, "second", self, input)?;

        clerk::debug!(
            "{:?}: parsed hour={}, min={}, sec={}, nanos={}",
            self,
            hour,
            min,
            sec,
            nanos
        );

        match NaiveTime::from_hms_nano_opt(hour, min, sec, nanos as u32) {
            Some(t) => {
                clerk::debug!("{:?}: parsed time: {}", self, t);
                Ok((Some(t), rest))
            }
            None => {
                clerk::error!(
                    "{:?}: invalid time: hour={}, min={}, sec={}, nanos={}",
                    self,
                    hour,
                    min,
                    sec,
                    nanos
                );
                Err(RuleError {
                    reason: format!(
                        "invalid time: hour={}, min={}, sec={}, nanos={}",
                        hour, min, sec, nanos
                    ),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use clerk::{LevelFilter, init_log_with_level};
    use rax::string::IStrFlowRule;

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
        init_log_with_level(LevelFilter::TRACE);
        let result = NmeaTime.apply(input);
        insta::assert_debug_snapshot!(name, result)
    }
}
