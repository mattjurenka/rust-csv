#![no_main]

extern crate csv;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data);
    for result in rdr.records() {
        match result {
            Ok(_) => {},
            Err(_) => {},
        }
    }
});
