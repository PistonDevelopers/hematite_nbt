#![feature(plugin, custom_derive)]
#![plugin(nbt_macros)]

#[derive(NbtFmt)] //~ ERROR `NbtFmt` has no meaning for unit structs.
struct UnitStruct;
