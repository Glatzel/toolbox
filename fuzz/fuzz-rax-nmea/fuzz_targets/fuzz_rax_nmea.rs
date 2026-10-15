#![no_main]

use std::str::FromStr;

use libfuzzer_sys::fuzz_target;
use rax::text::{IFlowRule, IGlobalRule};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = rax_nmea::rules::NmeaCoord.apply(s);
        let _ = rax_nmea::rules::NmeaDate.apply(s);
        let _ = rax_nmea::rules::NmeaDegree.apply(s);
        let _ = rax_nmea::rules::NmeaGsvLineCount.apply(s);
        let _ = rax_nmea::rules::NmeaIdentifier.apply(s);
        let _ = rax_nmea::rules::NmeaTalker.apply(s);
        let _ = rax_nmea::rules::NmeaTime.apply(s);
        let _ = rax_nmea::rules::NmeaTxtLineCount.apply(s);
        let _ = rax_nmea::rules::NmeaValidate.apply(s);
        let _ = rax_nmea::rules::NmeaValidateMultiLine.apply(s);

        let _ = rax_nmea::sentence::Dhv::from_str(s);
        let _ = rax_nmea::sentence::Dhv::from_str(s);
        let _ = rax_nmea::sentence::Dhv::from_str(s);
        let _ = rax_nmea::sentence::Dtm::from_str(s);
        let _ = rax_nmea::sentence::Gbq::from_str(s);
        let _ = rax_nmea::sentence::Gbs::from_str(s);
        let _ = rax_nmea::sentence::Gga::from_str(s);
        let _ = rax_nmea::sentence::Gll::from_str(s);
        let _ = rax_nmea::sentence::Glq::from_str(s);
        let _ = rax_nmea::sentence::Gnq::from_str(s);
        let _ = rax_nmea::sentence::Gns::from_str(s);
        let _ = rax_nmea::sentence::Gpq::from_str(s);
        let _ = rax_nmea::sentence::Grs::from_str(s);
        let _ = rax_nmea::sentence::Gsa::from_str(s);
        let _ = rax_nmea::sentence::Gst::from_str(s);
        let _ = rax_nmea::sentence::Gsv::from_str(s);
        let _ = rax_nmea::sentence::Rmc::from_str(s);
        let _ = rax_nmea::sentence::Ths::from_str(s);
        let _ = rax_nmea::sentence::Txt::from_str(s);
        let _ = rax_nmea::sentence::Vlw::from_str(s);
        let _ = rax_nmea::sentence::Vtg::from_str(s);
        let _ = rax_nmea::sentence::Zda::from_str(s);
    }
});
