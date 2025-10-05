use core::fmt::{self, Display};

use rax::str_parser::{IRule, IStrFlowRule};

use super::UNTIL_COMMA_DISCARD;

/// Rule to parse an NMEA coordinate in the format "DDDMM.MMM,<sign>,...".
/// Converts the coordinate to decimal degrees, applying the correct sign.
/// Returns a tuple of (decimal_degrees, rest_of_input) if successful, otherwise
/// None.
#[derive(Debug)]
pub struct NmeaCoord;
impl Display for NmeaCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

impl IRule for NmeaCoord {}
impl NmeaCoord {
    fn convert_to_decimal_degrees(v: f64) -> f64 {
        let deg = (v / 100.0).floor();
        let min = v - deg * 100.0;
        deg + min / 60.0
    }
}
impl<'a> IStrFlowRule<'a> for NmeaCoord {
    type Output = f64;
    /// Applies the NmeaCoord rule to the input string.
    /// Parses the coordinate and sign, converts to decimal degrees, and returns
    /// the result and the rest of the string. Logs each step for debugging.
    fn apply(&self, input: &'a str) -> (core::option::Option<f64>, &'a str) {
        clerk::trace!("NmeaCoord rule: input='{}'", input);

        let (num_str, rest1) = UNTIL_COMMA_DISCARD.apply(input);
        let (sign_str, rest2) = UNTIL_COMMA_DISCARD.apply(rest1);

        match (num_str.and_then(|s| s.parse::<f64>().ok()), sign_str) {
            (Some(v), Some(_sign @ ("N" | "E"))) => {
                let result = Self::convert_to_decimal_degrees(v);
                clerk::debug!(
                    "{}: positive sign '{}', deg={}, min={}, result={}",
                    self,
                    _sign,
                    (v / 100.0).floor(),
                    v - (v / 100.0).floor() * 100.0,
                    result
                );
                (Some(result), rest2)
            }
            (Some(v), Some(_sign @ ("S" | "W"))) => {
                let result = -Self::convert_to_decimal_degrees(v);
                clerk::debug!(
                    "{}: negative sign '{}', deg={}, min={}, result={}",
                    self,
                    _sign,
                    (v / 100.0).floor(),
                    v - (v / 100.0).floor() * 100.0,
                    result
                );
                (Some(result), rest2)
            }
            (Some(_), Some(_sign)) => {
                clerk::info!("{}: invalid sign '{}'", self, _sign);
                (None, rest2)
            }
            (_, Some("")) => {
                clerk::info!("{}: Null coord: '{}'", self, input);
                (None, rest2)
            }
            _ => {
                clerk::warn!("{}: Invalid input: '{}'", self, input);
                (None, rest2)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use clerk::{LogLevel, init_log_with_level};

    use super::*;

    #[test]
    fn test_nmea_coord_east() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaCoord;
        // 12319.123,E,rest
        let input = "12319.123,E,rest";
        let (val, rest) = rule.apply(input);
        // 12319.123 means 123 degrees, 19.123 minutes
        // deg = 123, min = 19.123, value = 123 + 19.123/60
        let expected = 123.0 + 19.123 / 60.0;
        assert_eq!(val, Some(expected));
        assert_eq!(rest, "rest");
    }

    #[test]
    fn test_nmea_coord_west() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaCoord;
        let input = "12319.123,W,foo";

        let (val, rest) = rule.apply(input);
        let expected = -(123.0 + 19.123 / 60.0);
        assert_eq!(val, Some(expected));
        assert_eq!(rest, "foo");
    }

    #[test]
    fn test_nmea_coord_north() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaCoord;
        let input = "4807.038,N,bar";

        let (val, rest) = rule.apply(input);
        let expected = 48.0 + 7.038 / 60.0;
        float_cmp::assert_approx_eq!(f64, val.unwrap(), expected);
        assert_eq!(rest, "bar");
    }

    #[test]
    fn test_nmea_coord_south() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaCoord;
        let input = "4807.038,S,xyz";

        let (val, rest) = rule.apply(input);
        let expected = -(48.0 + 7.038 / 60.0);
        float_cmp::assert_approx_eq!(f64, val.unwrap(), expected);
        assert_eq!(rest, "xyz");
    }

    #[test]
    fn test_nmea_coord_invalid_sign() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaCoord;
        let input = "12319.123,X,rest";

        let (val, rest) = rule.apply(input);
        assert_eq!(val, None);
        assert_eq!(rest, "rest");
    }

    #[test]
    fn test_nmea_coord_invalid_number() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaCoord;
        let input = "notanumber,E,rest";

        let (val, rest) = rule.apply(input);
        assert_eq!(val, None);
        assert_eq!(rest, "rest");
    }

    #[test]
    fn test_nmea_coord_missing_comma() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaCoord;
        let input = "12319.123Erest";

        let (val, rest) = rule.apply(input);
        assert_eq!(val, None);
        assert_eq!(rest, "12319.123Erest");
    }
}
