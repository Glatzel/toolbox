#![no_main]

use libfuzzer_sys::fuzz_target;
use rax::string::{Decoder, IDecode, IGlobalRule, IStrFlowRule};

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

        let mut decoder = Decoder::new(s);
        let _ = rax_nmea::sentence::Dhv::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Dhv::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Dhv::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Dtm::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Gbq::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Gbs::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Gga::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Gll::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Glq::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Gnq::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Gns::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Gpq::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Grs::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Gsa::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Gst::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Gsv::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Rmc::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Ths::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Txt::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Vlw::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Vtg::decode(&mut decoder);
        decoder.reset();
        let _ = rax_nmea::sentence::Zda::decode(&mut decoder);
    }
});
