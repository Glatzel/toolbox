use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rax::str_parser::{IStrFlowRule, IStrGlobalRule};
use rax_nmea::rules::*;
fn bench_rule<R: IStrFlowRule<'static>>(
    c: &mut Criterion,
    name: &str,
    rule: R,
    input: &'static str,
) {
    c.bench_function(name, |b| b.iter(|| rule.apply(black_box(input))));
}
fn benches(c: &mut Criterion) {
    bench_rule(c, "coord", NmeaCoord, "12319.123,E,rest");
    bench_rule(c, "date", NmeaDate, "110324,foo,bar");
    bench_rule(c, "degree", NmeaDegree, "123.45,N,other_data");
    bench_rule(c, "time", NmeaTime, "123456.789,foo,bar");
}

fn bench_validate(c: &mut Criterion) {
    c.bench_function("validate", |b| {
        b.iter(|| {
            NmeaValidate.apply(black_box(
                "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47",
            ))
        })
    });
}
criterion_group!(benches_group, benches, bench_validate);
criterion_main!(benches_group);
