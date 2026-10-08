use std::fs::File;
use std::io::{BufRead, BufReader};

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

        let identifier = StrParser::global(&buf, &NmeaIdentifier)?;
        let talker = StrParser::global(&buf, &NmeaTalker)?;
        // For multi-line sentences, accumulate all lines into buf first
        match identifier {
            Identifier::GSV => {
                let count = StrParser::global(&buf, &NmeaGsvLineCount)?;
                for _ in 0..count - 1 {
                    reader.read_line(&mut buf)?; // buf borrow is free here
                }
            }
            Identifier::TXT => {
                let count = StrParser::global(&buf, &NmeaTxtLineCount)?;
                for _ in 0..count - 1 {
                    reader.read_line(&mut buf)?;
                }
            }
            _ => {}
        }

        StrParser::global(&buf, &NmeaValidateMultiLine)?;

        match identifier {
            Identifier::DHV => collector.push(Dispatcher::DHV(talker, (&buf).parse()?)),
            Identifier::DTM => collector.push(Dispatcher::DTM(talker, (&buf).parse()?)),
            Identifier::GBQ => collector.push(Dispatcher::GBQ(talker, (&buf).parse()?)),
            Identifier::GBS => collector.push(Dispatcher::GBS(talker, (&buf).parse()?)),
            Identifier::GGA => collector.push(Dispatcher::GGA(talker, (&buf).parse()?)),
            Identifier::GLL => collector.push(Dispatcher::GLL(talker, (&buf).parse()?)),
            Identifier::GLQ => collector.push(Dispatcher::GLQ(talker, (&buf).parse()?)),
            Identifier::GNQ => collector.push(Dispatcher::GNQ(talker, (&buf).parse()?)),
            Identifier::GNS => collector.push(Dispatcher::GNS(talker, (&buf).parse()?)),
            Identifier::GPQ => collector.push(Dispatcher::GPQ(talker, (&buf).parse()?)),
            Identifier::GRS => collector.push(Dispatcher::GRS(talker, (&buf).parse()?)),
            Identifier::GSA => collector.push(Dispatcher::GSA(talker, (&buf).parse()?)),
            Identifier::GST => collector.push(Dispatcher::GST(talker, (&buf).parse()?)),
            Identifier::GSV => collector.push(Dispatcher::GSV(talker, (&buf).parse()?)),
            Identifier::RMC => collector.push(Dispatcher::RMC(talker, (&buf).parse()?)),
            Identifier::THS => collector.push(Dispatcher::THS(talker, (&buf).parse()?)),
            Identifier::TXT => collector.push(Dispatcher::TXT(talker, (&buf).parse()?)),
            Identifier::VLW => collector.push(Dispatcher::VLW(talker, (&buf).parse()?)),
            Identifier::VTG => collector.push(Dispatcher::VTG(talker, (&buf).parse()?)),
            Identifier::ZDA => collector.push(Dispatcher::ZDA(talker, (&buf).parse()?)),
        }
        buf.clear();
    }
    Ok(collector)
}

fn main() -> mischief::Result<()> {
    wrapper("data/nmea1.log")?;
    Ok(())
}
#[rstest]
#[case("external/nmea/tests/data/nmea1.log")]
#[case("external/nmea/tests/data/nmea2.log")]
#[case("external/nmea/tests/data/nmea_with_sat_info.log")]
fn test(#[case] file: &str) -> mischief::Result<()> {
    let _ = wrapper(file)?;
    Ok(())
}
