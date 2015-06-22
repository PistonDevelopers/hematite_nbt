#![feature(plugin, custom_derive)]
#![plugin(nbt_macros)]

extern crate nbt;

use std::io::Cursor;
use nbt::serialize::{from_reader, to_writer};

#[derive(Debug, PartialEq, NbtFmt)]
struct TupleStruct(i8, i8, String);

fn main() {
	let test = TupleStruct(0, 1, "TupleStruct".to_string());

    let mut dst = Vec::new();
    to_writer(&mut dst, &test).unwrap();

    let result: TupleStruct = from_reader(&mut Cursor::new(&dst[..])).unwrap();

    assert_eq!(&test, &result);
}
