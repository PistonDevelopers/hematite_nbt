# hematite_nbt 0.5.0

## New Features

* Compiling with the `preserve_order` feature will use an `IndexMap` instead of
  a `HashMap` for compound tags, which preserves insertion order when
  serializing. (#54 by @samlich)

* Structs can now be explicitly told to serialize as byte/int/long arrays
  instead of a list under Serde by using `serialize_with` and the new
  `i8_array()`, `i32_array()`, and `i64_array()` functions. (#51, #52, and #53
  by @Schuwi). For example:

```
#[derive(Serialize)]
struct MyStruct {
    #[serde(serialize_with="hematite_nbt::i8_array")]
    byte_array: Vec<i8>,
}
```

* `Blob` now has a `len_bytes()` method to compute the expected length when
  serialized, restoring functionality removed in #36. It turns out to be helpful
  to know the length when writing NBT payloads into network packets. (#48 by
  @ejmount)

* `Blob` now has a `get()` method to get values by name; previously this was
  only possible via indexing, which would panic if the key was absent. (#44 by
  @vlakreeh)

* `Blob` now implements `Default::default()`, as suggested by Clippy.

## Bug Fixes and Other Improvements

* The `flate2` dependency has been bumped to enable compilation for WASM. (#56
  by @oOBoomberOo)

* The use of various deprecated APIs in the Rust standard library has been
  fixed, so consumers of this crate will no longer see build warnings. (#49 by
  @atheriel)

* `UnrepresentableType` will now correctly show the type in the error message,
  instead of the literal string `$type`. (#47 by @caelunshun)

* The benchmark suite has been rewritten to use Criterion, so it should compile
  with stable Rust. However, it's not clear that the black-box benchmarks are
  actually working properly at this time.

* The project now uses and enforces `rustfmt` rules.

# hematite_nbt 0.4.1

* Strings are now encoded and decoded using [Modified UTF-8](https://en.wikipedia.org/wiki/UTF-8#Modified_UTF-8),
  matching the behaviour of the vanilla Minecraft client. This should only
  affect strings with codepoints outside of the Basic Multilingual Plane, which
  are likely quite rare in the wild. Note that we permissively handle true UTF-8
  input, but always write MUTF-8 output. (#39)

* Serde-based deserialization now supports `TAG_LongArray`. (#41 by @caelunshun)

* Empty lists are now written with `TAG_End` as the list type instead of
  `TAG_Byte`. This matches the behaviour of the vanilla Minecraft client,
  although it should not in theory affect well-behaved NBT parsers.

* Fixes Serde-based serialization of empty lists.

* Updates the project `README` and adds a `NEWS` file.

# hematite_nbt 0.4.0

## Breaking Changes

* `Blob::new()` no longer takes a name parameter; use `Blob::named()` instead.
  (#36)

* Implementation details on `Blob` and `Value` including `read_header()`,
  `write_header()`, and `len()` are no longer public or have been removed. (#36)

* `Value::write()` is now `Value::to_writer()`, matching the rest of the API.
  Similarly, `Blob::(to|from)_(gzip|zlib)` methods now include a
  `(reader|writer)` prefix. (#36)

* The unfinished `macros` module and the associated `NbtFmt` trait have been
  removed. These are superceded by Serde support. (#25)

## New Features

* Support for (de)serializing Rust types (including `Blob`) to NBT using the
  Serde framework. This is an optional feature but enabled by default. (#24,
  #30, #31)

* Support for `TAG_LongArray` via `Value::LongArray`. (#28 by @williewillus)

* Improved printing support for `Blob` and `Value` and added an example program
  for printing out NBT files.

# hematite_nbt 0.3.0

* Bumps the `byteorder` dependency to 1.0.0 and makes associated changes.
