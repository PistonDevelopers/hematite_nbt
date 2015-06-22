#![feature(plugin, custom_derive)]
#![plugin(nbt_macros)]

#[derive(NbtFmt)] //~ ERROR #[derive(NbtFmt)] only allowed on structs.
fn double(x: i8) -> i8 {
	x * 2
}
