#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(nbt_macros)]

extern crate nbt;

use std::io::Cursor;
use nbt::serialize::{from_reader, to_writer};

#[derive(Debug, PartialEq, NbtFmt)]
struct Simple {
    #[nbt_field = "name"]
    n: String,
}

fn main() {
    let test = Simple { n: "Simple".to_string() };

    let mut dst = Vec::new();
    to_writer(&mut dst, &test).unwrap();

    let result: Simple = from_reader(&mut Cursor::new(&dst[..])).unwrap();

    assert_eq!(&test, &result);
}
