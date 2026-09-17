use std::fs::File;
use std::io::{BufRead, BufReader};

use criterion::{Criterion, criterion_group, criterion_main};
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

fn reuse_parser(f: &str) -> mischief::Result<Vec<Dispatcher>> {
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
fn use_trait_parser(f: &str) -> mischief::Result<Vec<Dispatcher>> {
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
            Identifier::DHV => collector.push(Dispatcher::DHV(talker, Dhv::parse_str(&buf)?)),
            Identifier::DTM => collector.push(Dispatcher::DTM(talker, Dtm::parse_str(&buf)?)),
            Identifier::GBQ => collector.push(Dispatcher::GBQ(talker, Gbq::parse_str(&buf)?)),
            Identifier::GBS => collector.push(Dispatcher::GBS(talker, Gbs::parse_str(&buf)?)),
            Identifier::GGA => collector.push(Dispatcher::GGA(talker, Gga::parse_str(&buf)?)),
            Identifier::GLL => collector.push(Dispatcher::GLL(talker, Gll::parse_str(&buf)?)),
            Identifier::GLQ => collector.push(Dispatcher::GLQ(talker, Glq::parse_str(&buf)?)),
            Identifier::GNQ => collector.push(Dispatcher::GNQ(talker, Gnq::parse_str(&buf)?)),
            Identifier::GNS => collector.push(Dispatcher::GNS(talker, Gns::parse_str(&buf)?)),
            Identifier::GPQ => collector.push(Dispatcher::GPQ(talker, Gpq::parse_str(&buf)?)),
            Identifier::GRS => collector.push(Dispatcher::GRS(talker, Grs::parse_str(&buf)?)),
            Identifier::GSA => collector.push(Dispatcher::GSA(talker, Gsa::parse_str(&buf)?)),
            Identifier::GST => collector.push(Dispatcher::GST(talker, Gst::parse_str(&buf)?)),
            Identifier::GSV => collector.push(Dispatcher::GSV(talker, Gsv::parse_str(&buf)?)),
            Identifier::RMC => collector.push(Dispatcher::RMC(talker, Rmc::parse_str(&buf)?)),
            Identifier::THS => collector.push(Dispatcher::THS(talker, Ths::parse_str(&buf)?)),
            Identifier::TXT => collector.push(Dispatcher::TXT(talker, Txt::parse_str(&buf)?)),
            Identifier::VLW => collector.push(Dispatcher::VLW(talker, Vlw::parse_str(&buf)?)),
            Identifier::VTG => collector.push(Dispatcher::VTG(talker, Vtg::parse_str(&buf)?)),
            Identifier::ZDA => collector.push(Dispatcher::ZDA(talker, Zda::parse_str(&buf)?)),
        }
        buf.clear();
    }
    Ok(collector)
}
const FILES: &[&str] = &[
    include_str!("../external/nmea/tests/data/nmea1.log"),
    include_str!("../external/nmea/tests/data/nmea2.log"),
    include_str!("../external/nmea/tests/data/nmea_with_sat_info.log"),
];
fn bench_reuse_parser(c: &mut Criterion) {
    c.bench_function("validate", |b| {
        b.iter(|| {
            for text in FILES {
                reuse_parser(text).unwrap();
            }
        })
    });
}
fn bench_use_trait_parser(c: &mut Criterion) {
    c.bench_function("validate", |b| {
        b.iter(|| {
            for text in FILES {
                use_trait_parser(text).unwrap();
            }
        })
    });
}
criterion_group!(benches_group, bench_reuse_parser,bench_use_trait_parser);
criterion_main!(benches_group);
