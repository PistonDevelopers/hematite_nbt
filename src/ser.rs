//! Serialize a Rust data structure into Named Binary Tag data.

use std::io;

use serde;
use serde::ser;
use flate2::Compression;
use flate2::write::{GzEncoder, ZlibEncoder};

use raw;

use error::{Error, Result};

/// Encode `value` in Named Binary Tag format to the given `io::Write`
/// destination, with an optional header.
#[inline]
pub fn to_writer<'a, W, T>(dst: &mut W, value: &T, header: Option<&'a str>)
                           -> Result<()>
    where W: ?Sized + io::Write,
          T: ?Sized + ser::Serialize,
{
    let mut encoder = Encoder::new(dst, header);
    value.serialize(&mut encoder)
}

/// Encode `value` in Named Binary Tag format to the given `io::Write`
/// destination, with an optional header.
pub fn to_gzip_writer<'a, W, T>(dst: &mut W, value: &T, header: Option<&'a str>)
                           -> Result<()>
    where W: ?Sized + io::Write,
          T: ?Sized + ser::Serialize,
{
    let mut encoder = Encoder::new(GzEncoder::new(dst, Compression::Default), header);
    value.serialize(&mut encoder)
}

/// Encode `value` in Named Binary Tag format to the given `io::Write`
/// destination, with an optional header.
pub fn to_zlib_writer<'a, W, T>(dst: &mut W, value: &T, header: Option<&'a str>)
                           -> Result<()>
    where W: ?Sized + io::Write,
          T: ?Sized + ser::Serialize,
{
    let mut encoder = Encoder::new(ZlibEncoder::new(dst, Compression::Default), header);
    value.serialize(&mut encoder)
}

/// Encode objects to Named Binary Tag format.
///
/// This structure can be used to serialize objects which implement the
/// `serde::Serialize` trait into NBT format. Note that not all types are
/// representable in NBT format (notably unsigned integers), so this encoder may
/// return errors.
pub struct Encoder<'a, W> {
    writer: W,
    header: Option<&'a str>,
}

impl<'a, W> Encoder<'a, W> where W: io::Write {

    /// Create an encoder with optional `header` from a given Writer.
    pub fn new(writer: W, header: Option<&'a str>) -> Self {
        Encoder { writer: writer, header: header }
    }

    /// Write the NBT tag and an optional header to the underlying writer.
    #[inline]
    fn write_header(&mut self, tag: i8, header: Option<&str>) -> Result<()> {
        try!(raw::write_bare_byte(&mut self.writer, tag));
        match header {
            None =>
                raw::write_bare_short(&mut self.writer, 0).map_err(From::from),
            Some(h) =>
                raw::write_bare_string(&mut self.writer, h).map_err(From::from),
        }
    }
}

/// "Inner" version of the NBT encoder, capable of serializing bare types.
struct InnerEncoder<'a, 'b: 'a, W: 'a> {
    outer: &'a mut Encoder<'b, W>,
}

impl<'a, 'b, W> InnerEncoder<'a, 'b, W> where W: io::Write {
    pub fn from_outer(outer: &'a mut Encoder<'b, W>) -> Self {
        InnerEncoder { outer: outer }
    }
}

#[doc(hidden)]
pub struct Compound<'a, 'b: 'a, W: 'a> {
    outer: &'a mut Encoder<'b, W>,
    length: i32,
    sigil: bool,
}

impl<'a, 'b, W> Compound<'a, 'b, W> where W: io::Write {
    fn from_outer(outer: &'a mut Encoder<'b, W>) -> Self {
        Compound { outer: outer, length: 0, sigil: false }
    }

    fn for_seq(outer: &'a mut Encoder<'b, W>, length: i32) -> Result<Self> {
        // For an empty list, write TAG_End as the tag type.
        if length == 0 {
            raw::write_bare_byte(&mut outer.writer, 0x00)?;
            raw::write_bare_int(&mut outer.writer, 0)?;
        }
        Ok(Compound { outer: outer, length: length, sigil: false })
    }
}

impl<'a, 'b, W> ser::SerializeSeq for Compound<'a, 'b, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where T: serde::Serialize
    {
        if !self.sigil {
            value.serialize(&mut TagEncoder::from_outer(self.outer, Option::<&String>::None, false))?;
            raw::write_bare_int(&mut self.outer.writer, self.length)?;
            self.sigil = true;
        }
        value.serialize(&mut InnerEncoder::from_outer(self.outer))
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b, W> ser::SerializeStruct for Compound<'a, 'b, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T)
                                  -> Result<()>
        where T: serde::Serialize
    {
        value.serialize(&mut TagEncoder::from_outer(self.outer, Some(&key), false))?;
        value.serialize(&mut InnerEncoder::from_outer(self.outer))
    }

    fn end(self) -> Result<()> {
        raw::close_nbt(&mut self.outer.writer)
    }
}

impl<'a, 'b, W> ser::SerializeMap for Compound<'a, 'b, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
        where T: serde::Serialize
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
        where T: serde::Serialize
    {
        unimplemented!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<()>
        where K: serde::Serialize,
              V: serde::Serialize,
    {
        value.serialize(&mut TagEncoder::from_outer(self.outer, Some(&key), false))?;
        value.serialize(&mut InnerEncoder::from_outer(self.outer))
    }

    fn end(self) -> Result<()> {
        raw::close_nbt(&mut self.outer.writer)
    }
}

impl<'a, 'b, W> serde::Serializer for &'a mut Encoder<'b, W> where W: io::Write {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = Compound<'a, 'b, W>;
    type SerializeStruct = Compound<'a, 'b, W>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    return_expr_for_serialized_types!(
        Err(Error::NoRootCompound); bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
            char str bytes none some unit unit_variant newtype_variant
            seq tuple tuple_struct tuple_variant struct_variant
    );

    /// Serialize unit structs as empty `Tag_Compound` data.
    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        let header = self.header; // Circumvent strange borrowing errors.
        try!(self.write_header(0x0a, header));
        raw::close_nbt(&mut self.writer).map_err(From::from)
    }

    /// Serialize newtype structs by their underlying type. Note that this will
    /// only be successful if the underyling type is a struct or a map.
    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T)
                                           -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(self)
    }

    /// Serialize maps as `Tag_Compound` data.
    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        let header = self.header; // Circumvent strange borrowing errors.
        try!(self.write_header(0x0a, header));
        Ok(Compound::from_outer(self))
    }

    /// Serialize structs as `Tag_Compound` data.
    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize)
                        -> Result<Self::SerializeStruct>
    {
        let header = self.header; // Circumvent strange borrowing errors.
        try!(self.write_header(0x0a, header));
        Ok(Compound::from_outer(self))
    }
}

impl<'a, 'b, W> serde::Serializer for &'a mut InnerEncoder<'a, 'b, W> where W: io::Write {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, 'b, W>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = Compound<'a, 'b, W>;
    type SerializeStruct = Compound<'a, 'b, W>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    unrepresentable!(
        u8 u16 u32 u64 char unit unit_variant newtype_variant tuple tuple_struct
            tuple_variant struct_variant
    );

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        self.serialize_i8(value as i8)
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        raw::write_bare_byte(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        raw::write_bare_short(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        raw::write_bare_int(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        raw::write_bare_long(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<()> {
        raw::write_bare_float(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<()> {
        raw::write_bare_double(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        raw::write_bare_string(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(Error::UnrepresentableType("u8"))
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        raw::close_nbt(&mut self.outer.writer).map_err(From::from)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T)
                                           -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(l) = len {
            Compound::for_seq(self.outer, l as i32)
        } else {
            Err(Error::UnrepresentableType("unsized list"))
        }
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Compound::from_outer(self.outer))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize)
                        -> Result<Self::SerializeStruct>
    {
        Ok(Compound::from_outer(self.outer))
    }
}

/// A serializer for valid tag names, i.e. strings.
struct TagNameEncoder<'a, 'b: 'a, W: 'a> {
    outer: &'a mut Encoder<'b, W>,
}

impl<'a, 'b: 'a, W: 'a> TagNameEncoder<'a, 'b, W> where W: io::Write {
    pub fn from_outer(outer: &'a mut Encoder<'b, W>) -> Self {
        TagNameEncoder { outer: outer }
    }
}

impl<'a, 'b: 'a, W: 'a> serde::Serializer for &'a mut TagNameEncoder<'a, 'b, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    return_expr_for_serialized_types!(
        Err(Error::NonStringMapKey); bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
            char bytes unit unit_variant newtype_variant unit_struct seq tuple
            tuple_struct tuple_variant struct_variant newtype_struct map struct
    );

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where T: ser::Serialize
    {
        value.serialize(self)
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        raw::write_bare_string(&mut self.outer.writer, value)
    }
}

/// A serializer for valid tags.
struct TagEncoder<'a, 'b: 'a, W: 'a, K> {
    outer: &'a mut Encoder<'b, W>,
    key: Option<&'a K>,
    sequence_header: bool,
}

impl<'a, 'b: 'a, W: 'a, K> TagEncoder<'a, 'b, W, K>
where W: io::Write,
      K: serde::Serialize
{
    fn from_outer(outer: &'a mut Encoder<'b, W>, key: Option<&'a K>, sequence_header: bool) -> Self {
        TagEncoder {
            outer,
            key,
            sequence_header,
        }
    }

    fn write_header(&mut self, tag: i8) -> Result<()> {
        use serde::Serialize;

        if self.key.is_none() && matches!(tag, 0x01 | 0x03 | 0x04) {
            // Don't write headers in byte/integer/long arrays
            return Ok(());
        }

        let tag = if self.sequence_header {
            match tag {
                0x01 => 0x07, // Byte array
                0x03 => 0x0b, // Integer array
                0x04 => 0x0c, // Long array
                _ => 0x09, // List
            }
        } else {
            tag
        };

        raw::write_bare_byte(&mut self.outer.writer, tag)?;
        self.key.serialize(&mut TagNameEncoder::from_outer(self.outer))?;

        Ok(())
    }
}

impl<'a, 'b: 'a, W: 'a, K> serde::Serializer for &'a mut TagEncoder<'a, 'b, W, K>
where W: io::Write,
      K: serde::Serialize
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SeqHeader<'a, 'b, W, K>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = NoOp;
    type SerializeStruct = NoOp;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    unrepresentable!(
        u8 u16 u32 u64 char unit unit_variant newtype_variant tuple tuple_struct
            tuple_variant struct_variant
    );

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        self.serialize_i8(value as i8)
    }

    #[inline]
    fn serialize_i8(self, _value: i8) -> Result<()> {
        self.write_header(0x01)
    }

    #[inline]
    fn serialize_i16(self, _value: i16) -> Result<()> {
        self.write_header(0x02)
    }

    #[inline]
    fn serialize_i32(self, _value: i32) -> Result<()> {
        self.write_header(0x03)
    }

    #[inline]
    fn serialize_i64(self, _value: i64) -> Result<()> {
        self.write_header(0x04)
    }

    #[inline]
    fn serialize_f32(self, _value: f32) -> Result<()> {
        self.write_header(0x05)
    }

    #[inline]
    fn serialize_f64(self, _value: f64) -> Result<()> {
        self.write_header(0x06)
    }

    #[inline]
    fn serialize_str(self, _value: &str) -> Result<()> {
        self.write_header(0x08)
    }

    #[inline]
    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(Error::UnrepresentableType("u8"))
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.write_header(0x0a)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T)
                                           -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(len) = len {
            if self.sequence_header {
                // List in a List
                self.write_header(0x09)?;
                self.key = None;
            }

            let written = if len == 0 {
                self.write_header(0x09)?;

                true
            } else {
                false
            };

            Ok(SeqHeader {
                outer: self.outer,
                key: self.key,
                written,
            })
        } else {
            Err(Error::UnrepresentableType("unsized list"))
        }
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.write_header(0x0a)?;
        Ok(NoOp)
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize)
                        -> Result<Self::SerializeStruct>
    {
        self.write_header(0x0a)?;
        Ok(NoOp)
    }
}

/// This serializer writes the tag (list/array tag, (key)) for non-empty sequences.
/// This infers the sequence element type from the first element written to the sequence
/// to serialize i8, i32 or i64 sequences as ByteArray, IntegerArray or LongArray
/// NBT types respectively.
/// For every other sequence element type a normal NBT List tag is written.
struct SeqHeader<'a, 'b: 'a, W: 'a, K> {
    outer: &'a mut Encoder<'b, W>,
    key: Option<&'a K>,
    written: bool,
}

impl<'a, 'b: 'a, W: 'a, K> ser::SerializeSeq for SeqHeader<'a, 'b, W, K>
    where
        W: io::Write,
        K: serde::Serialize,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where
            T: serde::Serialize,
    {
        if !self.written {
            // Write type-specific sequence header
            let mut encoder = TagEncoder::from_outer(self.outer, self.key, true);
            value.serialize(&mut encoder)?;

            // Make sure we don't write the header again
            self.written = true;
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

/// This empty serializer provides a way to serialize only headers/tags for
/// sequences, maps, and structs.
struct NoOp;

impl ser::SerializeStruct for NoOp {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T)
                                  -> Result<()>
        where T: serde::Serialize
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeMap for NoOp {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
        where T: serde::Serialize
    {
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
        where T: serde::Serialize
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
