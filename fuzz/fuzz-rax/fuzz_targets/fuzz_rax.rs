#![no_main]

use libfuzzer_sys::fuzz_target;
use rax::text::IFlowRule;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        match s.is_ascii() {
            true => {
                let _ = rax::text::rules::ByteCount::<0, true>.apply(s);
                let _ = rax::text::rules::ByteCount::<1, true>.apply(s);
                let _ = rax::text::rules::ByteCount::<2, true>.apply(s);

                let _ = rax::text::rules::Char::<'c', true>.apply(s);
                let _ = rax::text::rules::Char::<'\n', true>.apply(s);
                let _ = rax::text::rules::Char::<'你', true>.apply(s);

                let _ = rax::text::rules::CharCount::<0, true>.apply(s);
                let _ = rax::text::rules::CharCount::<1, true>.apply(s);
                let _ = rax::text::rules::CharCount::<2, true>.apply(s);
            }
            false => {
                let _ = rax::text::rules::ByteCount::<0, false>.apply(s);
                let _ = rax::text::rules::ByteCount::<1, false>.apply(s);
                let _ = rax::text::rules::ByteCount::<2, false>.apply(s);

                let _ = rax::text::rules::Char::<'c', false>.apply(s);
                let _ = rax::text::rules::Char::<'\n', false>.apply(s);
                let _ = rax::text::rules::Char::<'你', false>.apply(s);

                let _ = rax::text::rules::CharCount::<0, false>.apply(s);
                let _ = rax::text::rules::CharCount::<1, false>.apply(s);
                let _ = rax::text::rules::CharCount::<2, false>.apply(s);
            }
        }
    }
});
