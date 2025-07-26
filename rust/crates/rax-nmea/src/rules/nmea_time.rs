use chrono::NaiveTime;
use rax::str_parser::IRule;

use super::UNTIL_COMMA_DISCARD;

/// Rule to parse an NMEA UTC time string in the format "hhmmss.sss,...".
/// Converts the time to a `DateTime<Utc>` using today's date.
/// Returns a tuple of (DateTime<Utc>, rest_of_input) if successful, otherwise
/// None.
pub struct NmeaTime();

impl IRule for NmeaTime {
    fn name(&self) -> &str { "NmeaUtc" }
}

impl<'a> rax::str_parser::IStrFlowRule<'a> for NmeaTime {
    type Output = NaiveTime;
    /// Applies the NmeaUtc rule to the input string.
    /// Parses the UTC time, converts to `DateTime<Utc>` using today's date, and
    /// returns the result and the rest of the string. Logs each step for
    /// debugging.
    fn apply(&self, input: &'a str) -> (std::option::Option<NaiveTime>, &'a str) {
        clerk::trace!("NmeaUtc rule: input='{}'", input);

        let (res, rest) = UNTIL_COMMA_DISCARD.apply(input);
        let res = match res {
            Some("") | None => {
                clerk::info!("NmeaTime: got empty string.");
                return (None, rest);
            }
            Some(res) => res,
        };

        let nanos = match res.get(7..) {
            Some(frac) => {
                let digits = frac.len() as u32;
                match frac.parse::<u64>() {
                    Ok(frac) => frac * 1_000_000_000 / 10_u64.pow(digits),
                    Err(_) => {
                        clerk::warn!("Can not parse nano:{}", frac);
                        return (None, rest);
                    }
                }
            }
            None => 0,
        };

        let parse_field = |range: std::ops::Range<usize>, label: &str| {
            res.get(range)
                .and_then(|s| s.parse::<u32>().ok())
                .ok_or_else(|| {
                    clerk::warn!("NmeaUtc: failed to parse {} ', input='{}'", label, input);
                })
        };

        let hour = match parse_field(0..2, "hour") {
            Ok(v) => v,
            Err(_) => return (None, rest),
        };
        let min = match parse_field(2..4, "minute") {
            Ok(v) => v,
            Err(_) => return (None, rest),
        };
        let sec = match parse_field(4..6, "second") {
            Ok(v) => v,
            Err(_) => return (None, rest),
        };

        clerk::debug!(
            "NmeaUtc: parsed hour={}, min={}, sec={}, nanos={}",
            hour,
            min,
            sec,
            nanos
        );

        match NaiveTime::from_hms_nano_opt(hour, min, sec, nanos as u32) {
            Some(t) => {
                clerk::debug!("NmeaUtc: parsed time: {}", t);
                (Some(t), rest)
            }
            None => {
                clerk::warn!(
                    "NmeaUtc: invalid time: hour={}, min={}, sec={}, nanos={}",
                    hour,
                    min,
                    sec,
                    nanos
                );
                (None, rest)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;
    use clerk::{LogLevel, init_log_with_level};
    use rax::str_parser::IStrFlowRule;

    use super::*;

    #[test]
    fn test_nmea_utc_valid() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaTime();
        let (dt, rest) = rule.apply("123456.789,foo,bar");
        let dt = dt.expect("Should parse valid UTC time");
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 34);
        assert_eq!(dt.second(), 56);
        assert_eq!(dt.nanosecond(), 789_000_000);

        assert_eq!(rest, "foo,bar");
    }

    #[test]
    fn test_nmea_utc_no_fraction() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaTime();
        let (dt, rest) = rule.apply("235959,rest");
        let dt = dt.expect("Should parse valid time");
        assert_eq!(dt.hour(), 23);
        assert_eq!(dt.minute(), 59);
        assert_eq!(dt.second(), 59);
        assert_eq!(dt.nanosecond(), 0);

        assert_eq!(rest, "rest");
    }

    #[test]
    fn test_nmea_utc_invalid_hour() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaTime();
        let (dt, rest) = rule.apply("xx3456,foo");
        assert!(dt.is_none());
        assert_eq!(rest, "foo");
    }

    #[test]
    fn test_nmea_utc_invalid_minute() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaTime();
        let (dt, rest) = rule.apply("12xx56,foo");
        assert!(dt.is_none());
        assert_eq!(rest, "foo");
    }

    #[test]
    fn test_nmea_utc_invalid_second() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaTime();
        let (dt, rest) = rule.apply("1234xx,foo");
        assert!(dt.is_none());
        assert_eq!(rest, "foo");
    }

    #[test]
    fn test_nmea_utc_empty() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaTime();
        let (dt, rest) = rule.apply(",foo");
        assert!(dt.is_none());
        assert_eq!(rest, "foo");
    }

    #[test]
    fn test_nmea_utc_no_comma() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaTime();
        let (dt, rest) = rule.apply("123456");
        assert!(dt.is_none());
        assert_eq!(rest, "123456");
    }
}
