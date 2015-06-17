#![feature(plugin, custom_derive)]
#![plugin(nbt_macros)]

// This throws many, many errors.

#[derive(NbtFmt)]
      //~^ ERROR failed to resolve. Maybe a missing `extern crate nbt`?
      //~^^ ERROR use of undeclared trait name `nbt::serialize::NbtFmt`
      //~^^^ ERROR failed to resolve. Maybe a missing `extern crate nbt`?
      //~^^^^ ERROR use of undeclared type name `nbt::Error`
      //~^^^^^ ERROR failed to resolve. Maybe a missing `extern crate nbt`?
      //~^^^^^^ ERROR unresolved name `nbt::serialize::close_nbt`
struct One {
	byte: i8
	//~^ ERROR failed to resolve. Maybe a missing `extern crate nbt`?
    //~^^ ERROR unresolved name `nbt::serialize::NbtFmt::to_nbt`
}
