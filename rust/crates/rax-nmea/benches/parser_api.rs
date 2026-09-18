use std::io::{BufRead, BufReader};

use criterion::{criterion_group, criterion_main, Criterion};
use rax::text::{IParseStr, StrParser};
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

fn reuse_parser(f: &str) -> mischief::Result<Vec<Dispatcher>> {
    parse_file(f, |identifier, talker, buf| {
        let mut parser = StrParser::new(buf);

        Ok(match identifier {
            Identifier::DHV => Dispatcher::DHV(talker, parser.parse()?),
            Identifier::DTM => Dispatcher::DTM(talker, parser.parse()?),
            Identifier::GBQ => Dispatcher::GBQ(talker, parser.parse()?),
            Identifier::GBS => Dispatcher::GBS(talker, parser.parse()?),
            Identifier::GGA => Dispatcher::GGA(talker, parser.parse()?),
            Identifier::GLL => Dispatcher::GLL(talker, parser.parse()?),
            Identifier::GLQ => Dispatcher::GLQ(talker, parser.parse()?),
            Identifier::GNQ => Dispatcher::GNQ(talker, parser.parse()?),
            Identifier::GNS => Dispatcher::GNS(talker, parser.parse()?),
            Identifier::GPQ => Dispatcher::GPQ(talker, parser.parse()?),
            Identifier::GRS => Dispatcher::GRS(talker, parser.parse()?),
            Identifier::GSA => Dispatcher::GSA(talker, parser.parse()?),
            Identifier::GST => Dispatcher::GST(talker, parser.parse()?),
            Identifier::GSV => Dispatcher::GSV(talker, parser.parse()?),
            Identifier::RMC => Dispatcher::RMC(talker, parser.parse()?),
            Identifier::THS => Dispatcher::THS(talker, parser.parse()?),
            Identifier::TXT => Dispatcher::TXT(talker, parser.parse()?),
            Identifier::VLW => Dispatcher::VLW(talker, parser.parse()?),
            Identifier::VTG => Dispatcher::VTG(talker, parser.parse()?),
            Identifier::ZDA => Dispatcher::ZDA(talker, parser.parse()?),
        })
    })
}

fn use_trait_parser(f: &str) -> mischief::Result<Vec<Dispatcher>> {
    parse_file(f, |identifier, talker, buf| {
        Ok(match identifier {
            Identifier::DHV => Dispatcher::DHV(talker, Dhv::parse_str(buf)?),
            Identifier::DTM => Dispatcher::DTM(talker, Dtm::parse_str(buf)?),
            Identifier::GBQ => Dispatcher::GBQ(talker, Gbq::parse_str(buf)?),
            Identifier::GBS => Dispatcher::GBS(talker, Gbs::parse_str(buf)?),
            Identifier::GGA => Dispatcher::GGA(talker, Gga::parse_str(buf)?),
            Identifier::GLL => Dispatcher::GLL(talker, Gll::parse_str(buf)?),
            Identifier::GLQ => Dispatcher::GLQ(talker, Glq::parse_str(buf)?),
            Identifier::GNQ => Dispatcher::GNQ(talker, Gnq::parse_str(buf)?),
            Identifier::GNS => Dispatcher::GNS(talker, Gns::parse_str(buf)?),
            Identifier::GPQ => Dispatcher::GPQ(talker, Gpq::parse_str(buf)?),
            Identifier::GRS => Dispatcher::GRS(talker, Grs::parse_str(buf)?),
            Identifier::GSA => Dispatcher::GSA(talker, Gsa::parse_str(buf)?),
            Identifier::GST => Dispatcher::GST(talker, Gst::parse_str(buf)?),
            Identifier::GSV => Dispatcher::GSV(talker, Gsv::parse_str(buf)?),
            Identifier::RMC => Dispatcher::RMC(talker, Rmc::parse_str(buf)?),
            Identifier::THS => Dispatcher::THS(talker, Ths::parse_str(buf)?),
            Identifier::TXT => Dispatcher::TXT(talker, Txt::parse_str(buf)?),
            Identifier::VLW => Dispatcher::VLW(talker, Vlw::parse_str(buf)?),
            Identifier::VTG => Dispatcher::VTG(talker, Vtg::parse_str(buf)?),
            Identifier::ZDA => Dispatcher::ZDA(talker, Zda::parse_str(buf)?),
        })
    })
}

const FILES: &[&str] = &[
    include_str!("../external/nmea/tests/data/nmea1.log"),
    include_str!("../external/nmea/tests/data/nmea2.log"),
    include_str!("../external/nmea/tests/data/nmea_with_sat_info.log"),
];

fn bench_reuse_parser(c: &mut Criterion) {
    c.bench_function("reuse_parser", |b| {
        b.iter(|| {
            for text in FILES {
                reuse_parser(text).unwrap();
            }
        })
    });
}

fn bench_use_trait_parser(c: &mut Criterion) {
    c.bench_function("use_trait_parser", |b| {
        b.iter(|| {
            for text in FILES {
                use_trait_parser(text).unwrap();
            }
        })
    });
}

criterion_group!(
    benches_group,
    bench_reuse_parser,
    bench_use_trait_parser
);

criterion_main!(benches_group);