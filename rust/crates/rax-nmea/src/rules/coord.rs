extern crate alloc;
use alloc::format;

use rax::error::RuleError;
use rax::text::{IFlowRule, IRule};

use super::UNTIL_COMMA_DISCARD;

/// Parses an NMEA coordinate in the `DDDMM.MMM,sign` format.
///
/// The numeric field is interpreted as degrees and decimal minutes and
/// converted to decimal degrees. The `N` and `E` signs produce positive
/// values; the `S` and `W` signs produce negative values. Two consecutive
/// empty fields produce [`None`].
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
impl IFlowRule<true> for NmeaCoord {
    type Output<'a> = Option<f64>;
    /// Parses one coordinate field and returns it together with the number of
    /// bytes consumed.
    ///
    /// # Errors
    ///
    /// Returns a [`RuleError`] if the numeric or sign field is missing, if the
    /// number is not a valid floating-point value, or if the sign is not one
    /// of `N`, `S`, `E`, or `W`.
    fn apply<'a>(&self, input: &'a str) -> Result<(Self::Output<'a>, usize), RuleError> {
        let (num_str, advanced1) = UNTIL_COMMA_DISCARD.apply(input).map_err(|_| RuleError {
            reason: "Missing number string.".into(),
        })?;

        let (sign_str, advanced2) = UNTIL_COMMA_DISCARD
            .apply(unsafe { input.get_unchecked(advanced1..) })
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

                Ok((Some(result), advanced))
            }
            (Ok(v), "S" | "W") => {
                let result = -Self::convert_to_decimal_degrees(v);

                Ok((Some(result), advanced))
            }
            (Ok(_), sign) => Err(RuleError {
                reason: format!("invalid sign string: '{sign}'").into(),
            }),
            (Err(_), _) => Err(RuleError {
                reason: format!("invalid coord string: '{num_str}'").into(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {

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
        let result = NmeaCoord
            .apply(input)
            .map(|(out, idx)| (out, input.get(idx..).unwrap()));
        insta::assert_debug_snapshot!(name, result)
    }
}
