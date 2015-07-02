//! Contains functions for serializing arbitrary objects to the Named Binary
//! Tag format.
//!
//! For working with existing serialization implementations, see `to_writer`.
//! For custom types, implement the `NbtFmt` trait.

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};

use error::{Error, Result};

pub mod raw;

/// A trait indicating that the type has a Named Binary Tag representation.
///
/// Keep in mind that not all Rust types (notably unsigned integers) have an
/// obvious NBT representation, and so structs that implement this trait may
/// have to convert them to one that does.
///
/// ## Usage with Derive
///
/// A compiler plugin is available in the `nbt_macros` package to enable
/// automatic derivation of NBT encoding/decoding for types. This is heavily
/// recommended over implementing this trait by hand. Usage is generally as
/// simple as the following:
///
/// ```ignore
/// #![feature(plugin, custom_derive)]
/// #![plugin(nbt_macros)]
///
/// extern crate nbt;
///
/// use nbt::serialize::{NbtFmt, to_writer};
///
/// #[derive(NbtFmt)]
/// struct MyMob {
///     name: String,
///     health: i8
/// }
///
/// fn main() {
///     let mut bytes = Vec::new();
///     let mob = MyMob { name: "Dr. Evil".to_string(), health: 240 };
///
///     to_writer(&mut bytes, &mob).unwrap();
/// }
/// ```
///
/// The package's documentation provides more detailed usage.
///
/// ## Manual Implementation
///
/// While it is not advisable to implement `NbtFmt` by hand, the code below is
/// similar to what the automated derivation produces:
///
/// ```rust
/// extern crate nbt;
///
/// use std::io::Cursor;
/// use nbt::serialize::*;
///
/// #[derive(Debug, PartialEq)]
/// struct MyMob {
///     name: String,
///     health: i8
/// }
///
/// impl NbtFmt for MyMob {
///     type Into = MyMob;
///
///     fn to_bare_nbt<W>(&self, dst: &mut W) -> nbt::Result<()>
///        where W: std::io::Write
///     {
///         try!(self.name.to_nbt(dst, "name"));
///         try!(self.health.to_nbt(dst, "health"));
///
///         close_nbt(dst)
///     }
///
///     fn read_bare_nbt<R>(src: &mut R) -> nbt::Result<MyMob>
///        where R: std::io::Read
///     {
///         let mut __name: String = Default::default();
///         let mut __health: i8 = Default::default();
///
///         loop {
///             let (t, n) = try!(emit_next_header(src));
///
///             if t == 0x00 { break; } // i.e. Tag_End
///
///             match &n[..] {
///                 "name" => {
///                     __name = try!(read_bare_nbt(src));
///                 },
///                 "health" => {
///                     __health = try!(read_bare_nbt(src));
///                 },
///                 e => {
///                     return Err(nbt::Error::UnexpectedField(e.to_string()));
///                 },
///             };
///         }
///
///         Ok(MyMob { name: __name, health: __health })
///     }
/// }
///
/// fn main() {
///     let mut bytes = Vec::new();
///     let mob = MyMob { name: "Dr. Evil".to_string(), health: 240 };
///
///     to_writer(&mut bytes, &mob).unwrap();
///     let read_mob: MyMob = from_reader(&mut Cursor::new(bytes.clone())).unwrap();
///
///     assert_eq!(&mob, &read_mob);
/// }
/// ```
pub trait NbtFmt {
    type Into: Sized = Self;

    /// Convert this type to NBT format using the specified `io::Write`
    /// destination, but does not serialize its identifying NBT tag or name.
    fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
       where W: io::Write;

    /// Reads from the specified `io::Read` source bytes that can be coverted
    /// into an instance of this type.
    fn read_bare_nbt<R>(src: &mut R) -> Result<Self::Into>
       where R: io::Read;

    /// Convert this type to NBT format using the specified `io::Write`
    /// destination, incuding its tag and a given name.
    #[inline]
    fn to_nbt<W, S>(&self, dst: &mut W, name: S) -> Result<()>
       where W: io::Write,
             S: AsRef<str>
    {
        try!(dst.write_u8(Self::tag()));
        try!(raw::write_bare_string(dst, name.as_ref()));
        self.to_bare_nbt(dst)
    }

    /// Indicates the NBT tag that this type corresponds to. Most custom types
    /// (usually structs) will advertise the default, `0x0a`, which is the
    /// default.
    #[inline] fn tag() -> u8 { 0x0a }

    /// Indicates whether this type is "bare", in that it must be wrapped in an
    /// NBT Compound before serialization. By default this is `false`, since
    /// most imeplementations will be Compound-like objects. Primitive NBT
    /// types (`i8`, `i16`, `String`, etc.) return `true`.
    #[inline] fn is_bare() -> bool { false }
}

/// A convenience function for closing NBT format objects.
///
/// This function writes a single `0x00` byte to the `io::Write` destination,
/// which in the NBT format indicates that an open Compound is now closed.
pub fn close_nbt<W>(dst: &mut W) -> Result<()>
    where W: io::Write {

    dst.write_u8(0x00).map_err(From::from)
}

/// Extracts the next header (tag and name) from an NBT format source.
///
/// This function will also return the `TAG_End` byte and an empty name if it
/// encounters it.
pub fn emit_next_header<R>(src: &mut R) -> Result<(u8, String)>
    where R: io::Read
{
    let tag  = try!(src.read_u8());

    match tag {
        0x00 => { Ok((tag, "".to_string())) },
        _    => {
            let name = try!(raw::read_bare_string(src));
            Ok((tag, name))
        },
    }
}

/// Serializes an object into NBT format at a given destination.
///
/// This function will try to ensure that the output is always a valid NBT
/// file, i.e. that it has a top-level Compound.
pub fn to_writer<W, T>(dst: &mut W, obj: &T) -> Result<()>
    where W: io::Write,
          T: NbtFmt
{
    match T::is_bare() {
        // Refuse to blindly serialize types not wrapped in an NBT Compound.
        true  => { return Err(Error::NoRootCompound); },
        false => obj.to_nbt(dst, ""),
    }
}

/// Deserializes an object in NBT format from a given source.
pub fn from_reader<R, T>(src: &mut R) -> Result<T>
    where R: io::Read,
          T: NbtFmt<Into=T>
{
    if T::is_bare() {
        // Valid NBT files do not contain bare types.
        return Err(Error::NoRootCompound);
    }

    let next_tag  = try!(src.read_u8());
    if next_tag != T::tag() {
        return Err(Error::TagMismatch(next_tag, T::tag()));
    }

    let _ = try!(raw::read_bare_string(src));
    let rval = try!(T::read_bare_nbt(src));

    Ok(rval)
}

/// Deserializes a bare object (i.e. with no name or tag) from a given source.
pub fn read_bare_nbt<R, T>(src: &mut R) -> Result<T>
    where R: io::Read,
          T: NbtFmt<Into=T>
{
    (T::read_bare_nbt(src)).map_err(From::from)
}

macro_rules! nbtfmt_value {
  ($T:ty, $read_method:expr, $write_method:expr, $tag:expr) => (
    impl NbtFmt for $T {
        #[inline]
        fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
           where W: io::Write
        {
            $write_method(dst, *self)
        }

        #[inline]
        fn read_bare_nbt<R>(src: &mut R) -> Result<$T>
           where R: io::Read
        {
            $read_method(src)
        }

        #[inline] fn tag() -> u8 { $tag }
        #[inline] fn is_bare() -> bool { true }
    }
  )
}

macro_rules! nbtfmt_ptr {
  ($T:ty, $Into:ty, $read_method:expr, $write_method:expr, $tag:expr) => (
    impl NbtFmt for $T {
        type Into = $Into;

        #[inline]
        fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
           where W: io::Write
        {
            $write_method(dst, self)
        }

        #[inline]
        fn read_bare_nbt<R>(src: &mut R) -> Result<$Into>
           where R: io::Read
        {
            $read_method(src)
        }

        #[inline] fn tag() -> u8 { $tag }
        #[inline] fn is_bare() -> bool { true }
    }
  )
}

macro_rules! nbtfmt_slice {
  ($T:ty, $read_method:expr, $write_method:expr, $tag:expr) => (
    impl NbtFmt for $T {
        #[inline]
        fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
           where W: io::Write
        {
            $write_method(dst, &self[..])
        }

        #[inline]
        fn read_bare_nbt<R>(src: &mut R) -> Result<$T>
           where R: io::Read
        {
            $read_method(src)
        }

        #[inline] fn tag() -> u8 { $tag }
        #[inline] fn is_bare() -> bool { true }
    }
  )
}

nbtfmt_value!(i8, raw::read_bare_byte, raw::write_bare_byte, 0x01);
nbtfmt_value!(i16, raw::read_bare_short, raw::write_bare_short, 0x02);
nbtfmt_value!(i32, raw::read_bare_int, raw::write_bare_int, 0x03);
nbtfmt_value!(i64, raw::read_bare_long, raw::write_bare_long, 0x04);
nbtfmt_value!(f32, raw::read_bare_float, raw::write_bare_float, 0x05);
nbtfmt_value!(f64, raw::read_bare_double, raw::write_bare_double, 0x06);
nbtfmt_ptr!(str, String, raw::read_bare_string, raw::write_bare_string, 0x08);
nbtfmt_slice!(String, raw::read_bare_string, raw::write_bare_string, 0x08);

// For now, to handle conflicting implementations, use slices to indicate when
// byte and int arrays should be preferred to lists.

nbtfmt_ptr!([i8], Vec<i8>, raw::read_bare_byte_array, raw::write_bare_byte_array, 0x07);
nbtfmt_ptr!([i32], Vec<i32>, raw::read_bare_int_array, raw::write_bare_int_array, 0x0b);

// FIXME: Remove this workaround and enable some way of uncommenting the lines
// that follow.

// nbtfmt_slice!(Vec<i8>, write_bare_byte_array, 0x07);
// nbtfmt_slice!(Vec<i32>, write_bare_int_array, 0x0b);

// impl<T> NbtFmt for [T] where T: NbtFmt {
//  fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
//        where W: io::Write {

//          write_bare_list(dst, self.iter())
//  }
//     #[inline] fn tag() -> u8 { 0x09 }
//     #[inline] fn is_bare() -> bool { true }
// }

// This leaves Vec<T> alone for lists (which kind of makes sense).

impl<T> NbtFmt for Vec<T> where T: NbtFmt<Into=T> {
    type Into = Vec<T>;

    #[inline]
    fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
       where W: io::Write
    {
        raw::write_bare_list(dst, self.iter())
    }

    #[inline]
    fn read_bare_nbt<R>(src: &mut R) -> Result<Vec<T>>
       where R: io::Read
    {
        let tag = try!(src.read_u8());
        if tag != T::tag() {
            // FIXME: New error needed for this.
            return Err(Error::IncompleteNbtValue);
        }

        // Err(Error::IncompleteNbtValue)

        raw::read_bare_list(src)
    }

    #[inline] fn tag() -> u8 { 0x09 }
    #[inline] fn is_bare() -> bool { true }
}

impl<S, T> NbtFmt for HashMap<S, T> where S: AsRef<str> + Hash + Eq, T: NbtFmt {
    type Into = HashMap<String, T::Into>;

    #[inline]
    fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
       where W: io::Write
    {
        raw::write_bare_compound(dst, self.iter())
    }

    #[inline]
    fn read_bare_nbt<R>(src: &mut R) -> Result<Self::Into>
       where R: io::Read
    {
        let mut rval = HashMap::new();

        loop {
            let (tag, key) = try!(emit_next_header(src));

            if tag == 0x00 { break; } // i.e. Tag_End
            if tag != T::tag() {
                return Err(Error::TagMismatch(T::tag(), tag));
            }

            let value = try!(T::read_bare_nbt(src));

            // Check for key collisions.
            match rval.insert(key.clone(), value) {
                None    => (),
                Some(_) => return Err(Error::UnexpectedField(key)),
            };
        }

        Ok(rval)
    }

    #[inline] fn tag() -> u8 { 0x0a }
    #[inline] fn is_bare() -> bool { false }
}

impl<S, T> NbtFmt for BTreeMap<S, T> where S: AsRef<str>, T: NbtFmt {
    type Into = BTreeMap<String, T::Into>;

    #[inline]
    fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
       where W: io::Write
    {
        raw::write_bare_compound(dst, self.iter())
    }

    #[inline]
    fn read_bare_nbt<R>(src: &mut R) -> Result<Self::Into>
       where R: io::Read
    {
        let mut rval = BTreeMap::new();

        loop {
            let (tag, key) = try!(emit_next_header(src));

            if tag == 0x00 { break; } // i.e. Tag_End
            if tag != T::tag() {
                return Err(Error::TagMismatch(T::tag(), tag));
            }

            let value = try!(T::read_bare_nbt(src));

            // Check for key collisions.
            match rval.insert(key.clone(), value) {
                None    => (),
                Some(_) => return Err(Error::UnexpectedField(key)),
            };
        }

        Ok(rval)
    }

    #[inline] fn tag() -> u8 { 0x0a }
    #[inline] fn is_bare() -> bool { false }
}
