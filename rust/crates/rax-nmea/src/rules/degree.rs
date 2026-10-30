extern crate alloc;

use alloc::format;

use rax::error::RuleError;
use rax::text::{IFlowRule, IRule};

use super::UNTIL_COMMA_DISCARD;

/// Parses a signed degree value in the `DDD.dddd,sign` format.
///
/// The numeric field is already in decimal degrees. The `N` and `E` signs
/// produce positive values; the `S` and `W` signs produce negative values.
/// Two consecutive empty fields produce [`None`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaDegree;

impl IRule for NmeaDegree {}

impl IFlowRule<true> for NmeaDegree {
    type Output<'a> = Option<f64>;

    /// Parses one signed degree field and returns it together with the number
    /// of bytes consumed.
    ///
    /// # Errors
    ///
    /// Returns a [`RuleError`] if the numeric or sign field is missing, if the
    /// number is not a valid floating-point value, or if the sign is not one
    /// of `N`, `S`, `E`, or `W`.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        // Log the input at trace level.
        let (deg_str, advanced1) = UNTIL_COMMA_DISCARD.apply(input).map_err(|_| RuleError {
            reason: "Missing degree string.".into(),
        })?;
        let (sign_str, advanced2) = UNTIL_COMMA_DISCARD
            .apply(unsafe { input.get_unchecked(advanced1..) })
            .map_err(|_| RuleError {
                reason: "Missing sign string.".into(),
            })?;
        let advanced = advanced1 + advanced2;
        if deg_str.is_empty() && sign_str.is_empty() {
            return Ok((None, advanced));
        }
        match (deg_str.parse::<f64>(), sign_str) {
            (Ok(val), "E" | "N") => Ok((Some(val), advanced)),
            (Ok(val), "W" | "S") => Ok((Some(-val), advanced)),
            (Ok(_), sign) => Err(RuleError {
                reason: format!("invalid sign string: '{sign}'").into(),
            }),
            (Err(_), _) => Err(RuleError {
                reason: format!("invalid coord string: '{deg_str}'").into(),
            }),
        }
    }
}
#[cfg(test)]
mod test {

    use super::*;
    #[rstest::rstest]
    #[case("valid_positive", "123.45,N,other_data")]
    #[case("valid_negative", "123.45,S,other_data")]
    #[case("invalid", "invalid_input")]
    #[case("no_second_comma", "12345.6789,Nother_data")]
    #[case("null", ",,other_data")]
    fn test_nmea_degree(#[case] name: &str, #[case] input: &str) {
        let result = NmeaDegree
            .apply(input)
            .map(|(out, idx)| (out, input.get(idx..).unwrap()));
        insta::assert_debug_snapshot!(name, result)
    }
}
