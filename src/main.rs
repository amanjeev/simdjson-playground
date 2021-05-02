#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use simd_json;
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
    let reader = BufReader::new(data_file);

    for line in reader.lines() {
        //println!("{}", line.unwrap());
        let row: &mut str = &mut line.unwrap();
        let row: SIMDExample = simd_json::serde::from_str(row).unwrap();
        if row.id == 2794105886539304589 {
            println!("look ma! a match! - {}", row.id_str);
            break;
        } else {
            println!("No match yet");
        }
    }
}
