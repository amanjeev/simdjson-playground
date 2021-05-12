#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
struct SIMDExample<'sin> {
    id: u64,
    #[serde(borrow)]
    id_str: &'sin str,
}

fn main() {
    let data_file = File::open("/home/aj/deploy/rust/zcapjson/data/fake_data.json").unwrap();
    let mut reader = BufReader::new(data_file);

    let mut data = Vec::with_capacity(1024);

    while reader.read_until(b'\n', &mut data).unwrap() > 0 {
        let row: SIMDExample = serde_json::from_slice(data.as_mut_slice()).unwrap();
        if row.id == 2807149942735425369 {
            println!("look ma! a match! - {}", row.id_str);
        }
        data.clear();
    }
}
