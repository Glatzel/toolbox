use rax::str_parser::{IRule, IStrFlowRule};

use super::UNTIL_COMMA_DISCARD;

/// Rule to parse an NMEA coordinate in the format "DDDMM.MMM,<sign>,...".
/// Converts the coordinate to decimal degrees, applying the correct sign.
/// Returns a tuple of (decimal_degrees, rest_of_input) if successful, otherwise
/// None.
pub struct NmeaDegree;

impl IRule for NmeaDegree {

}

impl<'a> IStrFlowRule<'a> for NmeaDegree {
    type Output = f64;

    fn apply(&self, input: &'a str) -> (core::option::Option<f64>, &'a str) {
        // Log the input at trace level.
        clerk::trace!("NmeaDegree rule: input='{}'", input);
        let (deg_str, rest1) = UNTIL_COMMA_DISCARD.apply(input);
        let (sign_str, rest2) = UNTIL_COMMA_DISCARD.apply(rest1);
        match (deg_str.and_then(|d| d.parse::<f64>().ok()), sign_str) {
            (Some(val), Some("E" | "N")) => (Some(val), rest2),
            (Some(val), Some("W" | "S")) => (Some(-val), rest2),
            (Some(_), Some(_sign)) => {
                clerk::info!("NmeaDegree: unknown sign '{}'", _sign);
                (None, rest2)
            }
            (_, Some("")) => {
                clerk::info!("NmeaDegree: Null degree: `{}`", input);
                (None, rest2)
            }
            _ => {
                clerk::warn!("NmeaDegree: failed to parse input '{}'", input);
                (None, rest2)
            }
        }
    }
}
#[cfg(test)]
mod test {
    use clerk::{LogLevel, init_log_with_level};
    use float_cmp::assert_approx_eq;

    use super::*;
    #[test]
    fn test_nmea_degree() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaDegree();
        let input = "123.45,N,other_data";
        let (result, rest) = rule.apply(input);
        assert!(result.is_some());
        assert_approx_eq!(f64, result.unwrap(), 123.45);
        assert_eq!(rest, "other_data");
    }
    #[test]
    fn test_nmea_degree_negative() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaDegree();
        let input = "123.45,S,other_data";
        let (result, rest) = rule.apply(input);
        assert!(result.is_some());
        assert_approx_eq!(f64, result.unwrap(), -123.45);
        assert_eq!(rest, "other_data");
    }
    #[test]
    fn test_nmea_degree_invalid() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaDegree();
        let input = "invalid_input";
        let (result, rest) = rule.apply(input);
        assert!(result.is_none());
        assert_eq!(rest, input);
    }
    #[test]
    fn test_nmea_degree_no_second_comma() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaDegree();
        let input = "12345.6789,Nother_data";
        let (result, rest) = rule.apply(input);
        assert!(result.is_none());
        assert_eq!(rest, "Nother_data");
    }
    #[test]
    fn test_nmea_degree_null() {
        init_log_with_level(LogLevel::TRACE);
        let rule = NmeaDegree();
        let input = ",,Nother_data";
        let (result, rest) = rule.apply(input);
        assert!(result.is_none());
        assert_eq!(rest, "Nother_data");
    }
}
