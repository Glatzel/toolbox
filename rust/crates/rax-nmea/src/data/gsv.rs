use std::fmt;

use rax::str_parser::{IStrGlobalRule, ParseOptExt, StrParserContext};
use serde::{Deserialize, Serialize};

use crate::data::{INmeaData, Talker};
use crate::macros::readonly_struct;
use crate::rules::*;

/// Represents a single satellite's data in a GSV sentence.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Satellite {
    /// Satellite ID, typically a number from 1 to 32.
    svid: Option<u16>,
    /// Elevation in degrees.
    elv: Option<u8>,
    /// Azimuth in degrees.
    az: Option<u16>,
    /// Signal-to-noise ratio.
    cno: Option<u8>,
}

impl fmt::Debug for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("Satellite");
        if let Some(ref id) = self.svid {
            ds.field("id", id);
        }
        if let Some(elevation_degrees) = self.elv {
            ds.field("elevation_degrees", &elevation_degrees);
        }
        if let Some(azimuth_degree) = self.az {
            ds.field("azimuth_degree", &azimuth_degree);
        }
        if let Some(snr) = self.cno {
            ds.field("snr", &snr);
        }
        ds.finish()
    }
}

readonly_struct!(
    Gsv,
    "GNSS satellites in view",
    {talker: Talker},
    {
        satellites: Vec<Satellite>,
        "Satellite data"
    },
    {
        signal_id:Option<u16>
    }
);

impl INmeaData for Gsv {
    fn new(ctx: &mut StrParserContext, talker: Talker) -> mischief::Result<Self> {
        clerk::trace!("Gsv::new: sentence='{}'", ctx.full_str());
        // Validate each line with NMEA_VALIDATE
        for l in ctx.full_str().lines() {
            NMEA_VALIDATE.apply(l)?;
        }

        // Count the number of lines and satellites
        let line_count = ctx.full_str().lines().count();
        clerk::trace!("Gsv::new: line_count={line_count}");

        // The first line contains the talker, number of lines, and number of satellites
        let satellite_count = ctx
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .skip_strict(&UNTIL_COMMA_DISCARD)?
            .take(&UNTIL_COMMA_DISCARD)
            .parse_opt::<usize>()
            .expect("Cannot get the count of satellites.");
        clerk::trace!("Gsv::new: satellite_count={satellite_count}");

        // The last line may have fewer than 4 satellites, so we calculate how many
        // satellites are in the last line based on the total count.
        let last_line_satellite_count = {
            let rem = satellite_count % 4;
            if rem == 0 && line_count == 1 && satellite_count != 0 {
                4
            } else {
                rem
            }
        };
        clerk::trace!("Gsv::new: last_line_satellite_count={last_line_satellite_count}");

        let mut satellites = Vec::with_capacity(satellite_count);
        // Parse all but the last line (each has 4 satellites)
        for _ in 0..line_count - 1 {
            for _ in 0..3 {
                satellites.push(Self::parse_satellite(ctx, false)?);
            }
            satellites.push(Self::parse_satellite(ctx, true)?);
            // Skip any extra fields after the 4th satellite in the line
            ctx.skip(&UNTIL_COMMA_DISCARD)
                .skip(&UNTIL_COMMA_DISCARD)
                .skip(&UNTIL_COMMA_DISCARD)
                .skip(&UNTIL_COMMA_DISCARD);
        }

        // Parse the last line (may have fewer than 4 satellites)
        if last_line_satellite_count != 0 {
            for _ in 0..(last_line_satellite_count - 1) {
                satellites.push(Self::parse_satellite(ctx, false)?);
            }
            satellites.push(Self::parse_satellite(ctx, true)?);
        }
        let signal_id = ctx.take(&UNTIL_COMMA_OR_STAR_DISCARD).parse_opt();

        Ok(Self {
            talker,
            satellites,
            signal_id,
        })
    }
}
impl Gsv {
    /// Helper to parse a single satellite entry.
    /// If `last` is true, the SNR field is terminated by a star.
    fn parse_satellite(ctx: &mut StrParserContext, last: bool) -> mischief::Result<Satellite> {
        let id = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let elevation_degrees = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let azimuth_degree = ctx.take(&UNTIL_COMMA_DISCARD).parse_opt();
        let snr = if last {
            ctx.take(&UNTIL_STAR_DISCARD).parse_opt()
        } else {
            ctx.take(&UNTIL_COMMA_DISCARD).parse_opt()
        };
        Ok(Satellite {
            svid: id,
            elv: elevation_degrees,
            az: azimuth_degree,
            cno: snr,
        })
    }
}
impl fmt::Debug for Gsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("GSV");
        ds.field("talker", &self.talker);
        ds.field("count", &self.satellites.len());
        ds.field("satellites", &self.satellites);
        ds.finish()
    }
}

#[cfg(test)]
mod test {
    use clerk::{LogLevel, init_log_with_level};

    use super::*;

    #[test]
    fn test_new_gsv() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGSV,3,1,10,25,68,053,47,21,59,306,49,29,56,161,49,31,36,265,49*79\r\n$GPGSV,3,2,10,12,29,048,49,05,22,123,49,18,13,000,49,01,00,000,49*72\r\n$GPGSV,3,3,10,14,00,000,03,16,00,000,27*7C";
        let mut ctx = StrParserContext::new();
        let gsv = Gsv::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{gsv:?}");
        assert_eq!(gsv.talker, Talker::GP);
        assert_eq!(gsv.satellites.len(), 10);
        // line 1
        assert_eq!(gsv.satellites[0].svid, Some(25));
        assert_eq!(gsv.satellites[0].elv, Some(68));
        assert_eq!(gsv.satellites[0].az, Some(53));
        assert_eq!(gsv.satellites[0].cno, Some(47));
        assert_eq!(gsv.satellites[1].svid, Some(21));
        assert_eq!(gsv.satellites[1].elv, Some(59));
        assert_eq!(gsv.satellites[1].az, Some(306));
        assert_eq!(gsv.satellites[1].cno, Some(49));
        assert_eq!(gsv.satellites[2].svid, Some(29));
        assert_eq!(gsv.satellites[2].elv, Some(56));
        assert_eq!(gsv.satellites[2].az, Some(161));
        assert_eq!(gsv.satellites[2].cno, Some(49));
        assert_eq!(gsv.satellites[3].svid, Some(31));
        assert_eq!(gsv.satellites[3].elv, Some(36));
        assert_eq!(gsv.satellites[3].az, Some(265));
        assert_eq!(gsv.satellites[3].cno, Some(49));
        // line 2
        assert_eq!(gsv.satellites[4].svid, Some(12));
        assert_eq!(gsv.satellites[4].elv, Some(29));
        assert_eq!(gsv.satellites[4].az, Some(48));
        assert_eq!(gsv.satellites[4].cno, Some(49));
        assert_eq!(gsv.satellites[5].svid, Some(5));
        assert_eq!(gsv.satellites[5].elv, Some(22));
        assert_eq!(gsv.satellites[5].az, Some(123));
        assert_eq!(gsv.satellites[5].cno, Some(49));
        assert_eq!(gsv.satellites[6].svid, Some(18));
        assert_eq!(gsv.satellites[6].elv, Some(13));
        assert_eq!(gsv.satellites[6].az, Some(0));
        assert_eq!(gsv.satellites[6].cno, Some(49));
        assert_eq!(gsv.satellites[7].svid, Some(1));
        assert_eq!(gsv.satellites[7].elv, Some(0));
        assert_eq!(gsv.satellites[7].az, Some(0));
        assert_eq!(gsv.satellites[7].cno, Some(49));
        // line 3
        assert_eq!(gsv.satellites[8].svid, Some(14));
        assert_eq!(gsv.satellites[8].elv, Some(0));
        assert_eq!(gsv.satellites[8].az, Some(0));
        assert_eq!(gsv.satellites[8].cno, Some(3));
        assert_eq!(gsv.satellites[9].svid, Some(16));
        assert_eq!(gsv.satellites[9].elv, Some(0));
        assert_eq!(gsv.satellites[9].az, Some(0));
        assert_eq!(gsv.satellites[9].cno, Some(27));
        Ok(())
    }

    #[test]
    fn test_new_gsv_4() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGSV,1,1,4,02,35,291,,03,09,129,,05,14,305,,06,38,226,*4E";
        let mut ctx = StrParserContext::new();
        let gsv = Gsv::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{gsv:?}");
        assert_eq!(gsv.talker, Talker::GP);
        assert_eq!(gsv.satellites.len(), 4);
        assert_eq!(gsv.satellites[0].svid, Some(2));
        assert_eq!(gsv.satellites[0].elv, Some(35));
        assert_eq!(gsv.satellites[0].az, Some(291));
        assert!(gsv.satellites[0].cno.is_none());
        assert_eq!(gsv.satellites[1].svid, Some(3));
        assert_eq!(gsv.satellites[1].elv, Some(9));
        assert_eq!(gsv.satellites[1].az, Some(129));
        assert!(gsv.satellites[1].cno.is_none());
        assert_eq!(gsv.satellites[2].svid, Some(5));
        assert_eq!(gsv.satellites[2].elv, Some(14));
        assert_eq!(gsv.satellites[2].az, Some(305));
        assert!(gsv.satellites[2].cno.is_none());
        assert_eq!(gsv.satellites[3].svid, Some(6));
        assert_eq!(gsv.satellites[3].elv, Some(38));
        assert_eq!(gsv.satellites[3].az, Some(226));
        assert!(gsv.satellites[3].cno.is_none());
        Ok(())
    }

    #[test]
    fn test_new_gsv_3() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGSV,1,1,3,02,35,291,,03,09,129,,05,14,305,*72";
        let mut ctx = StrParserContext::new();
        let gsv = Gsv::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{gsv:?}");
        assert_eq!(gsv.talker, Talker::GP);
        assert_eq!(gsv.satellites.len(), 3);
        assert_eq!(gsv.satellites[0].svid, Some(2));
        assert_eq!(gsv.satellites[0].elv, Some(35));
        assert_eq!(gsv.satellites[0].az, Some(291));
        assert!(gsv.satellites[0].cno.is_none());
        assert_eq!(gsv.satellites[1].svid, Some(3));
        assert_eq!(gsv.satellites[1].elv, Some(9));
        assert_eq!(gsv.satellites[1].az, Some(129));
        assert!(gsv.satellites[1].cno.is_none());
        assert_eq!(gsv.satellites[2].svid, Some(5));
        assert_eq!(gsv.satellites[2].elv, Some(14));
        assert_eq!(gsv.satellites[2].az, Some(305));
        assert!(gsv.satellites[2].cno.is_none());
        Ok(())
    }
    #[test]
    fn test_new_gsv_0() -> mischief::Result<()> {
        init_log_with_level(LogLevel::TRACE);
        let s = "$GPGSV,1,1,0,*65";
        let mut ctx = StrParserContext::new();
        let gsv = Gsv::new(ctx.init(s.to_string()), Talker::GP)?;
        println!("{gsv:?}");
        assert_eq!(gsv.talker, Talker::GP);
        assert_eq!(gsv.satellites.len(), 0);
        Ok(())
    }
}
