use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rax::text::IFlowRule;
use rax::text::filters::{CHAR_SET_ASCII_LETTERS_DIGITS, CHAR_SET_DIGITS, CharSetFilter};
use rax::text::rules::{
    ByteCount, Char, CharCount, NInCharSet, OneOfCharSet, UntilChar, UntilMode, UntilNInCharSet,
    UntilNotInCharSet, UntilOneInCharSet, UntilStr,
};

fn bench_rule<R: IFlowRule<true>>(c: &mut Criterion, name: &str, rule: R, input: &'static str) {
    c.bench_function(name, |b| b.iter(|| rule.apply(black_box(input))));
}

fn benches(c: &mut Criterion) {
    bench_rule(c, "byte_count", ByteCount::<2, true>, "hello");
    bench_rule(c, "char_count", CharCount::<2, true>, "110324,foo,bar");
    bench_rule(c, "char", Char::<'a', true>, "a123");
    bench_rule(
        c,
        "n_in_char_set",
        NInCharSet::<4, true, _, _>(&CHAR_SET_ASCII_LETTERS_DIGITS),
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
        UntilChar::<';', true> {
            mode: UntilMode::KeepInRest,
        },
        "123;abc",
    );
    bench_rule(
        c,
        "until_n_in_char_set",
        UntilNInCharSet::<2, true, _, _> {
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
