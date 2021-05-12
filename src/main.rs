use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use simd_json::AlignedBuf;
use simd_json_derive::Deserialize;

#[derive(simd_json_derive::Deserialize, simd_json_derive::Serialize)]
struct SIMDExample<'sin> {
    id: u64,
    #[serde(borrow)]
    id_str: &'sin str,
}

fn main() {
    let data_file = File::open("/home/aj/deploy/rust/zcapjson/data/fake_data.json").unwrap();
    let mut reader = BufReader::new(data_file);

    let mut data = Vec::with_capacity(1024);

    let mut string_buffer = Vec::with_capacity(2048);
    let mut input_buffer = AlignedBuf::with_capacity(1024);

    while reader.read_until(b'\n', &mut data).unwrap() > 0 {
        let row =
            SIMDExample::from_slice_with_buffers(&mut data, &mut input_buffer, &mut string_buffer)
                .unwrap();

        if row.id == 2807149942735425369 {
            println!("look ma! a match! - {}", row.id_str)
        }
        data.clear();
    }
}
