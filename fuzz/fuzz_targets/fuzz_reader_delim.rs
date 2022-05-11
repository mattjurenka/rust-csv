#![no_main]

extern crate csv;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let delim = if data.len() > 0 {
        data[0]
    } else {
        b':'
    };
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delim)
        .from_reader(data);
    for result in rdr.records() {
        match result {
            Ok(_) => {},
            Err(_) => {},
        }
    }
});
