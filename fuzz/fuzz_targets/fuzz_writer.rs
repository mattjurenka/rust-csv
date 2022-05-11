#![no_main]

extern crate csv;
use libfuzzer_sys::fuzz_target;
use std::io;

fuzz_target!(|data: &[u8]| {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    match wtr.write_record(&[data]) {
        Ok(_) => {},
        Err(_) => {},
    }
});
