use derive_getters::Getters;
use rax::string::{DecodeOptExt, Decoder, IDecode};

use crate::RaxNmeaError;
use crate::rules::*;
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
        let time = parser.skip_strict(&UNTIL_COMMA_DISCARD)?.take(&NmeaTime);
        let speed3d = parser.take(&UNTIL_COMMA_DISCARD).decode_opt();
        let speed_x = parser.take(&UNTIL_COMMA_DISCARD).decode_opt();
        let speed_y = parser.take(&UNTIL_COMMA_DISCARD).decode_opt();
        let speed_z = parser.take(&UNTIL_COMMA_DISCARD).decode_opt();
        let gdspd = parser.take(&UNTIL_STAR_DISCARD).decode_opt();

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
    use std::string::ToString;

    use clerk::{LevelFilter, init_log_with_level};

    use super::*;
    #[test]
    fn test_new_dhv() -> mischief::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let s = "$GNDHV,021150.000,0.03,0.006,-0.042,-0.026,0.06*65";
        let mut parser = Decoder::new();
        let dhv = Dhv::decode(parser.init(s.to_string()))?;
        println!("{dhv:?}");
        insta::assert_json_snapshot!(dhv);
        Ok(())
    }
}
