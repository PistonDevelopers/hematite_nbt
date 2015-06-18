#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(nbt_macros)]

extern crate nbt;

#[derive(NbtFmt)]
struct Simple {
    #[nbt_field = "names"]
    n: String,
}

fn main() { }
