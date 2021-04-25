#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::borrow::Cow;
use simd_json;

#[derive(Debug, Deserialize)]
struct Foo<'input> {
    #[serde(borrow)]
    bar: Cow<'input, str>,
}

#[derive(Debug, Deserialize)]
struct SIMDExample<'sin> {
    #[serde(borrow)]
    id: Cow<'sin, i64>
}

fn main() {
    let input = r#"{ "bar": "baz" }"#;
    let foo: Foo = serde_json::from_str(input).unwrap();

    match foo.bar {
        Cow::Borrowed(x) => println!("borrowed {}", x),
        Cow::Owned(x) => println!("owned {}", x),
    }
}