use std::fs::File;
use std::io::{BufRead, BufReader};

use clerk::LevelFilter;
use rax::text::StrParser;
use rax_nmea::common::*;
use rax_nmea::rules::*;
use rax_nmea::sentence::*;
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
    let mut buf = String::new();
    let mut collector = Vec::<Dispatcher>::new();
    while reader.read_line(&mut buf).is_ok() {
        if buf.is_empty() {
            return Ok(collector);
        }

        let mut probe = StrParser::new(&buf);
        let identifier = probe.global(&NmeaIdentifier)?;
        let talker = probe.global(&NmeaTalker)?;
        // For multi-line sentences, accumulate all lines into buf first
        match identifier {
            Identifier::GSV => {
                let count = probe.global(&NmeaGsvLineCount)?;
                for _ in 0..count - 1 {
                    reader.read_line(&mut buf)?; // buf borrow is free here
                }
            }
            Identifier::TXT => {
                let count = probe.global(&NmeaTxtLineCount)?;
                for _ in 0..count - 1 {
                    reader.read_line(&mut buf)?;
                }
            }
            _ => {}
        }
        let mut parser = StrParser::new(&buf);
        parser.global(&NmeaValidateMultiLine)?;
        match identifier {
            Identifier::DHV => collector.push(Dispatcher::DHV(talker, parser.parse()?)),
            Identifier::DTM => collector.push(Dispatcher::DTM(talker, parser.parse()?)),
            Identifier::GBQ => collector.push(Dispatcher::GBQ(talker, parser.parse()?)),
            Identifier::GBS => collector.push(Dispatcher::GBS(talker, parser.parse()?)),
            Identifier::GGA => collector.push(Dispatcher::GGA(talker, parser.parse()?)),
            Identifier::GLL => collector.push(Dispatcher::GLL(talker, parser.parse()?)),
            Identifier::GLQ => collector.push(Dispatcher::GLQ(talker, parser.parse()?)),
            Identifier::GNQ => collector.push(Dispatcher::GNQ(talker, parser.parse()?)),
            Identifier::GNS => collector.push(Dispatcher::GNS(talker, parser.parse()?)),
            Identifier::GPQ => collector.push(Dispatcher::GPQ(talker, parser.parse()?)),
            Identifier::GRS => collector.push(Dispatcher::GRS(talker, parser.parse()?)),
            Identifier::GSA => collector.push(Dispatcher::GSA(talker, parser.parse()?)),
            Identifier::GST => collector.push(Dispatcher::GST(talker, parser.parse()?)),
            Identifier::GSV => collector.push(Dispatcher::GSV(talker, parser.parse()?)),
            Identifier::RMC => collector.push(Dispatcher::RMC(talker, parser.parse()?)),
            Identifier::THS => collector.push(Dispatcher::THS(talker, parser.parse()?)),
            Identifier::TXT => collector.push(Dispatcher::TXT(talker, parser.parse()?)),
            Identifier::VLW => collector.push(Dispatcher::VLW(talker, parser.parse()?)),
            Identifier::VTG => collector.push(Dispatcher::VTG(talker, parser.parse()?)),
            Identifier::ZDA => collector.push(Dispatcher::ZDA(talker, parser.parse()?)),
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
#[case("external/nmea/tests/data/nmea1.log")]
#[case("external/nmea/tests/data/nmea2.log")]
#[case("external/nmea/tests/data/nmea_with_sat_info.log")]
fn test(#[case] file: &str) -> mischief::Result<()> {
    clerk::init_log_with_level(LevelFilter::WARN);
    let _ = wrapper(file)?;
    Ok(())
}
