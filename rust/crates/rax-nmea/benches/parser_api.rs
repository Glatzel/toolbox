use std::io::{BufRead, BufReader};
use std::str::FromStr;

use criterion::{Criterion, criterion_group, criterion_main};
use rax::text::StrParser;
use rax_nmea::common::*;
use rax_nmea::rules::*;
use rax_nmea::sentence::*;

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

fn parse_file(
    f: &str,
    mut parse: impl FnMut(Identifier, Talker, &str) -> mischief::Result<Dispatcher>,
) -> mischief::Result<Vec<Dispatcher>> {
    let mut reader = BufReader::new(f.as_bytes());
    let mut buf = String::new();
    let mut collector = Vec::new();

    while reader.read_line(&mut buf)? != 0 {
        let identifier = StrParser::global(&buf, &NmeaIdentifier)?;
        let talker = StrParser::global(&buf, &NmeaTalker)?;

        match identifier {
            Identifier::GSV => {
                let count = StrParser::global(&buf, &NmeaGsvLineCount)?;
                for _ in 1..count {
                    reader.read_line(&mut buf)?;
                }
            }
            Identifier::TXT => {
                let count = StrParser::global(&buf, &NmeaTxtLineCount)?;
                for _ in 1..count {
                    reader.read_line(&mut buf)?;
                }
            }
            _ => {}
        }

        StrParser::global(&buf, &NmeaValidateMultiLine)?;

        collector.push(parse(identifier, talker, &buf)?);

        buf.clear();
    }

    Ok(collector)
}

fn use_trait_parser(f: &str) -> mischief::Result<Vec<Dispatcher>> {
    parse_file(f, |identifier, talker, buf| {
        Ok(match identifier {
            Identifier::DHV => Dispatcher::DHV(talker, Dhv::from_str(buf)?),
            Identifier::DTM => Dispatcher::DTM(talker, Dtm::from_str(buf)?),
            Identifier::GBQ => Dispatcher::GBQ(talker, Gbq::from_str(buf)?),
            Identifier::GBS => Dispatcher::GBS(talker, Gbs::from_str(buf)?),
            Identifier::GGA => Dispatcher::GGA(talker, Gga::from_str(buf)?),
            Identifier::GLL => Dispatcher::GLL(talker, Gll::from_str(buf)?),
            Identifier::GLQ => Dispatcher::GLQ(talker, Glq::from_str(buf)?),
            Identifier::GNQ => Dispatcher::GNQ(talker, Gnq::from_str(buf)?),
            Identifier::GNS => Dispatcher::GNS(talker, Gns::from_str(buf)?),
            Identifier::GPQ => Dispatcher::GPQ(talker, Gpq::from_str(buf)?),
            Identifier::GRS => Dispatcher::GRS(talker, Grs::from_str(buf)?),
            Identifier::GSA => Dispatcher::GSA(talker, Gsa::from_str(buf)?),
            Identifier::GST => Dispatcher::GST(talker, Gst::from_str(buf)?),
            Identifier::GSV => Dispatcher::GSV(talker, Gsv::from_str(buf)?),
            Identifier::RMC => Dispatcher::RMC(talker, Rmc::from_str(buf)?),
            Identifier::THS => Dispatcher::THS(talker, Ths::from_str(buf)?),
            Identifier::TXT => Dispatcher::TXT(talker, Txt::from_str(buf)?),
            Identifier::VLW => Dispatcher::VLW(talker, Vlw::from_str(buf)?),
            Identifier::VTG => Dispatcher::VTG(talker, Vtg::from_str(buf)?),
            Identifier::ZDA => Dispatcher::ZDA(talker, Zda::from_str(buf)?),
        })
    })
}

const FILES: &[&str] = &[
    include_str!("../external/nmea/tests/data/nmea1.log"),
    include_str!("../external/nmea/tests/data/nmea2.log"),
    include_str!("../external/nmea/tests/data/nmea_with_sat_info.log"),
];

fn bench_use_trait_parser(c: &mut Criterion) {
    c.bench_function("use_trait_parser", |b| {
        b.iter(|| {
            for text in FILES {
                use_trait_parser(text).unwrap();
            }
        })
    });
}

criterion_group!(benches_group, bench_use_trait_parser);

criterion_main!(benches_group);
