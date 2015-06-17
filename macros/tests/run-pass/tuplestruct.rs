#![feature(plugin, custom_derive)]
#![plugin(nbt_macros)]

extern crate nbt;

#[derive(NbtFmt)]
struct TupleStruct(i8, i8, String);

fn main() { }
