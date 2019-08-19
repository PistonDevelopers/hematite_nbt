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
