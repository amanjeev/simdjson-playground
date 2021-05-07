use simd_json_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Serialize, simd_json_derive::Deserialize)]
struct SIMDExample<'sin> {
    id: u64,
    id_str: &'sin str,
}

fn main() {
    let data_file = File::open("/home/aj/deploy/rust/zcapjson/data/fake_data.json").unwrap();
    let reader = BufReader::new(data_file);

    for line in reader.lines() {
        //println!("{}", line.unwrap());
        let row: &mut str = &mut line.unwrap();
        let row: SIMDExample = SIMDExample::from_str(row).unwrap();

        match row.id {
            2807149942735425369 => println!("look ma! a match! - {}", row.id_str),
            _ => println!("No match yet"),
        }
    }
}
