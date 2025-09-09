extern crate std;
fn main() -> mischief::Result<()> {
    use std::io::BufReader;
    use std::time::Duration;

    use clerk::LogLevel;
    use rax::io::IRaxReader;
    use rax::str_parser::StrParserContext;
    use rax_nmea::Dispatcher;
    use rax_nmea::data::*;
    clerk::init_log_with_level(LogLevel::WARN);
    let path = "COM5";
    let port = serialport::new(path, 9600)
        .timeout(Duration::from_millis(3000))
        .open()?;
    let mut reader = rax::io::RaxReader::new(BufReader::new(port));
    let mut ctx = StrParserContext::new();
    let mut dispatcher = Dispatcher::new();
    loop {
        if let Some((talker, identifier, sentence)) = reader
            .read_line()?
            .and_then(|line| dispatcher.dispatch(line))
        {
            match identifier {
                Identifier::DHV => {
                    let ctx = ctx.init(sentence);
                    let nmea = Dhv::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GBS => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gbs::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GGA => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gga::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GLL => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gll::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GNS => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gns::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GRS => {
                    let ctx = ctx.init(sentence);
                    let nmea = Grs::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GSA => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gsa::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GST => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gst::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GSV => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gsv::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::RMC => {
                    let ctx = ctx.init(sentence);
                    let nmea = Rmc::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::TXT => {
                    let ctx = ctx.init(sentence);
                    let nmea = Txt::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::VTG => {
                    let ctx = ctx.init(sentence);
                    let nmea = Vtg::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::ZDA => {
                    let ctx = ctx.init(sentence);
                    let nmea = Zda::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::DTM => {
                    let ctx = ctx.init(sentence);
                    let nmea = Dtm::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GBQ => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gbq::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GLQ => {
                    let ctx = ctx.init(sentence);
                    let nmea = Glq::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GNQ => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gnq::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::GPQ => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gpq::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::THS => {
                    let ctx = ctx.init(sentence);
                    let nmea = Ths::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
                Identifier::VLW => {
                    let ctx = ctx.init(sentence);
                    let nmea = Vlw::new(ctx, talker)?;
                    println!("{nmea:?}")
                }
            }
        }
    }
}
