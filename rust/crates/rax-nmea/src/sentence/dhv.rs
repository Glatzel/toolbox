use derive_getters::Getters;
use rax::string::{Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;
use crate::utils::ParseOptionPrimitive;
/// Dhv - Velocity in 3 dimensions
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Getters)]
pub struct Dhv {
    /// UTC time of the DHV fix associated with this sentence.
    time: Option<chrono::NaiveTime>,

    /// 3D speed (meters/second)
    speed3d: Option<f64>,

    /// Speed in X direction (meters/second)
    speed_x: Option<f64>,

    /// Speed in Y direction (meters/second)
    speed_y: Option<f64>,

    /// Speed in Z direction (meters/second)
    speed_z: Option<f64>,

    /// Ground speed (meters/second)
    gdspd: Option<f64>,
}
impl IDecode<RaxNmeaError> for Dhv {
    fn decode(parser: &mut Decoder) -> Result<Self, RaxNmeaError> {
        let time = parser.skip(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime)?;
        let speed3d = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let speed_x = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let speed_y = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let speed_z = parser.take(&UNTIL_COMMA_DISCARD)?.parse_option()?;
        let gdspd = parser.take(&UNTIL_STAR_DISCARD)?.parse_option()?;

        Ok(Dhv {
            time,
            speed3d,
            speed_x,
            speed_y,
            speed_z,
            gdspd,
        })
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[rstest::rstest]
    #[case("1", "$GNDHV,021150.000,0.03,0.006,-0.042,-0.026,0.06*65")]
    fn test_dhv(#[case] index: &str, #[case] input: &str) -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let mut decoder = Decoder::new(input);
        let dhv = Dhv::decode(&mut decoder)?;
        println!("{dhv:?}");
        insta::assert_json_snapshot!(index, dhv);
        Ok(())
    }
}
