extern crate std;
use std::fs::File;
use std::io;

use clerk::{LogLevel, init_log_with_level};
use rax::io::{IRaxReader, RaxReader};
use rax::str_parser::StrParserContext;
use rax_nmea::Dispatcher;
use rax_nmea::data::*;

#[test]
fn test_parse_nmea() -> mischief::Result<()> {
    init_log_with_level(LogLevel::WARN);
    for f in [
        "data/nmea1.log",
        "data/nmea2.log",
        "data/nmea_with_sat_info.log",
    ] {
        let mut reader = RaxReader::new(io::BufReader::new(File::open(f)?));
        let mut ctx = StrParserContext::new();
        let mut dispatcher = Dispatcher::new();
        while let Some((talker, identifier, sentence)) = reader
            .read_line()?
            .and_then(|line| dispatcher.dispatch(line))
        {
            match identifier {
                Identifier::DHV => {
                    let ctx = ctx.init(sentence);
                    let _ = Dhv::new(ctx, talker)?;
                }
                Identifier::GBS => {
                    let ctx = ctx.init(sentence);
                    let _ = Gbs::new(ctx, talker)?;
                }
                Identifier::GGA => {
                    let ctx = ctx.init(sentence);
                    let _ = Gga::new(ctx, talker)?;
                }
                Identifier::GLL => {
                    let ctx = ctx.init(sentence);
                    let _ = Gll::new(ctx, talker)?;
                }
                Identifier::GNS => {
                    let ctx = ctx.init(sentence);
                    let _ = Gns::new(ctx, talker)?;
                }
                Identifier::GRS => {
                    let ctx = ctx.init(sentence);
                    let _ = Grs::new(ctx, talker)?;
                }
                Identifier::GSA => {
                    let ctx = ctx.init(sentence);
                    let _ = Gsa::new(ctx, talker)?;
                }
                Identifier::GST => {
                    let ctx = ctx.init(sentence);
                    let _ = Gst::new(ctx, talker)?;
                }
                Identifier::GSV => {
                    let ctx = ctx.init(sentence);
                    let _ = Gsv::new(ctx, talker)?;
                }
                Identifier::RMC => {
                    let ctx = ctx.init(sentence);
                    let _ = Rmc::new(ctx, talker)?;
                }
                Identifier::TXT => {
                    let ctx = ctx.init(sentence);
                    let _ = Txt::new(ctx, talker)?;
                }
                Identifier::VTG => {
                    let ctx = ctx.init(sentence);
                    let _ = Vtg::new(ctx, talker)?;
                }
                Identifier::ZDA => {
                    let ctx = ctx.init(sentence);
                    let _ = Zda::new(ctx, talker)?;
                }
                Identifier::DTM => {
                    let ctx = ctx.init(sentence);
                    let _ = Dtm::new(ctx, talker)?;
                }
                Identifier::GBQ => {
                    let ctx = ctx.init(sentence);
                    let _ = Gbq::new(ctx, talker)?;
                }
                Identifier::GLQ => {
                    let ctx = ctx.init(sentence);
                    let _ = Glq::new(ctx, talker)?;
                }
                Identifier::GNQ => {
                    let ctx = ctx.init(sentence);
                    let _ = Gnq::new(ctx, talker)?;
                }
                Identifier::GPQ => {
                    let ctx = ctx.init(sentence);
                    let _ = Gpq::new(ctx, talker)?;
                }
                Identifier::THS => {
                    let ctx = ctx.init(sentence);
                    let _ = Ths::new(ctx, talker)?;
                }
                Identifier::VLW => {
                    let ctx = ctx.init(sentence);
                    let _ = Vlw::new(ctx, talker)?;
                }
            }
        }
    }

    Ok(())
}
