#![no_main]

extern crate csv;
use libfuzzer_sys::fuzz_target;

#[macro_use]
extern crate serde_derive;

#[derive(Debug,Deserialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

fuzz_target!(|data: &[u8]| {
    let mut rdr = csv::Reader::from_reader(data);
    for result in rdr.deserialize::<Record>() {
        match result {
            Ok(_) => {},
            Err(_) => {},
        }
    }
});
