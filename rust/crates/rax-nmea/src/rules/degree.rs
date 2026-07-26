extern crate alloc;

use alloc::format;
use alloc::string::ToString;

use rax::error::RuleError;
use rax::string::{Decoder, IRule, IStrFlowRule};

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

    fn apply(&self, decoder: &mut Decoder<'a>) -> Result<Self::Output, RuleError> {
        // Log the input at trace level.
        clerk::trace!("{:?}: input='{}'", self, decoder.rest_str());
        let deg_str = UNTIL_COMMA_DISCARD.apply(decoder).map_err(|_| RuleError {
            reason: "Missing degree string.".to_string(),
        })?;
        let sign_str = UNTIL_COMMA_DISCARD.apply(decoder).map_err(|_| RuleError {
            reason: "Missing sign string.".to_string(),
        })?;
        if deg_str.is_empty() && sign_str.is_empty() {
            return Ok(None);
        }
        match (deg_str.parse::<f64>(), sign_str) {
            (Ok(val), "E" | "N") => Ok(Some(val)),
            (Ok(val), "W" | "S") => Ok(Some(-val)),
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
        let mut decoder = Decoder::new(input);
        let result = NmeaDegree
            .apply(&mut decoder)
            .map(|out| (out, decoder.rest_str()));
        insta::assert_debug_snapshot!(name, result)
    }
}
