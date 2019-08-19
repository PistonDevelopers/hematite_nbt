# hematite_nbt [![hematite_nbt at crates.io](https://img.shields.io/crates/v/hematite_nbt.svg)](https://crates.io/crates/hematite_nbt) [![hematite_nbt at docs.rs](https://docs.rs/hematite-nbt/badge.svg)](https://docs.rs/hematite-nbt) [![Build Status](https://travis-ci.org/PistonDevelopers/hematite_nbt.svg?branch=master)](https://travis-ci.org/PistonDevelopers/hematite_nbt)

This repository contains the [Hematite project](http://hematite.piston.rs/)'s
standalone `nbt` crate for working with Minecraft's [Named Binary Tag](https://minecraft.gamepedia.com/NBT_format)
(NBT) format.

This is not the only NBT-related crate available, but it has some notable
features:

* Full support for serializing and deserializing types via [Serde](https://serde.rs/).
  This means that you can read and write the NBT binary format of any struct
  annotated with the standard `#[derive(Serialize, Deserialize)]` traits
  (provided it actually has a valid NBT representation).

* An API that attempts to differentiate between complete and partial NBT objects
  via `nbt::Blob` and `nbt::Value`. Only complete objects can be serialized.

* Support for the `TAG_Long_Array` data introduced in Minecraft 1.12.

* Support for the modified UTF-8 encoding used by the vanilla Minecraft client.

## License

Licensed under the terms of the MIT license.
