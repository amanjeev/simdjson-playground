use serde;
use serde_json;
use simd_json_derive::Deserialize;
use simd_json_derive::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Deserialize, Serialize)]
struct SIMDExample<'sin> {
    id: u64,
    #[serde(borrow)]
    id_str: &'sin str,
}

fn main() {
    let data_file = File::open("/home/aj/deploy/rust/zcapjson/data/fake_data.json").unwrap();
    let reader = BufReader::new(data_file);

    for line in reader.lines() {
        let mut row = line.unwrap();
        let row: SIMDExample = SIMDExample::from_str(row.as_mut_str()).unwrap();

        match row.id {
            2807149942735425369 => println!("look ma! a match! - {}", row.id_str),
            _ => println!("No match yet"),
        }
    }
}
