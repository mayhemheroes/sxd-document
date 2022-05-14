#![no_main]
use libfuzzer_sys::fuzz_target;

use sxd_document::parser;

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok(s) => {
            match parser::parse(s) {
                Ok(pack) => {
                    let _ = pack.as_document();
                },
                _ => {},
            }
        },
        _ => {},
    }
    // fuzzed code goes here
});
