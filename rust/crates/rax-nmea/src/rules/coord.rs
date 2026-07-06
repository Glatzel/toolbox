extern crate alloc;
use alloc::format;
use alloc::string::ToString;

use rax::error::RuleError;
use rax::string::{IRule, IStrFlowRule};

use super::UNTIL_COMMA_DISCARD;

/// Rule to parse an NMEA coordinate in the format "DDDMM.MMM,sign,...".
/// Converts the coordinate to decimal degrees, applying the correct sign.
/// Returns a tuple of (decimal_degrees, rest_of_input) if successful, otherwise
/// None.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaCoord;

impl IRule for NmeaCoord {}
impl NmeaCoord {
    fn convert_to_decimal_degrees(v: f64) -> f64 {
        let deg = (v / 100.0).floor();
        let min = v - deg * 100.0;
        deg + min / 60.0
    }
}
impl<'a> IStrFlowRule<'a> for NmeaCoord {
    type Output = Option<f64>;
    /// Applies the NmeaCoord rule to the input string.
    /// Parses the coordinate and sign, converts to decimal degrees, and returns
    /// the result and the rest of the string. Logs each step for debugging.
    fn apply(&self, input: &'a str) -> Result<(Self::Output, &'a str), RuleError> {
        clerk::trace!("NmeaCoord rule: input='{}'", input);

        let (num_str, rest1) = match UNTIL_COMMA_DISCARD.apply(input) {
            Ok(result) => result,
            Err(_) => {
                return Err(RuleError {
                    reason: "Missing number string.".to_string(),
                });
            }
        };
        let (sign_str, rest2) = match UNTIL_COMMA_DISCARD.apply(rest1) {
            Ok(result) => result,
            Err(_) => {
                return Err(RuleError {
                    reason: "Missing sign string.".to_string(),
                });
            }
        };
        if num_str.is_empty() && sign_str.is_empty() {
            return Ok((None, rest2));
        }

        match (num_str.parse::<f64>(), sign_str) {
            (Ok(v), "N" | "E") => {
                let result = Self::convert_to_decimal_degrees(v);
                clerk::debug!(
                    "{:?}: positive sign '{}', deg={}, min={}, result={}",
                    self,
                    sign_str,
                    (v / 100.0).floor(),
                    v - (v / 100.0).floor() * 100.0,
                    result
                );
                Ok((Some(result), rest2))
            }
            (Ok(v), "S" | "W") => {
                let result = -Self::convert_to_decimal_degrees(v);
                clerk::debug!(
                    "{:?}: negative sign '{}', deg={}, min={}, result={}",
                    self,
                    sign_str,
                    (v / 100.0).floor(),
                    v - (v / 100.0).floor() * 100.0,
                    result
                );
                Ok((Some(result), rest2))
            }
            (Ok(_), _sign) => {
                clerk::error!("{:?}: invalid sign string: '{}'", self, _sign);
                Err(RuleError {
                    reason: format!("invalid sign string: '{}'", _sign),
                })
            }
            (Err(_), _) => {
                clerk::error!("{:?}: invalid coord string: '{}'", self, num_str);
                Err(RuleError {
                    reason: format!("invalid coord string: '{}'", num_str),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[rstest::rstest]
    #[case("east", "12319.123,E,rest")]
    #[case("west", "12319.123,W,foo")]
    #[case("north", "4807.038,N,bar")]
    #[case("south", "4807.038,S,xyz")]
    #[case("invalid_sign", "12319.123,X,rest")]
    #[case("invalid_number", "abc123.456,N,foo")]
    #[case("missing_comma", "12319.123Erest")]
    #[case("empty", ",,bar")]
    fn test_nmea_coord(#[case] name: &str, #[case] input: &str) {
        init_log_with_level(LevelFilter::TRACE);
        let result = NmeaCoord.apply(input);
        insta::assert_debug_snapshot!(name, result)
    }
}
