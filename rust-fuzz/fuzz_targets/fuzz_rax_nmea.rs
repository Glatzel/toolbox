#![no_main]

use libfuzzer_sys::fuzz_target;
use rax::string::{IGlobalRule, IStrFlowRule};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = rax_nmea::rules::NmeaCoord.apply(s);
        let _ = rax_nmea::rules::NmeaDate.apply(s);
        let _ = rax_nmea::rules::NmeaDegree.apply(s);
        let _ = rax_nmea::rules::NmeaGsvLineCount.apply(s);
        let _ = rax_nmea::rules::NmeaIdentifier.apply(s);
        let _ = rax_nmea::rules::NmeaValidate.apply(s);
        let _ = rax_nmea::rules::NmeaValidateMultiLine.apply(s);
    }
});
