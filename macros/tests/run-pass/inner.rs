#![feature(plugin, custom_derive)]
#![plugin(nbt_macros)]

extern crate nbt;

use std::io::Cursor;
use nbt::serialize::{from_reader, to_writer};

#[derive(Debug, PartialEq, Default, NbtFmt)]
struct Inner {
    name: String
}

#[derive(Debug, PartialEq, NbtFmt)]
struct Outer {
    name: String,
    inner: Inner
}

fn main() {
    let test = Outer {
        name: "Outer".to_string(),
        inner: Inner { name: "Inner".to_string() }
    };

    let mut dst = Vec::new();
    to_writer(&mut dst, &test).unwrap();

    let result: Outer = from_reader(&mut Cursor::new(&dst[..])).unwrap();

    assert_eq!(&test, &result);
}
