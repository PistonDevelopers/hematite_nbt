//! Contains functions for serializing arbitrary objects to the Named Binary
//! Tag format.
//!
//! For working with existing serialization implementations, see `to_writer`.
//! For custom types, implement the `NbtFmt` trait.

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::io;

use byteorder::{ByteOrder, BigEndian, WriteBytesExt};

use error::{Error, Result};

/// A trait indicating that the type has a Named Binary Tag representation.
///
/// Keep in mind that not all Rust types (notably unsigned integers) have an
/// obvious NBT representation, and so structs that implement this trait may
/// have to convert them to one that does.
///
/// ## Serialization
///
/// To enable NBT serialization of a type, implement the `to_bare_nbt` method.
/// Many basic types (`i8`, `i16`, `String`, etc.) implement `NbtFmt`, enabling
/// the use of `to_nbt` when writing this implementation. The `close_nbt`
/// function is also provided to aid in implementing custom serialization,
/// given that types will likely advertise themselves as Compound-like. For
/// example:
///
/// ```rust
/// extern crate nbt;
///
/// use nbt::serialize::{NbtFmt, to_writer, close_nbt};
///
/// struct MyMob {
///     name: String,
///     health: i8
/// }
///
/// impl NbtFmt for MyMob {
///     fn to_bare_nbt<W>(&self, dst: &mut W) -> nbt::Result<()>
///        where W: std::io::Write
///     {
///         try!(self.name.to_nbt(dst, "name"));
///         try!(self.health.to_nbt(dst, "health"));
///
///         close_nbt(dst)
///     }
/// }
///
/// fn main() {
///     let mut bytes = Vec::new();
///     let mob = MyMob { name: "Dr. Evil".to_string(), health: 240 };
///
///     to_writer(&mut bytes, mob).unwrap();
/// }
/// ```
///
pub trait NbtFmt {

    /// Convert this type to NBT format using the specified `io::Write`
    /// destination, but does not serialize its identifying NBT tag or name.
    fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
       where W: io::Write;

    /// Convert this type to NBT format using the specified `io::Write`
    /// destination, incuding its tag and a given name.
    #[inline]
    fn to_nbt<W, S>(&self, dst: &mut W, name: S) -> Result<()>
       where W: io::Write,
             S: AsRef<str>
    {
        try!(dst.write_u8(Self::tag()));
        try!(write_bare_string(dst, name.as_ref()));
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

/// Serializes an object into NBT format at a given destination.
///
/// This function will try to ensure that the output is always a valid NBT
/// file, i.e. that it has a top-level Compound.
pub fn to_writer<W, T>(dst: &mut W, obj: T) -> Result<()>
    where W: io::Write,
          T: NbtFmt
{
    match T::is_bare() {
        // Refuse to blindly serialize types not wrapped in an NBT Compound.
        true  => { return Err(Error::NoRootCompound); },
        false => obj.to_nbt(dst, ""),
    }
}

macro_rules! nbtfmt_value {
  ($T:ty, $method:ident, $tag:expr) => (
    impl NbtFmt for $T {
        #[inline]
        fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
           where W: io::Write
        {
            $method(dst, *self)
        }

        #[inline] fn tag() -> u8 { $tag }
        #[inline] fn is_bare() -> bool { true }
    }
  )
}

macro_rules! nbtfmt_ptr {
  ($T:ty, $method:ident, $tag:expr) => (
    impl NbtFmt for $T {
        #[inline]
        fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
           where W: io::Write
        {
            $method(dst, self)
        }

        #[inline] fn tag() -> u8 { $tag }
        #[inline] fn is_bare() -> bool { true }
    }
  )
}

macro_rules! nbtfmt_slice {
  ($T:ty, $method:ident, $tag:expr) => (
    impl NbtFmt for $T {
        #[inline]
        fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
           where W: io::Write
        {
            $method(dst, &self[..])
        }

        #[inline] fn tag() -> u8 { $tag }
        #[inline] fn is_bare() -> bool { true }
    }
  )
}

nbtfmt_value!(i8, write_bare_byte, 0x01);
nbtfmt_value!(i16, write_bare_short, 0x02);
nbtfmt_value!(i32, write_bare_int, 0x03);
nbtfmt_value!(i64, write_bare_long, 0x04);
nbtfmt_value!(f32, write_bare_float, 0x05);
nbtfmt_value!(f64, write_bare_double, 0x06);
nbtfmt_ptr!(str, write_bare_string, 0x08);
nbtfmt_slice!(String, write_bare_string, 0x08);

// For now, to handle conflicting implementations, use slices to indicate when
// byte and int arrays should be preferred to lists.

nbtfmt_ptr!([i8], write_bare_byte_array, 0x07);
nbtfmt_ptr!([i32], write_bare_int_array, 0x0b);

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

impl<T> NbtFmt for Vec<T> where T: NbtFmt {
    #[inline]
    fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
       where W: io::Write
    {
           write_bare_list(dst, self.iter())
    }

    #[inline] fn tag() -> u8 { 0x09 }
    #[inline] fn is_bare() -> bool { true }
}

impl<S, T> NbtFmt for HashMap<S, T> where S: AsRef<str> + Hash + Eq, T: NbtFmt {
    #[inline]
    fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
       where W: io::Write
    {
        write_bare_compound(dst, self.iter())
    }

    #[inline] fn tag() -> u8 { 0x0a }
    #[inline] fn is_bare() -> bool { false }
}

impl<S, T> NbtFmt for BTreeMap<S, T> where S: AsRef<str>, T: NbtFmt {
    #[inline]
    fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
       where W: io::Write
    {
        write_bare_compound(dst, self.iter())
    }

    #[inline] fn tag() -> u8 { 0x0a }
    #[inline] fn is_bare() -> bool { false }
}

#[inline]
fn write_bare_byte<W>(dst: &mut W, value: i8) -> Result<()>
   where W: io::Write {

    dst.write_i8(value).map_err(From::from)
}

#[inline]
fn write_bare_short<W>(dst: &mut W, value: i16) -> Result<()>
   where W: io::Write {

    dst.write_i16::<BigEndian>(value).map_err(From::from)
}

#[inline]
fn write_bare_int<W>(dst: &mut W, value: i32) -> Result<()>
   where W: io::Write {

    dst.write_i32::<BigEndian>(value).map_err(From::from)
}

#[inline]
fn write_bare_long<W>(dst: &mut W, value: i64) -> Result<()>
   where W: io::Write {

    dst.write_i64::<BigEndian>(value).map_err(From::from)
}

#[inline]
fn write_bare_float<W>(dst: &mut W, value: f32) -> Result<()>
   where W: io::Write {

    dst.write_f32::<BigEndian>(value).map_err(From::from)
}

#[inline]
fn write_bare_double<W>(dst: &mut W, value: f64) -> Result<()>
   where W: io::Write {

    dst.write_f64::<BigEndian>(value).map_err(From::from)
}

#[inline]
fn write_bare_byte_array<W>(dst: &mut W, value: &[i8]) -> Result<()>
   where W: io::Write {

    try!(dst.write_i32::<BigEndian>(value.len() as i32));
    for &v in value {
        try!(dst.write_i8(v));
    }
    Ok(())
}

#[inline]
fn write_bare_int_array<W>(dst: &mut W, value: &[i32]) -> Result<()>
   where W: io::Write {

    try!(dst.write_i32::<BigEndian>(value.len() as i32));
    for &v in value {
        try!(dst.write_i32::<BigEndian>(v));
    }
    Ok(())
}

#[inline]
fn write_bare_string<W>(dst: &mut W, value: &str) -> Result<()>
   where W: io::Write {
    
    try!(dst.write_u16::<BigEndian>(value.len() as u16));
    dst.write_all(value.as_bytes()).map_err(From::from)
}

#[inline]
fn write_bare_list<'a, W, I, T>(dst: &mut W, values: I) -> Result<()>
   where W: io::Write,
         I: Iterator<Item=&'a T> + ExactSizeIterator,
         T: 'a + NbtFmt {

    // The list contents are prefixed by a byte tag for the type and the
    // length of the list (a big-endian i32).
    try!(dst.write_u8(T::tag()));
    try!(dst.write_i32::<BigEndian>(values.len() as i32));

    for ref value in values {
        // Note the use of bare values.
        try!(value.to_bare_nbt(dst));
    }

    Ok(())
}

#[inline]
fn write_bare_compound<'a, W, I, T, S>(dst: &mut W, values: I) -> Result<()>
   where W: io::Write,
         I: Iterator<Item=(&'a S, &'a T)>,
         S: 'a + AsRef<str>,
         T: 'a + NbtFmt {

    for (key, ref value) in values {
        try!(value.to_nbt(dst, key.as_ref()));
    }
    
    // Write the marker for the end of the Compound.
    close_nbt(dst)
}

#[test]
fn serialize_basic_types() {
  struct TestStruct {
    name: String,
    health: i8,
    food: f32,
    emeralds: i16,
    timestamp: i32,
    ids: HashMap<String, i8>,
    data: Vec<i8>
  }

  impl NbtFmt for TestStruct {
    fn to_bare_nbt<W>(&self, dst: &mut W) -> Result<()>
           where W: io::Write {

            try!(self.name.to_nbt(dst, "name"));
            try!(self.health.to_nbt(dst, "health"));
            try!(self.food.to_nbt(dst, "food"));
            try!(self.emeralds.to_nbt(dst, "emeralds"));
            try!(self.timestamp.to_nbt(dst, "timestamp"));
            try!(self.ids.to_nbt(dst, "ids"));
            try!(self.data.to_nbt(dst, "data"));

            close_nbt(dst)
        }
  }

  let test = TestStruct {
    name: "Herobrine".to_string(),
    health: 100, food: 20.0, emeralds: 12345, timestamp: 1424778774,
    ids: HashMap::new(), data: vec![1, 2, 3]
  };

  let mut dst = Vec::new();
  to_writer(&mut dst, test).unwrap();

  let bytes = [
        0x0a,
            0x00, 0x00,
            0x08,
                0x00, 0x04,
                0x6e, 0x61, 0x6d, 0x65,
                0x00, 0x09,
                0x48, 0x65, 0x72, 0x6f, 0x62, 0x72, 0x69, 0x6e, 0x65,
            0x01,
                0x00, 0x06,
                0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
                0x64,
            0x05,
                0x00, 0x04,
                0x66, 0x6f, 0x6f, 0x64,
                0x41, 0xa0, 0x00, 0x00,
            0x02,
                0x00, 0x08,
                0x65, 0x6d, 0x65, 0x72, 0x61, 0x6c, 0x64, 0x73,
                0x30, 0x39,
            0x03,
                0x00, 0x09,
                0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
                0x54, 0xec, 0x66, 0x16,
            0x0a,
                0x00, 0x03,
                0x69, 0x64, 0x73,
                // No content.
            0x00,
            0x09,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x01, // List type.
                0x00, 0x00, 0x00, 0x03, // Length.
                0x01, 0x02, 0x03, // Content.
        0x00
    ];

    assert_eq!(&bytes[..], &dst[..]);
}
