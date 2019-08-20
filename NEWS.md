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
