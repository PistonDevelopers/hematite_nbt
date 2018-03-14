# hematite_nbt [![Build Status](https://travis-ci.org/PistonDevelopers/hematite_nbt.svg?branch=master)](https://travis-ci.org/PistonDevelopers/hematite_nbt)

`hematite-nbt` is a Rust library for reading and writing Minecraft's
[Named Binary Tag file format](http://minecraft.gamepedia.com/NBT_format) (NBT).

This crate is maintained by the [Hematite][] project, and is used in the
[Hematite server][].

## Basic Usage

The `nbt` crate can be used to read and write NBT-format streams. NBT-format
files are represented as `Blob` objects, which contain NBT-compatible `Value`
elements. For example:

```rust
use nbt::{Blob, Value};

// Create a `Blob` from key/value pairs.
let mut nbt = Blob::new("".to_string());
nbt.insert("name".to_string(), "Herobrine").unwrap();
nbt.insert("health".to_string(), 100i8).unwrap();
nbt.insert("food".to_string(), 20.0f32).unwrap();

// Write a compressed binary representation to a byte array.
let mut dst = Vec::new();
nbt.write_zlib(&mut dst).unwrap();
```

As of 0.3, the API is still experimental, and may change in future versions.

## Usage with `serde_derive`

This repository also contains an `nbt-serde` crate that supports reading and
writing the NBT format using the [`serde` framework][]. This enables
automatically deriving serialization/deserialization of user-defined types,
bypassing the use of the generic `Blob` and `Value` types illustrated above.

For example:

``` rust
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate nbt_serde;

use nbt_serde::encode::to_writer;

#[derive(Debug, PartialEq, Serialize)]
struct MyNbt {
    name: String,
    score: i8,
}

fn main() {
    let nbt = MyNbt { name: "Herobrine".to_string(), score: 42 };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let read: MyNbt = from_reader(&dst[..]).unwrap();

    assert_eq!(read, nbt);
}
```

This approach can be considerably faster if one knows the data encoded in the
incoming NBT stream beforehand.

## Installation

Neither `nbt` nor `nbt_serde` is available on [crates.io][]. However, they can
still be listed as a dependency in your `Cargo.toml` file as follows:

``` toml
[dependencies]
hematite-nbt = { git = "https://github.com/PistonDevelopers/hematite_nbt.git" }
hematite-nbt-serde = { git = "https://github.com/PistonDevelopers/hematite_nbt.git" }
```

(You can also clone the repository into a subdirectory and specify the crates as
a dependency with `path = "..."` if you wish.)

## License

All code is available under the terms of the MIT license. See the `LICENSE` file
for details.

[Hematite]: http://hematite.piston.rs/ (Hematite)
[Hematite server]: https://github.com/PistonDevelopers/hematite_server (github: PistonDevelopers: hematite_server)
[Minecraft]: https://minecraft.net/ (Minecraft)
[Rust]: http://www.rust-lang.org/ (The Rust Programming Language)
[crates.io]: https://crates.io/ (crates.io)
[`serde` framework]: https://serde.rs/
