use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rax::string::IStrFlowRule;
use rax::string::filters::{CHAR_SET_ASCII_LETTERS_DIGITS, CharSetFilter, CHAR_SET_DIGITS};
use rax::string::rules::{
    ByteCount, Char, CharCount, NInCharSet, OneOfCharSet, UntilChar, UntilMode, UntilNInCharSet,
    UntilNotInCharSet, UntilOneInCharSet, UntilStr,
};

fn bench_rule<R: IStrFlowRule<'static>>(
    c: &mut Criterion,
    name: &str,
    rule: R,
    input: &'static str,
) {
    c.bench_function(name, |b| b.iter(|| rule.apply(black_box(input), true)));
}

fn benches(c: &mut Criterion) {
    bench_rule(c, "byte_count", ByteCount::<2>, "hello");
    bench_rule(c, "char_count", CharCount::<2>, "110324,foo,bar");
    bench_rule(c, "char", Char::<'a'>, "a123");
    bench_rule(
        c,
        "n_in_char_set",
        NInCharSet::<3, 62>(&CHAR_SET_ASCII_LETTERS_DIGITS),
        "abc123",
    );
    bench_rule(
        c,
        "one_in_char_set",
        OneOfCharSet(&CHAR_SET_ASCII_LETTERS_DIGITS),
        "a123",
    );
    bench_rule(
        c,
        "until_char",
        UntilChar::<';'> {
            mode: UntilMode::KeepInRest,
        },
        "123;abc",
    );
    bench_rule(
        c,
        "until_n_in_char_set",
        UntilNInCharSet::<2, 10> {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::KeepInRest,
        },
        "a1b2c3",
    );
    bench_rule(
        c,
        "until_not_in_char_set",
        UntilNotInCharSet {
            filter: &CHAR_SET_DIGITS,
            mode: UntilMode::KeepInRest,
        },
        "123abc",
    );
    const FILTER: CharSetFilter<2> = CharSetFilter::<2>::new([',', '*']);
    bench_rule(
        c,
        "until_one_in_char_set",
        UntilOneInCharSet {
            filter: &FILTER,
            mode: UntilMode::KeepInRest,
        },
        "0.7,1*38",
    );
    bench_rule(
        c,
        "until_str",
        UntilStr {
            pattern: ";",
            mode: UntilMode::KeepInRest,
        },
        "123;abc",
    );
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
