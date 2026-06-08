use std::fs::File;
use std::io::{BufRead, BufReader};

use clerk::LevelFilter;
use rax::string::Decoder;
use rax_nmea::data::*;
use rax_nmea::rules::{
    NmeaGsvLineCount, NmeaIdentifier, NmeaTalker, NmeaTxtLineCount, NmeaValidate,
};
use rstest::rstest;
#[derive(Debug)]
pub enum Dispatcher {
    DHV(Talker, Dhv),
    DTM(Talker, Dtm),
    GBQ(Talker, Gbq),
    GBS(Talker, Gbs),
    GGA(Talker, Gga),
    GLL(Talker, Gll),
    GLQ(Talker, Glq),
    GNQ(Talker, Gnq),
    GNS(Talker, Gns),
    GPQ(Talker, Gpq),
    GRS(Talker, Grs),
    GSA(Talker, Gsa),
    GST(Talker, Gst),
    GSV(Talker, Gsv),
    RMC(Talker, Rmc),
    THS(Talker, Ths),
    TXT(Talker, Txt),
    VLW(Talker, Vlw),
    VTG(Talker, Vtg),
    ZDA(Talker, Zda),
}

fn wrapper(f: &str) -> mischief::Result<Vec<Dispatcher>> {
    let mut reader = BufReader::new(File::open(f)?);
    let mut decoder = Decoder::new();
    let mut buf = String::new();
    let mut collector = Vec::<Dispatcher>::new();
    while reader.read_line(&mut buf).is_ok() {
        if buf.is_empty() {
            return Ok(collector);
        }
        decoder.init(buf.clone());
        decoder.global(&NmeaValidate)?;
        let identifier = decoder.global(&NmeaIdentifier)?;
        let talker = decoder.global(&NmeaTalker)?;
        match identifier {
            Identifier::DHV => collector.push(Dispatcher::DHV(talker, decoder.decode()?)),
            Identifier::DTM => collector.push(Dispatcher::DTM(talker, decoder.decode()?)),
            Identifier::GBQ => collector.push(Dispatcher::GBQ(talker, decoder.decode()?)),
            Identifier::GBS => collector.push(Dispatcher::GBS(talker, decoder.decode()?)),
            Identifier::GGA => collector.push(Dispatcher::GGA(talker, decoder.decode()?)),
            Identifier::GLL => collector.push(Dispatcher::GLL(talker, decoder.decode()?)),
            Identifier::GLQ => collector.push(Dispatcher::GLQ(talker, decoder.decode()?)),
            Identifier::GNQ => collector.push(Dispatcher::GNQ(talker, decoder.decode()?)),
            Identifier::GNS => collector.push(Dispatcher::GNS(talker, decoder.decode()?)),
            Identifier::GPQ => collector.push(Dispatcher::GPQ(talker, decoder.decode()?)),
            Identifier::GRS => collector.push(Dispatcher::GRS(talker, decoder.decode()?)),
            Identifier::GSA => collector.push(Dispatcher::GSA(talker, decoder.decode()?)),
            Identifier::GST => collector.push(Dispatcher::GST(talker, decoder.decode()?)),
            Identifier::GSV => {
                let count = decoder.global(&NmeaGsvLineCount)?;
                for _ in 0..count - 1 {
                    reader.read_line(&mut buf)?;
                }
                decoder.init(buf.clone());
                let result: Gsv = decoder.decode()?;
                if result.satellites().len() < count as usize * 4 - 3
                    || result.satellites().len() > count as usize * 4
                {
                    mischief::bail!(
                        "message length mismatch: {:?}, expected {} to {}, got {}:\n{}",
                        result,
                        count * 4 - 3,
                        count * 4,
                        result.satellites().len(),
                        buf
                    )
                }
                collector.push(Dispatcher::GSV(talker, result));
            }
            Identifier::RMC => collector.push(Dispatcher::RMC(talker, decoder.decode()?)),
            Identifier::THS => collector.push(Dispatcher::THS(talker, decoder.decode()?)),
            Identifier::TXT => {
                let count = decoder.global(&NmeaTxtLineCount)?;
                for _ in 0..count - 1 {
                    reader.read_line(&mut buf)?;
                }
                decoder.init(buf.clone());
                let result: Txt = decoder.decode()?;
                if result.message().len() != count as usize {
                    mischief::bail!(
                        "message length mismatch: expected {}, got {}: {}",
                        count,
                        result.message().len(),
                        buf
                    )
                }
                collector.push(Dispatcher::TXT(talker, result));
            }
            Identifier::VLW => collector.push(Dispatcher::VLW(talker, decoder.decode()?)),
            Identifier::VTG => collector.push(Dispatcher::VTG(talker, decoder.decode()?)),
            Identifier::ZDA => collector.push(Dispatcher::ZDA(talker, decoder.decode()?)),
        }
        buf.clear();
    }
    Ok(collector)
}

fn main() -> mischief::Result<()> {
    clerk::init_log_with_level(LevelFilter::WARN);
    wrapper("data/nmea1.log")?;
    Ok(())
}
#[rstest]
#[case("data/nmea1.log")]
#[case("data/nmea2.log")]
#[case("data/nmea_with_sat_info.log")]
fn test(#[case] file: &str) -> mischief::Result<()> {
    use insta::assert_debug_snapshot;
    clerk::init_log_with_level(LevelFilter::WARN);
    let result = wrapper(file)?;
    Ok(())
}
