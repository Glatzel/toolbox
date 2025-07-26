use chrono::NaiveDate;
use rax::str_parser::IRule;

use super::UNTIL_COMMA_DISCARD;

pub struct NmeaDate();

impl IRule for NmeaDate {
    fn name(&self) -> &str { "NmeaDate" }
}

impl<'a> rax::str_parser::IStrFlowRule<'a> for NmeaDate {
    type Output = NaiveDate;
    /// Applies the NmeaUtc rule to the input string.
    /// Parses the UTC time, converts to `DateTime<Utc>` using today's date, and
    /// returns the result and the rest of the string. Logs each step for
    /// debugging.
    fn apply(&self, input: &'a str) -> (std::option::Option<NaiveDate>, &'a str) {
        clerk::trace!("NmeaUtc rule: input='{}'", input);

        let (res, rest) = UNTIL_COMMA_DISCARD.apply(input);
        match res {
            Some(res) => {
                let day = match res.get(0..2).and_then(|s| s.parse::<u32>().ok()) {
                    Some(d) => d,
                    None => {
                        clerk::info!("NmeaDate: failed to parse day from '{}'", res);
                        return (None, rest);
                    }
                };
                let month = match res.get(2..4).and_then(|s| s.parse::<u32>().ok()) {
                    Some(m) => m,
                    None => {
                        clerk::info!("NmeaDate: failed to parse month from '{}'", res);
                        return (None, rest);
                    }
                };
                let year = match res.get(4..6).and_then(|s| s.parse::<i32>().ok()) {
                    Some(y) => y,
                    None => {
                        clerk::info!("NmeaDate: failed to parse year from '{}'", res);
                        return (None, rest);
                    }
                };
                let dt = match NaiveDate::from_ymd_opt(year + 2000, month, day) {
                    Some(date) => {
                        clerk::debug!("NmeaDate: parsed date: {}", date);
                        date
                    }
                    None => {
                        clerk::warn!(
                            "NmeaDate: invalid date: y={}, m={}, d={}",
                            year + 2000,
                            month,
                            day
                        );
                        return (None, rest);
                    }
                };
                (Some(dt), rest)
            }
            None => {
                clerk::warn!("NmeaDate: no comma found in input '{}'", input);
                (None, input)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use rax::str_parser::IStrFlowRule;

    use super::*;

    #[test]
    fn test_nmea_date_valid() {
        let rule = NmeaDate();
        let (date, rest) = rule.apply("110324,foo,bar");
        assert_eq!(date, Some(NaiveDate::from_ymd_opt(2024, 3, 11).unwrap()));
        assert_eq!(rest, "foo,bar");
    }

    #[test]
    fn test_nmea_date_invalid_day() {
        let rule = NmeaDate();
        let (date, rest) = rule.apply("xx0324,foo,bar");
        assert_eq!(date, None);
        assert_eq!(rest, "foo,bar");
    }

    #[test]
    fn test_nmea_date_invalid_month() {
        let rule = NmeaDate();
        let (date, rest) = rule.apply("11xx24,foo,bar");
        assert_eq!(date, None);
        assert_eq!(rest, "foo,bar");
    }

    #[test]
    fn test_nmea_date_invalid_year() {
        let rule = NmeaDate();
        let (date, rest) = rule.apply("1103xx,foo,bar");
        assert_eq!(date, None);
        assert_eq!(rest, "foo,bar");
    }

    #[test]
    fn test_nmea_date_no_comma() {
        let rule = NmeaDate();
        let (date, rest) = rule.apply("110324");
        assert_eq!(date, None);
        assert_eq!(rest, "110324");
    }

    #[test]
    fn test_nmea_date_invalid_date() {
        let rule = NmeaDate();
        let (date, rest) = rule.apply("320224,foo,bar"); // 32nd day is invalid
        assert_eq!(date, None);
        assert_eq!(rest, "foo,bar");
    }
}
