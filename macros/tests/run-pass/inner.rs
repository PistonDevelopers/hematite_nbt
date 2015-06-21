#![feature(plugin, custom_derive)]
#![plugin(nbt_macros)]

extern crate nbt;

#[derive(Default, NbtFmt)]
struct Inner {
    name: String
}

#[derive(NbtFmt)]
struct Outer {
    name: String,
    inner: Inner
}

fn main() { }
