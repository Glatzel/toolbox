#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }

    let value = u32::from_le_bytes([
        data[0],
        data[1],
        data[2],
        data[3],
    ]);

    // Bug: only crashes for a specific value
    if value == 0xDEADBEEF {
        panic!("found magic value");
    }
});
