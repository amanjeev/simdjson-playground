#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rand::distributions::{Alphanumeric, Standard, Uniform};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use simd_json;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::iter;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
struct SIMDExample<'sin> {
    id: u64,
    #[serde(borrow)]
    id_str: &'sin str,
}

fn main() {
    // generate data
    let mut fake_data_file = Arc::new(Mutex::new(
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open("/home/aj/deploy/rust/zcapjson/data/fake_data.json")
            .unwrap(),
    ));

    fake_data_file.lock().unwrap().write_all(b"{[").unwrap();
    println!("{}", "wrote {[\n");

    (0..u64::MAX).into_par_iter().for_each(|item| {
        let mut rng = thread_rng();

        let s: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(150)
            .collect();
        let fake_data_element = SIMDExample {
            id: item,
            id_str: s.as_str(),
        };

        let serialized_fake_data_element = serde_json::to_string(&fake_data_element).unwrap();
        fake_data_file
            .lock()
            .unwrap()
            .write_all(serialized_fake_data_element.as_bytes())
            .unwrap();
        fake_data_file.lock().unwrap().write_all(b",\n").unwrap();

        println!("wrote {:?}", serialized_fake_data_element);
    });

    fake_data_file.lock().unwrap().write_all(b"]}");
    println!("{}", "wrote ]}");

    let contents = fs::read_to_string("/home/aj/deploy/rust/zcapjson/data/twitter.json");
    let contents = &mut contents.unwrap()[..]; // get that intro str on stack

    let x: serde_json::Value = simd_json::serde::from_str(contents).unwrap();
}
