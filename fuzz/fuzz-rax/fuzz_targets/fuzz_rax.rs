#![no_main]

use libfuzzer_sys::fuzz_target;
use rax::string::IStrFlowRule;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let is_ascii = s.is_ascii();
        let _ = rax::string::rules::ByteCount::<0>.apply(s, is_ascii);
        let _ = rax::string::rules::ByteCount::<1>.apply(s, is_ascii);
        let _ = rax::string::rules::ByteCount::<2>.apply(s, is_ascii);

        let _ = rax::string::rules::Char::<'c'>.apply(s, is_ascii);
        let _ = rax::string::rules::Char::<'\n'>.apply(s, is_ascii);
        let _ = rax::string::rules::Char::<'你'>.apply(s, is_ascii);

        let _ = rax::string::rules::CharCount::<0>.apply(s, is_ascii);
        let _ = rax::string::rules::CharCount::<1>.apply(s, is_ascii);
        let _ = rax::string::rules::CharCount::<2>.apply(s, is_ascii);
    }
});
