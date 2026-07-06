extern crate alloc;

use alloc::format;
use alloc::string::ToString;
use core::fmt::Debug;

use rax::error::RuleError;
use rax::string::IRule;

/// Rule to validate an NMEA sentence for correct start character and checksum.
/// Returns Ok(()) if the sentence is valid, otherwise returns a mischief error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaValidate;

impl IRule for NmeaValidate {}

impl<'a> rax::string::IGlobalRule<'a> for NmeaValidate {
    type Output = ();

    /// Applies the NmeaValidate rule to the input string.
    /// Checks that the sentence starts with '$', contains a checksum delimiter
    /// '*', and that the calculated checksum matches the provided checksum.
    /// Logs each step for debugging.
    fn apply(&self, input: &'a str) -> Result<(), RuleError> {
        // Log the input at trace level.
        clerk::trace!("NmeaValidate rule: input='{:?}'", input);

        let line = input.trim_end();

        // Check if the sentence starts with '$'.
        if !line.starts_with('$') {
            clerk::error!(
                "{:?}: Invalid sentence prefix: expected '$', got '{}'",
                self,
                line
            );
            return Err(RuleError {
                reason: format!("Invalid sentence prefix: expected '$', got '{}'", line),
            });
        }

        // Find the position of the '*' checksum delimiter.
        let Some(star_pos) = line.find('*') else {
            clerk::error!(
                "{:?}: Missing checksum delimiter: expected '*', got '{}'",
                self,
                line
            );
            return Err(RuleError {
                reason: format!("Missing checksum delimiter: expected '*', got '{}'", line),
            });
        };

        // Split the input into data and checksum string.
        let (data, checksum_str) = line[1..].split_at(star_pos - 1); // skip $
        let checksum_str = &checksum_str[1..];
        clerk::debug!(
            "{:?}: data='{:?}', checksum_str='{:?}'",
            self,
            data,
            checksum_str
        );

        // Check that the checksum string is exactly 2 characters.
        if checksum_str.len() != 2 {
            clerk::error!(
                "{:?}: Invalid checksum length: expected 2, got {}",
                self,
                checksum_str.len()
            );
            return Err(RuleError {
                reason: format!(
                    "Invalid checksum length: expected 2, got {}",
                    checksum_str.len()
                ),
            });
        }

        // Parse the expected checksum from hex.
        let expected = match u8::from_str_radix(checksum_str, 16) {
            Ok(v) => v,
            Err(e) => {
                clerk::error!("{:?}: Invalid hex checksum: {:?}", self, e);
                return Err(RuleError {
                    reason: "Invalid hex checksum".to_string(),
                });
            }
        };

        // Calculate the checksum by XOR'ing all data bytes.
        let calculated = data.bytes().fold(0u8, |acc, b| acc ^ b);
        clerk::debug!(
            "NmeaValidate: calculated checksum={:02X}, expected={:02X}",
            calculated,
            expected
        );

        // Compare calculated and expected checksums.
        if calculated != expected {
            clerk::warn!(
                "{:?}: Checksum mismatch: calculated={:02X}, expected={:02X}",
                self,
                calculated,
                expected
            );
            return Err(RuleError {
                reason: "Checksum mismatch".to_string(),
            });
        }
        clerk::info!("{:?}: sentence is valid: {:?}", self, line);

        Ok(())
    }
}
/// Rule to validate an NMEA sentence for correct start character and checksum.
/// Returns Ok(()) if the sentence is valid, otherwise returns a mischief error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaValidateMultiLine;

impl IRule for NmeaValidateMultiLine {}

impl<'a> rax::string::IGlobalRule<'a> for NmeaValidateMultiLine {
    type Output = ();

    /// Applies the NmeaValidate rule to the input string.
    /// Checks that the sentence starts with '$', contains a checksum delimiter
    /// '*', and that the calculated checksum matches the provided checksum.
    /// Logs each step for debugging.
    fn apply(&self, input: &'a str) -> Result<(), RuleError> {
        // Log the input at trace level.
        clerk::trace!("NmeaValidate rule: input='{:?}'", input);
        let validator = NmeaValidate;
        for line in input.split_inclusive("\n") {
            validator.apply(line)?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    extern crate std;

    use clerk::{LevelFilter, init_log_with_level};
    use rax::string::IGlobalRule;
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
        "$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42",
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
        init_log_with_level(LevelFilter::TRACE);
        let result = NmeaValidate.apply(input);
        insta::assert_debug_snapshot!(name, result)
    }
}
