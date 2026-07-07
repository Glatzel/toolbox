extern crate alloc;
use alloc::vec::Vec;

use derive_getters::Getters;
use rax::string::{Decoder, IDecode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::RaxNmeaError;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;

/// Represents a single satellite's data in a GSV sentence.
#[derive(Debug, Clone, Copy, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Satellite {
    /// Satellite ID, typically a number from 1 to 32.
    svid: Option<u16>,

    /// Elevation in degrees.
    elv: Option<u16>,

    /// Azimuth in degrees.
    az: Option<u16>,

    /// Signal-to-noise ratio.
    cno: Option<u16>,
}

///GNSS satellites in view
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Gsv {
    /// Satellite data
    satellites: Vec<Satellite>,

    /// Signal ID
    signal_id: Option<u16>,
}
impl IDecode<RaxNmeaError> for Gsv {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        clerk::trace!("Gsv::decode: sentence='{}'", parser.full_str());

        // Count the number of lines and satellites
        let line_count = parser.full_str().lines().count();
        clerk::trace!("Gsv::new: line_count={}", line_count);

        // The first line contains the talker, number of lines, and number of satellites
        let satellite_count: usize = parser
            .skip(&UNTIL_COMMA_DISCARD)?
            .skip(&UNTIL_COMMA_DISCARD)?
            .skip(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)?
            .parse()?;
        clerk::trace!("Gsv::new: satellite_count={}", satellite_count);

        // The last line may have fewer than 4 satellites, so we calculate how many
        // satellites are in the last line based on the total count.
        let last_line_satellite_count = satellite_count - 4 * (line_count - 1);
        clerk::trace!(
            "Gsv::new: last_line_satellite_count={}",
            last_line_satellite_count
        );

        let mut satellites = Vec::with_capacity(satellite_count);
        // Parse all but the last line (each has 4 satellites)
        for _i in 0..line_count - 1 {
            for _ in 0..3 {
                satellites.push(Self::parse_satellite(parser, false)?);
            }
            satellites.push(Self::parse_satellite(parser, true)?);
            // Skip any extra fields after the 4th satellite in the line
            parser
                .skip(&UNTIL_NEW_LINE_DISCARD)?
                .skip(&UNTIL_COMMA_DISCARD)?
                .skip(&UNTIL_COMMA_DISCARD)?
                .skip(&UNTIL_COMMA_DISCARD)?
                .skip(&UNTIL_COMMA_DISCARD)?;
        }
        // Parse the last line (may have fewer than 4 satellites)
        if last_line_satellite_count != 0 {
            for _ in 0..(last_line_satellite_count - 1) {
                satellites.push(Self::parse_satellite(parser, false)?);
            }
            satellites.push(Self::parse_satellite(parser, true)?);
            let _ = parser.skip(&UNTIL_COMMA_DISCARD);
        }
        clerk::debug!("satellites: {:?}", satellites);
        clerk::debug!("rest: {}", parser.rest_str());
        let signal_id = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Self {
            satellites,
            signal_id,
        })
    }
}
impl Gsv {
    /// Helper to parse a single satellite entry.
    /// If `last` is true, the SNR field is terminated by a star.
    fn parse_satellite(ctx: &mut Decoder, last: bool) -> Result<Satellite, RaxNmeaError> {
        let id = ctx.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let elevation_degrees = ctx.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let azimuth_degree = ctx.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let snr = if last {
            ctx.take(&UNTIL_COMMA_OR_STAR_KEEP_RIGHT)?.parse_option()?
        } else {
            ctx.take(&UNTIL_COMMA_DISCARD)?.parse_option()?
        };
        Ok(Satellite {
            svid: id,
            elv: elevation_degrees,
            az: azimuth_degree,
            cno: snr,
        })
    }
}

#[cfg(test)]
mod test {
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    extern crate std;
    #[rstest::rstest]
    #[case(
        "1",
        "$GPGSV,3,1,10,25,68,053,47,21,59,306,49,29,56,161,49,31,36,265,49*79\r\n$GPGSV,3,2,10,12,29,048,49,05,22,123,49,18,13,000,49,01,00,000,49*72\r\n$GPGSV,3,3,10,14,00,000,03,16,00,000,27*7C"
    )]
    #[case("2", "$GPGSV,1,1,4,02,35,291,,03,09,129,,05,14,305,,06,38,226,*4E")]
    #[case("3", "$GPGSV,1,1,3,02,35,291,,03,09,129,,05,14,305,*72")]
    #[case("4", "$GPGSV,1,1,0,*65")]
    #[case(
        "5",
        "$GPGSV,3,1,11,05,19,222,36,07,05,090,29,13,84,239,39,14,56,052,36,1*64\r\n$GPGSV,3,2,11,15,50,296,25,17,35,125,24,23,11,319,28,24,16,284,32,1*60\r\n$GPGSV,3,3,11,19,23,147,,20,03,201,,30,28,084,,1*58\r\n"
    )]
    fn test_gsv(#[case] index: &str, #[case] input: &str) -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = Decoder::new(input);
        let gsv = Gsv::decode(&mut decoder)?;
        println!("{gsv:?}");
        insta::assert_json_snapshot!(index, gsv);
        Ok(())
    }
}
