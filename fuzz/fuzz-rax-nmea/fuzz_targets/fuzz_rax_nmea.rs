#![no_main]

use libfuzzer_sys::fuzz_target;
use rax::text::{IFlowRule, IGlobalRule, IParseStr};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) && s.is_ascii(){
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

        let _ = rax_nmea::sentence::Dhv::parse_str(s);
        let _ = rax_nmea::sentence::Dhv::parse_str(s);
        let _ = rax_nmea::sentence::Dhv::parse_str(s);
        let _ = rax_nmea::sentence::Dtm::parse_str(s);
        let _ = rax_nmea::sentence::Gbq::parse_str(s);
        let _ = rax_nmea::sentence::Gbs::parse_str(s);
        let _ = rax_nmea::sentence::Gga::parse_str(s);
        let _ = rax_nmea::sentence::Gll::parse_str(s);
        let _ = rax_nmea::sentence::Glq::parse_str(s);
        let _ = rax_nmea::sentence::Gnq::parse_str(s);
        let _ = rax_nmea::sentence::Gns::parse_str(s);
        let _ = rax_nmea::sentence::Gpq::parse_str(s);
        let _ = rax_nmea::sentence::Grs::parse_str(s);
        let _ = rax_nmea::sentence::Gsa::parse_str(s);
        let _ = rax_nmea::sentence::Gst::parse_str(s);
        let _ = rax_nmea::sentence::Gsv::parse_str(s);
        let _ = rax_nmea::sentence::Rmc::parse_str(s);
        let _ = rax_nmea::sentence::Ths::parse_str(s);
        let _ = rax_nmea::sentence::Txt::parse_str(s);
        let _ = rax_nmea::sentence::Vlw::parse_str(s);
        let _ = rax_nmea::sentence::Vtg::parse_str(s);
        let _ = rax_nmea::sentence::Zda::parse_str(s);
    }
});
