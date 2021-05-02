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
    let reader = BufReader::new(data_file);

    for line in reader.lines() {
        //println!("{}", line.unwrap());
        let row: &mut str = &mut line.unwrap();
        let row: SIMDExample = serde_json::from_str(row).unwrap();
        match row.id {
            2807149942735425369 => println!("look ma! a match! - {}", row.id_str),
            _ => println!("No match yet"),
        }
    }
}
