use core::fmt::Debug;

use rax::string::IRule;
extern crate alloc;
use alloc::string::ToString;

use crate::RaxNmeaError;
/// Rule to validate an NMEA sentence for correct start character and checksum.
/// Returns Ok(()) if the sentence is valid, otherwise returns a mischief error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NmeaValidate;

impl IRule for NmeaValidate {}

impl<'a> rax::string::IGlobalRule<'a> for NmeaValidate {
    type Output = Result<(), RaxNmeaError>;
    /// Applies the NmeaValidate rule to the input string.
    /// Checks that the sentence starts with '$', contains a checksum delimiter
    /// '*', and that the calculated checksum matches the provided checksum.
    /// Logs each step for debugging.
    fn apply(&self, input: &'a str) -> Result<(), RaxNmeaError> {
        // Log the input at trace level.
        clerk::trace!("NmeaValidate rule: input='{:?}'", input);

        let line = input.trim_end();

        // Check if the sentence starts with '$'.
        if !line.starts_with('$') {
            let e = RaxNmeaError::InvalidSentencePrefix(line.to_string());
            clerk::warn!("{:?}: {:?}", self, e);
            return Err(e);
        }

        // Find the position of the '*' checksum delimiter.
        let Some(star_pos) = line.find('*') else {
            let e = RaxNmeaError::MissingChecksumDelimiter(line.to_string());
            clerk::warn!("{:?}: {:?}", self, e);
            return Err(e);
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
            let e = RaxNmeaError::InvalidChecksumLength(checksum_str.len());
            clerk::warn!("{:?}: {:?}", self, e);
            return Err(e);
        }

        // Parse the expected checksum from hex.
        let expected = match u8::from_str_radix(checksum_str, 16) {
            Ok(v) => v,
            Err(e) => {
                let e = RaxNmeaError::InvalidHexChecksum(e);
                clerk::warn!("{:?}: {:?}", self, e);
                return Err(e);
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
            let e = RaxNmeaError::ChecksumMismatch {
                calculated,
                expected,
            };
            clerk::warn!("{:?}: {:?}", self, e);
            return Err(e);
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
    type Output = Result<(), RaxNmeaError>;
    /// Applies the NmeaValidate rule to the input string.
    /// Checks that the sentence starts with '$', contains a checksum delimiter
    /// '*', and that the calculated checksum matches the provided checksum.
    /// Logs each step for debugging.
    fn apply(&self, input: &'a str) -> Result<(), RaxNmeaError> {
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
    use rax::string::IGlobalRule;
    use rstest::rstest;
    extern crate std;
    use std::{format, println};

    use super::*;

    #[rstest]
    #[case("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47")]
    #[case("$GPGSV,4,1,15,05,00,000,17,07,06,105,20,08,11,032,15,10,00,000,16*77")]
    #[case("$GPGSV,4,2,15,15,40,292,19,17,26,156,17,18,09,330,19,19,07,171,13*7E")]
    #[case("$GPGSV,4,3,15,30,45,105,21,01,04,081,,11,18,068,,13,64,241,*73")]
    #[case("$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42")]
    #[case("$GLGSV,3,1,10,74,43,070,14,66,37,310,19,75,71,306,21,85,16,136,16*65")]
    #[case("$GLGSV,3,2,10,67,03,351,18,72,02,198,18,76,21,272,,65,33,234,*64")]
    #[case("$GLGSV,3,3,10,84,38,081,,83,20,019,*6B")]
    #[case("$GPGSA,A,3,05,07,08,10,15,17,18,19,30,,,,1.2,0.9,0.8*3B")]
    #[case("$GPVTG,86.2,T,86.2,M,152.6,N,282.7,K,D*29")]
    #[case("$GPRMC,110124,A,5505.330990,N,03858.587325,E,152.6,86.2,310317,8.9,E,D*2E")]
    #[case("$GPGGA,110124,5505.330990,N,03858.587325,E,2,09,0.9,2177.0,M,14.0,M,,*7D")]
    #[case("$GPGSV,4,1,15,05,00,000,17,07,06,105,20,08,11,032,15,10,00,000,16*77")]
    #[case("$GPGSV,4,2,15,15,40,292,19,17,26,156,16,18,09,330,17,19,07,171,13*71")]
    #[case("$GPGSV,4,3,15,30,45,105,16,01,04,081,,11,18,068,,13,64,241,*77")]
    #[case("$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42")]
    #[case("$GLGSV,3,1,10,74,43,070,14,66,37,310,24,75,71,306,21,85,16,136,16*6B")]
    #[case("$GLGSV,3,2,10,67,03,351,18,72,02,198,18,76,21,272,,65,33,234,*64")]
    #[case("$GLGSV,3,3,10,84,38,081,,83,20,019,*6B")]
    #[case("$GPGSA,A,3,05,07,08,10,15,17,18,19,30,,,,1.3,0.9,0.9*3B")]
    #[case("$GPVTG,84.6,T,84.6,M,148.8,N,275.6,K,D*25")]
    #[case("$GPRMC,110125,A,5505.337580,N,03858.653666,E,148.8,84.6,310317,8.9,E,D*2E")]
    #[case("$GPGGA,110125,5505.337580,N,03858.653666,E,2,09,0.9,2177.0,M,14.0,M,,*7E")]
    #[case("$GPGSV,4,1,15,05,00,000,17,07,06,105,14,08,11,032,15,10,00,000,16*70")]
    #[case("$GPGSV,4,2,15,15,40,292,19,17,26,156,19,18,09,330,17,19,07,171,13*7E")]
    #[case("$GPGSV,4,3,15,30,45,105,16,01,04,081,,11,18,068,,13,64,241,*77")]
    #[case("$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42")]
    #[case("$GLGSV,3,1,10,74,43,070,14,66,37,310,26,75,71,306,21,85,16,136,16*69")]
    #[case("$GLGSV,3,2,10,67,03,351,18,72,02,198,13,76,21,272,,65,33,234,*6F")]
    #[case("$GLGSV,3,3,10,84,38,081,,83,20,019,*6B")]
    #[case("$GPGSA,A,3,05,07,08,10,15,17,18,19,30,,,,1.4,1.1,0.9*35")]
    #[case("$GPVTG,83.7,T,83.7,M,146.3,N,271.0,K,D*22")]
    #[case("$GPRMC,110126,A,5505.343905,N,03858.720715,E,146.3,83.7,310317,8.9,E,D*2C")]
    #[case("$GPGGA,110126,5505.343905,N,03858.720715,E,2,09,1.1,2180.0,M,14.0,M,,*7E")]
    #[case("$GPGSV,4,1,15,05,00,000,17,07,06,105,24,08,11,032,15,10,00,000,16*73")]
    #[case("$GPGSV,4,2,15,15,40,292,15,17,26,156,19,18,09,330,17,19,07,171,13*72")]
    #[case("$GPGSV,4,3,15,30,45,105,17,01,04,081,,11,18,068,,13,64,241,*76")]
    #[case("$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42")]
    #[case("$GLGSV,3,1,10,74,43,070,18,66,37,310,26,75,71,306,21,85,16,136,16*65")]
    #[case("$GLGSV,3,2,10,67,03,351,18,72,02,198,13,76,21,272,,65,33,234,*6F")]
    #[case("$GLGSV,3,3,10,84,38,081,,83,20,019,*6B")]
    #[case("$GPGSA,A,3,05,07,08,10,15,17,18,19,30,,,,1.3,1.0,0.9*33")]
    #[case("$GPVTG,82.3,T,82.3,M,136.0,N,251.9,K,D*2D")]
    #[case("$GPRMC,110127,A,5505.349815,N,03858.767316,E,136.0,82.3,310317,8.9,E,D*22")]
    #[case("$GPGGA,110127,5505.349815,N,03858.767316,E,2,09,1.0,2184.0,M,14.0,M,,*74")]
    #[case("$GPGSV,4,1,15,05,00,000,17,07,06,105,24,08,11,032,15,10,00,000,16*73")]
    #[case("$GPGSV,4,2,15,15,40,292,21,17,26,156,19,18,09,330,17,19,07,171,13*75")]
    #[case("$GPGSV,4,3,15,30,45,105,17,01,04,081,,11,18,068,,13,64,241,*76")]
    #[case("$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42")]
    #[case("$GLGSV,3,1,10,74,43,070,19,66,37,310,21,75,71,306,21,85,16,136,16*63")]
    #[case("$GLGSV,3,2,10,67,03,351,18,72,02,198,13,76,21,272,,65,33,234,*6F")]
    #[case("$GLGSV,3,3,10,84,38,081,,83,20,019,*6B")]
    fn test_valid_sentence(#[case] input: &str) {
        let rule = NmeaValidate;
        assert!(rule.apply(input).is_ok());
    }
    #[test]
    fn test_valid_multiline() -> mischief::Result<()> {
        let rule = NmeaValidateMultiLine;
        let input = [
            "$GPGSV,4,1,15,05,00,000,17,07,06,10520,08,11,032,15,10,00,000,16*77",
            "$GPGSV,4,2,15,15,40,292,19,17,26,156,17,18,09,330,19,19,07,171,13*7E",
            "$GPGSV,4,3,15,30,45,105,21,01,04,081,,11,18,068,,13,64,241,*73",
            "$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42",
        ]
        .join("\n");
        rule.apply(&input)?;
        Ok(())
    }
    #[test]
    fn test_invalid_checksum() {
        let rule = NmeaValidate;
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        println!("{msg}");
        assert!(msg.contains("ChecksumMismatch"));
    }

    #[test]
    fn test_missing_dollar() {
        let rule = NmeaValidate;
        let input = "GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        println!("{msg}");
        assert!(msg.contains("InvalidSentencePrefix"));
    }

    #[test]
    fn test_missing_star() {
        let rule = NmeaValidate;
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,47";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        println!("{msg}");
        assert!(msg.contains("MissingChecksumDelimiter"));
    }

    #[test]
    fn test_invalid_hex_checksum() {
        let rule = NmeaValidate;
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*ZZ";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        println!("{msg}");
        assert!(msg.contains("InvalidHexChecksum"));
    }

    #[test]
    fn test_short_checksum() {
        let rule = NmeaValidate;
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*4";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        println!("{msg}");
        assert!(msg.contains("InvalidChecksumLength"));
    }
}
