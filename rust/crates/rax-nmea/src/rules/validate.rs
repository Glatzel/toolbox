extern crate alloc;

use alloc::format;
use core::fmt::Debug;

use rax::error::RuleError;
use rax::text::IRule;

/// Validates a single NMEA sentence.
///
/// The sentence must start with `$`, contain a `*` checksum delimiter followed
/// by exactly two hexadecimal digits, and carry a checksum that matches the
/// XOR of the payload bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaValidate;

impl IRule for NmeaValidate {}

impl rax::text::IGlobalRule<true> for NmeaValidate {
    type Output<'a> = ();

    /// Validates the sentence and returns `()` when it is well formed.
    ///
    /// # Errors
    ///
    /// Returns a [`RuleError`] if the `$` prefix, the `*` delimiter, or a
    /// two-digit hexadecimal checksum is missing, or if the checksum does not
    /// match the payload.
    fn apply<'a>(&self, input: &'a str) -> Result<Self::Output<'a>, RuleError> {
        let line = input.trim_end();

        // Check if the sentence starts with '$'.
        if !line.starts_with('$') {
            return Err(RuleError {
                reason: format!("Invalid sentence prefix: expected '$', got '{line}'").into(),
            });
        }

        // Find the position of the '*' checksum delimiter.
        let Some(star_pos) = line.find('*') else {
            return Err(RuleError {
                reason: format!("Missing checksum delimiter: expected '*', got '{line}'").into(),
            });
        };

        // Split the input into data and checksum string.
        let (data, checksum_str) = line[1..].split_at(star_pos - 1); // skip $
        let checksum_str = &checksum_str[1..];

        // Check that the checksum string is exactly 2 characters.
        if checksum_str.len() != 2 {
            return Err(RuleError {
                reason: format!(
                    "Invalid checksum length: expected 2, got {}",
                    checksum_str.len()
                )
                .into(),
            });
        }

        // Parse the expected checksum from hex.
        let expected = match u8::from_str_radix(checksum_str, 16) {
            Ok(v) => v,
            Err(_e) => {
                return Err(RuleError {
                    reason: "Invalid hex checksum".into(),
                });
            }
        };

        // Calculate the checksum by XOR'ing all data bytes.
        let calculated = data.bytes().fold(0_u8, |acc, b| acc ^ b);

        // Compare calculated and expected checksums.
        if calculated != expected {
            return Err(RuleError {
                reason: "Checksum mismatch".into(),
            });
        }

        Ok(())
    }
}
/// Validates every line of a multi-line NMEA message.
///
/// Each line is checked with [`NmeaValidate`]; the first invalid line causes
/// the whole message to be rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaValidateMultiLine;

impl IRule for NmeaValidateMultiLine {}

impl rax::text::IGlobalRule<true> for NmeaValidateMultiLine {
    type Output<'a> = ();

    /// Validates every line in the message and returns `()` when all lines
    /// are well formed.
    ///
    /// # Errors
    ///
    /// Returns a [`RuleError`] from [`NmeaValidate`] for the first invalid
    /// line.
    fn apply<'a>(&self, input: &'a str) -> Result<Self::Output<'a>, RuleError> {
        // Log the input at trace level.

        for line in input.split_inclusive('\n') {
            NmeaValidate.apply(line)?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    extern crate std;

    use rax::text::IGlobalRule;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "valid",
        "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47"
    )]
    #[case("multiline", &[
        "$GPGSV,4,1,15,05,00,000,17,07,06,105,20,08,11,032,15,10,00,000,16*77",
        "$GPGSV,4,2,15,15,40,292,19,17,26,156,17,18,09,330,19,19,07,171,13*7E",
        "$GPGSV,4,3,15,30,45,105,21,01,04,081,,11,18,068,,13,64,241,*73",
        "$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42\n",
    ]
    .join("\n"))]
    #[case(
        "invalid_checksum",
        "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00"
    )]
    #[case(
        "missing_dollar",
        "GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47"
    )]
    #[case(
        "missing_star",
        "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,47"
    )]
    #[case(
        "invalid_hex_checksum",
        "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*ZZ"
    )]
    #[case(
        "short_checksum",
        "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*4"
    )]
    fn test_nmea_validate(#[case] name: &str, #[case] input: &str) {
        let result = NmeaValidate.apply(input);
        insta::assert_debug_snapshot!(name, result)
    }
    #[rstest]
    #[case("1", &[
        "$GPGSV,4,1,15,05,00,000,17,07,06,105,20,08,11,032,15,10,00,000,16*77",
        "$GPGSV,4,2,15,15,40,292,19,17,26,156,17,18,09,330,19,19,07,171,13*7E",
        "$GPGSV,4,3,15,30,45,105,21,01,04,081,,11,18,068,,13,64,241,*73",
        "$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42\n",
    ]
    .join("\n"))]
    #[case("2", &[
        "$GPGSV,4,1,15,05,00,000,17,07,06,105,20,08,11,032,15,10,00,000,16*77",
        "$GPGSV,4,2,15,15,40,292,19,17,26,156,17,18,09,330,19,19,07,171,13*7E",
        "$GPGSV,4,3,15,30,45,105,21,01,04,081,,11,18,068,,13,64,241,*73",
        "$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42",
    ] .join("\n"))]
    fn test_nmea_validate_multiline(#[case] name: &str, #[case] input: &str) {
        let result = NmeaValidateMultiLine.apply(input);
        insta::assert_debug_snapshot!(name, result)
    }
}
