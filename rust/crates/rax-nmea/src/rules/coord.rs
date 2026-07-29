extern crate alloc;
use alloc::format;

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
    fn apply(&self, input: &'a str, is_ascii: bool) -> Result<(Self::Output, usize), RuleError> {
        clerk::trace!("NmeaCoord rule: input='{}'", input);

        let (num_str, advanced1) =
            UNTIL_COMMA_DISCARD
                .apply(input, is_ascii)
                .map_err(|_| RuleError {
                    reason: "Missing number string.".into(),
                })?;

        let (sign_str, advanced2) = UNTIL_COMMA_DISCARD
            .apply(unsafe { input.get_unchecked(advanced1..) }, is_ascii)
            .map_err(|_| RuleError {
                reason: "Missing sign string.".into(),
            })?;
        let advanced = advanced1 + advanced2;
        if num_str.is_empty() && sign_str.is_empty() {
            return Ok((None, advanced));
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
                Ok((Some(result), advanced))
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
                Ok((Some(result), advanced))
            }
            (Ok(_), _sign) => {
                clerk::error!("{:?}: invalid sign string: '{}'", self, _sign);
                Err(RuleError {
                    reason: format!("invalid sign string: '{}'", _sign).into(),
                })
            }
            (Err(_), _) => {
                clerk::error!("{:?}: invalid coord string: '{}'", self, num_str);
                Err(RuleError {
                    reason: format!("invalid coord string: '{}'", num_str).into(),
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
        let result = NmeaCoord
            .apply(input, true)
            .map(|(out, idx)| (out, input.get(idx..).unwrap()));
        insta::assert_debug_snapshot!(name, result)
    }
}
