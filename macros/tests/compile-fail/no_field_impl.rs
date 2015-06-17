#![feature(plugin, custom_derive)]
#![plugin(nbt_macros)]

extern crate nbt;

struct Inner {
    name: String
}

#[derive(NbtFmt)]
struct Outer {
    name: String,
    inner: Inner //~ ERROR the trait `nbt::serialize::NbtFmt` is not implemented for the type `Inner`
}

fn main() { }
