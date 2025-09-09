use core::fmt::{self, Debug, Display};

use rax::str_parser::IRule;
extern crate alloc;
use alloc::string::ToString;

use crate::RaxNmeaError;
/// Rule to validate an NMEA sentence for correct start character and checksum.
/// Returns Ok(()) if the sentence is valid, otherwise returns a mischief error.
#[derive(Debug)]
pub struct NmeaValidate;
impl Display for NmeaValidate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

impl IRule for NmeaValidate {}

impl<'a> rax::str_parser::IStrGlobalRule<'a> for NmeaValidate {
    type Output = Result<(), RaxNmeaError>;
    /// Applies the NmeaValidate rule to the input string.
    /// Checks that the sentence starts with '$', contains a checksum delimiter
    /// '*', and that the calculated checksum matches the provided checksum.
    /// Logs each step for debugging.
    fn apply(&self, input: &'a str) -> Result<(), RaxNmeaError> {
        // Log the input at trace level.
        clerk::trace!("NmeaValidate rule: input='{}'", input);
        let input = input.trim_end();

        // Check if the sentence starts with '$'.
        if !input.starts_with('$') {
            let e = RaxNmeaError::InvalidSentencePrefix(input.to_string());
            clerk::warn!("{self}: {e}");
            return Err(e);
        }

        // Find the position of the '*' checksum delimiter.
        let Some(star_pos) = input.find('*') else {
            let e = RaxNmeaError::MissingChecksumDelimiter(input.to_string());
            clerk::warn!("{self}: {e}");
            return Err(e);
        };

        // Split the input into data and checksum string.
        let (data, checksum_str) = input[1..].split_at(star_pos - 1); // skip $
        let checksum_str = &checksum_str[1..];
        clerk::debug!("{self}: data='{}', checksum_str='{}'", data, checksum_str);

        // Check that the checksum string is exactly 2 characters.
        if checksum_str.len() != 2 {
            let e = RaxNmeaError::InvalidChecksumLength(checksum_str.len());
            clerk::warn!("{self}: {e}");
            return Err(e);
        }

        // Parse the expected checksum from hex.
        let expected = match u8::from_str_radix(checksum_str, 16) {
            Ok(v) => v,
            Err(e) => {
                let e = RaxNmeaError::InvalidHexChecksum(e);
                clerk::warn!("{self}: {e}");
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
            clerk::warn!("{self}: {e}");
            return Err(e);
        }
        clerk::info!("{self}: sentence is valid: {input}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rax::str_parser::IStrGlobalRule;
    extern crate std;
    use std::{format, vec};

    use super::*;

    #[test]
    fn test_valid_sentence() {
        let rule = NmeaValidate;
        for input in vec![
            "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47",
            "$GPGSV,4,1,15,05,00,000,17,07,06,105,20,08,11,032,15,10,00,000,16*77",
            "$GPGSV,4,2,15,15,40,292,19,17,26,156,17,18,09,330,19,19,07,171,13*7E",
            "$GPGSV,4,3,15,30,45,105,21,01,04,081,,11,18,068,,13,64,241,*73",
            "$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42",
            "$GLGSV,3,1,10,74,43,070,14,66,37,310,19,75,71,306,21,85,16,136,16*65",
            "$GLGSV,3,2,10,67,03,351,18,72,02,198,18,76,21,272,,65,33,234,*64",
            "$GLGSV,3,3,10,84,38,081,,83,20,019,*6B",
            "$GPGSA,A,3,05,07,08,10,15,17,18,19,30,,,,1.2,0.9,0.8*3B",
            "$GPVTG,86.2,T,86.2,M,152.6,N,282.7,K,D*29",
            "$GPRMC,110124,A,5505.330990,N,03858.587325,E,152.6,86.2,310317,8.9,E,D*2E",
            "$GPGGA,110124,5505.330990,N,03858.587325,E,2,09,0.9,2177.0,M,14.0,M,,*7D",
            "$GPGSV,4,1,15,05,00,000,17,07,06,105,20,08,11,032,15,10,00,000,16*77",
            "$GPGSV,4,2,15,15,40,292,19,17,26,156,16,18,09,330,17,19,07,171,13*71",
            "$GPGSV,4,3,15,30,45,105,16,01,04,081,,11,18,068,,13,64,241,*77",
            "$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42",
            "$GLGSV,3,1,10,74,43,070,14,66,37,310,24,75,71,306,21,85,16,136,16*6B",
            "$GLGSV,3,2,10,67,03,351,18,72,02,198,18,76,21,272,,65,33,234,*64",
            "$GLGSV,3,3,10,84,38,081,,83,20,019,*6B",
            "$GPGSA,A,3,05,07,08,10,15,17,18,19,30,,,,1.3,0.9,0.9*3B",
            "$GPVTG,84.6,T,84.6,M,148.8,N,275.6,K,D*25",
            "$GPRMC,110125,A,5505.337580,N,03858.653666,E,148.8,84.6,310317,8.9,E,D*2E",
            "$GPGGA,110125,5505.337580,N,03858.653666,E,2,09,0.9,2177.0,M,14.0,M,,*7E",
            "$GPGSV,4,1,15,05,00,000,17,07,06,105,14,08,11,032,15,10,00,000,16*70",
            "$GPGSV,4,2,15,15,40,292,19,17,26,156,19,18,09,330,17,19,07,171,13*7E",
            "$GPGSV,4,3,15,30,45,105,16,01,04,081,,11,18,068,,13,64,241,*77",
            "$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42",
            "$GLGSV,3,1,10,74,43,070,14,66,37,310,26,75,71,306,21,85,16,136,16*69",
            "$GLGSV,3,2,10,67,03,351,18,72,02,198,13,76,21,272,,65,33,234,*6F",
            "$GLGSV,3,3,10,84,38,081,,83,20,019,*6B",
            "$GPGSA,A,3,05,07,08,10,15,17,18,19,30,,,,1.4,1.1,0.9*35",
            "$GPVTG,83.7,T,83.7,M,146.3,N,271.0,K,D*22",
            "$GPRMC,110126,A,5505.343905,N,03858.720715,E,146.3,83.7,310317,8.9,E,D*2C",
            "$GPGGA,110126,5505.343905,N,03858.720715,E,2,09,1.1,2180.0,M,14.0,M,,*7E",
            "$GPGSV,4,1,15,05,00,000,17,07,06,105,24,08,11,032,15,10,00,000,16*73",
            "$GPGSV,4,2,15,15,40,292,15,17,26,156,19,18,09,330,17,19,07,171,13*72",
            "$GPGSV,4,3,15,30,45,105,17,01,04,081,,11,18,068,,13,64,241,*76",
            "$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42",
            "$GLGSV,3,1,10,74,43,070,18,66,37,310,26,75,71,306,21,85,16,136,16*65",
            "$GLGSV,3,2,10,67,03,351,18,72,02,198,13,76,21,272,,65,33,234,*6F",
            "$GLGSV,3,3,10,84,38,081,,83,20,019,*6B",
            "$GPGSA,A,3,05,07,08,10,15,17,18,19,30,,,,1.3,1.0,0.9*33",
            "$GPVTG,82.3,T,82.3,M,136.0,N,251.9,K,D*2D",
            "$GPRMC,110127,A,5505.349815,N,03858.767316,E,136.0,82.3,310317,8.9,E,D*22",
            "$GPGGA,110127,5505.349815,N,03858.767316,E,2,09,1.0,2184.0,M,14.0,M,,*74",
            "$GPGSV,4,1,15,05,00,000,17,07,06,105,24,08,11,032,15,10,00,000,16*73",
            "$GPGSV,4,2,15,15,40,292,21,17,26,156,19,18,09,330,17,19,07,171,13*75",
            "$GPGSV,4,3,15,30,45,105,17,01,04,081,,11,18,068,,13,64,241,*76",
            "$GPGSV,4,4,15,20,12,265,,24,05,285,,28,73,085,*42",
            "$GLGSV,3,1,10,74,43,070,19,66,37,310,21,75,71,306,21,85,16,136,16*63",
            "$GLGSV,3,2,10,67,03,351,18,72,02,198,13,76,21,272,,65,33,234,*6F",
            "$GLGSV,3,3,10,84,38,081,,83,20,019,*6B",
        ] {
            assert!(rule.apply(input).is_ok());
        }
    }

    #[test]
    fn test_invalid_checksum() {
        let rule = NmeaValidate;
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Checksum mismatch"));
    }

    #[test]
    fn test_missing_dollar() {
        let rule = NmeaValidate;
        let input = "GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("doesn't start with"));
    }

    #[test]
    fn test_missing_star() {
        let rule = NmeaValidate;
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,47";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Missing checksum delimiter"));
    }

    #[test]
    fn test_invalid_hex_checksum() {
        let rule = NmeaValidate;
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*ZZ";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Invalid hex checksum"));
    }

    #[test]
    fn test_short_checksum() {
        let rule = NmeaValidate;
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*4";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("require checksum_str length 2"));
    }
}
