use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rax::str_parser::StrParserContext;
use rax_nmea::data::{INmeaData, Talker};

fn bench_nmea<'a, F, D>(c: &mut Criterion, name: &str, sentence: &'static str, ctor: F)
where
    F: Fn(&mut StrParserContext, Talker) -> mischief::Result<D> + 'static,
    D: INmeaData,
{
    let mut ctx = StrParserContext::new();
    ctx.init(sentence.to_string());
    c.bench_function(name, move |b| {
        b.iter(|| {
            ctx.reset();
            ctor(black_box(&mut ctx), black_box(Talker::GN)).unwrap();
        })
    });
}

pub fn benches(c: &mut Criterion) {
    bench_nmea(
        c,
        "dhv",
        "$GNDHV,021150.000,0.03,0.006,-0.042,-0.026,0.06*65",
        rax_nmea::data::Dhv::new,
    );
    bench_nmea(
        c,
        "dtm",
        "$GPDTM,999,,0.08,N,0.07,E,-47.7,W84*1B",
        rax_nmea::data::Dtm::new,
    );
    bench_nmea(c, "gbq", "$EIGBQ,RMC*28", rax_nmea::data::Gbq::new);
    bench_nmea(
        c,
        "gbs",
        "$GPGBS,125027,23.43,M,13.91,M,34.01,M*07",
        rax_nmea::data::Gbs::new,
    );
    bench_nmea(
        c,
        "gga",
        "$GPGGA,110256,5505.676996,N,03856.028884,E,2,08,0.7,2135.0,M,14.0,M,,*7D",
        rax_nmea::data::Gga::new,
    );
    bench_nmea(
        c,
        "gll",
        "$GPGLL,2959.9925,S,12000.0090,E,235316.000,A,A*4E",
        rax_nmea::data::Gll::new,
    );
    bench_nmea(c, "glq", "$EIGLQ,RMC*26", rax_nmea::data::Glq::new);
    bench_nmea(c, "gnq", "$EIGNQ,RMC*24", rax_nmea::data::Gnq::new);
    bench_nmea(
        c,
        "gns",
        "$GPGNS,112257.00,3844.24011,N,00908.43828,W,AN,03,10.5,,*57",
        rax_nmea::data::Gns::new,
    );
    bench_nmea(c, "gpq", "$EIGPQ,RMC*3A", rax_nmea::data::Gpq::new);
    bench_nmea(
        c,
        "grs",
        "$GPGRS,220320.0,0,-0.8,-0.2,-0.1,-0.2,0.8,0.6,,,,,,,*55",
        rax_nmea::data::Grs::new,
    );
    bench_nmea(
        c,
        "gsa",
        "$GNGSA,A,3,05,07,13,14,15,17,19,23,24,,,,1.0,0.7,0.7,1*38",
        rax_nmea::data::Gsa::new,
    );
    bench_nmea(
        c,
        "gst",
        "$GPGST,182141.000,15.5,15.3,7.2,21.8,0.9,0.5,0.8*54",
        rax_nmea::data::Gst::new,
    );
    bench_nmea(
        c,
        "gsv",
        "$GPGSV,3,1,10,25,68,053,47,21,59,306,49,29,56,161,49,31,36,265,49*79\r\n$GPGSV,3,2,10,12,29,048,49,05,22,123,49,18,13,000,49,01,00,000,49*72\r\n$GPGSV,3,3,10,14,00,000,03,16,00,000,27*7C",
        rax_nmea::data::Gsv::new,
    );
    bench_nmea(
        c,
        "rmc",
        "$GPRMC,110125,A,5505.337580,N,03858.653666,E,148.8,84.6,310317,8.9,E,D*2E",
        rax_nmea::data::Rmc::new,
    );
    bench_nmea(c, "ths", "$GPTHS,77.52,E*34", rax_nmea::data::Ths::new);
    bench_nmea(
        c,
        "txt",
        "$GPTXT,03,01,02,MA=CASIC*25\r\n$GPTXT,03,02,02,IC=ATGB03+ATGR201*70\r\n$GPTXT,03,03,02,SW=URANUS2,V2.2.1.0*1D",
        rax_nmea::data::Txt::new,
    );
    bench_nmea(
        c,
        "vlw",
        "$GPVLW,,N,,N,15.8,N,1.2,N*65",
        rax_nmea::data::Vlw::new,
    );
    bench_nmea(
        c,
        "vtg",
        "$GPVTG,83.7,T,83.7,M,146.3,N,271.0,K,D*22",
        rax_nmea::data::Vtg::new,
    );
    bench_nmea(
        c,
        "zda",
        "$GPZDA,160012.71,11,03,2004,-1,00*7D",
        rax_nmea::data::Zda::new,
    );
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
