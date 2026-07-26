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
pub struct NmeaDegree;

impl IRule for NmeaDegree {}

impl<'a> IStrFlowRule<'a> for NmeaDegree {
    type Output = Option<f64>;

    fn apply(&self, input: &'a str) -> Result<(Option<f64>, usize), RuleError> {
        // Log the input at trace level.
        clerk::trace!("{:?}: input='{}'", self, input);
        let (deg_str, consumed1) = UNTIL_COMMA_DISCARD.apply(input).map_err(|_| RuleError {
            reason: "Missing degree string.".to_string(),
        })?;
        let (sign_str, consumed2) =
            UNTIL_COMMA_DISCARD
                .apply(&input[consumed1..])
                .map_err(|_| RuleError {
                    reason: "Missing sign string.".to_string(),
                })?;
        if deg_str.is_empty() && sign_str.is_empty() {
            return Ok((None, consumed1 + consumed2));
        }
        match (deg_str.parse::<f64>(), sign_str) {
            (Ok(val), "E" | "N") => Ok((Some(val), consumed1 + consumed2)),
            (Ok(val), "W" | "S") => Ok((Some(-val), consumed1 + consumed2)),
            (Ok(_), _sign) => {
                clerk::error!("{:?}: invalid sign string: '{}'", self, _sign);
                Err(RuleError {
                    reason: format!("invalid sign string: '{}'", _sign),
                })
            }
            (Err(_), _) => {
                clerk::error!("{:?}: invalid coord string: '{}'", self, deg_str);
                Err(RuleError {
                    reason: format!("invalid coord string: '{}'", deg_str),
                })
            }
        }
    }
}
#[cfg(test)]
mod test {
    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[rstest::rstest]
    #[case("valid_positive", "123.45,N,other_data")]
    #[case("valid_negative", "123.45,S,other_data")]
    #[case("invalid", "invalid_input")]
    #[case("no_second_comma", "12345.6789,Nother_data")]
    #[case("null", ",,other_data")]
    fn test_nmea_degree(#[case] name: &str, #[case] input: &str) {
        init_log_with_level(LevelFilter::TRACE);
        let result = NmeaDegree.apply(input);
        insta::assert_debug_snapshot!(name, result)
    }
}
