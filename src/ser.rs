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

    /// Consume this encoder and return the underlying writer.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
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

#[derive(Clone)]
enum State<'a> {
    Named(&'a str),
    ListHead(usize),
    Bare,
}

/// "Inner" version of the NBT encoder, capable of serializing bare types.
struct InnerEncoder<'a, 'b: 'a, W: 'a> {
    outer: &'a mut Encoder<'b, W>,
    state: State<'a>,
}

impl<'a, 'b, W> InnerEncoder<'a, 'b, W> where W: io::Write {
    fn write_header(&mut self, tag: i8) -> Result<()> {
        match self.state {
            State::Bare          => Ok(()),
            State::Named(header) => self.outer.write_header(tag, Some(header)),
            State::ListHead(s)   => {
                try!(raw::write_bare_byte(&mut self.outer.writer, tag));
                try!(raw::write_bare_int(&mut self.outer.writer, s as i32));
                self.state = State::Bare;
                Ok(())
            }
        }
    }
}

#[doc(hidden)]
pub struct Compound<'a, 'b: 'a, W: 'a> {
    outer: &'a mut Encoder<'b, W>,
    state: State<'b>,
}

impl<'a, 'b, W> ser::SerializeSeq for Compound<'a, 'b, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where T: serde::Serialize
    {
        let mut ser = InnerEncoder {
            outer: self.outer,
            state: self.state.clone()
        };
        try!(value.serialize(&mut ser));
        self.state = State::Bare;
        Ok(())
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
        value.serialize(&mut InnerEncoder {
            outer: self.outer,
            state: State::Named(key)
        })
    }

    fn end(self) -> Result<()> {
        raw::close_nbt(&mut self.outer.writer).map_err(From::from)
    }
}

impl<'a, 'b, W> serde::Serializer for &'a mut Encoder<'b, W> where W: io::Write {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
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
    /// only be successful if the underyling type is a struct.
    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T)
                                           -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(self)
    }

    /// Arbitrary maps cannot be serialized, so calling this method will always
    /// return an error.
    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnrepresentableType("map"))
    }

    /// Serialize structs as `Tag_Compound` data.
    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize)
                        -> Result<Self::SerializeStruct>
    {
        let header = self.header; // Circumvent strange borrowing errors.
        try!(self.write_header(0x0a, header));
        Ok(Compound { outer: self, state: State::Bare })
    }
}

impl<'a, 'b, W> serde::Serializer for &'a mut InnerEncoder<'a, 'b, W> where W: io::Write {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, 'b, W>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = Compound<'a, 'b, W>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        self.serialize_i8(value as i8)
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        try!(self.write_header(0x01));
        raw::write_bare_byte(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        try!(self.write_header(0x02));
        raw::write_bare_short(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        try!(self.write_header(0x03));
        raw::write_bare_int(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        try!(self.write_header(0x04));
        raw::write_bare_long(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_u8(self, _value: u8) -> Result<()> {
        Err(Error::UnrepresentableType("u8"))
    }

    #[inline]
    fn serialize_u16(self, _value: u16) -> Result<()> {
        Err(Error::UnrepresentableType("u16"))
    }

    #[inline]
    fn serialize_u32(self, _value: u32) -> Result<()> {
        Err(Error::UnrepresentableType("u32"))
    }

    #[inline]
    fn serialize_u64(self, _value: u64) -> Result<()> {
        Err(Error::UnrepresentableType("u64"))
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<()> {
        try!(self.write_header(0x05));
        raw::write_bare_float(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<()> {
        try!(self.write_header(0x06));
        raw::write_bare_double(&mut self.outer.writer, value)
            .map_err(From::from)
    }

    #[inline]
    fn serialize_char(self, _value: char) -> Result<()> {
        Err(Error::UnrepresentableType("char"))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        try!(self.write_header(0x08));
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
    fn serialize_unit(self) -> Result<()> {
        Err(Error::UnrepresentableType("unit"))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        try!(self.write_header(0x0a));
        raw::close_nbt(&mut self.outer.writer).map_err(From::from)
    }

    #[inline]
    fn serialize_unit_variant(self, _name: &'static str, _index: u32,
                              _variant: &'static str) -> Result<()>
    {
        Err(Error::UnrepresentableType("unit variant"))
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T)
                                           -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str,
                                            _index: u32,
                                            _variant: &'static str,
                                            _value: &T) -> Result<()>
        where T: ser::Serialize
    {
        Err(Error::UnrepresentableType("newtype variant"))
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(l) = len {
            try!(self.write_header(0x09));
            Ok(Compound { outer: self.outer, state: State::ListHead(l) })
        } else {
            Err(Error::UnrepresentableType("unsized list"))
        }
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::UnrepresentableType("tuple"))
    }

    #[inline]
    fn serialize_tuple_struct(self, _name: &'static str, _len: usize)
                              -> Result<Self::SerializeTupleStruct>
    {
        Err(Error::UnrepresentableType("tuple struct"))
    }

    #[inline]
    fn serialize_tuple_variant(self, _name: &'static str, _index: u32,
                               _variant: &'static str, _len: usize)
                               -> Result<Self::SerializeTupleVariant>
    {
        Err(Error::UnrepresentableType("tuple variant"))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnrepresentableType("map"))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize)
                        -> Result<Self::SerializeStruct>
    {
        try!(self.write_header(0x0a));
        Ok(Compound { outer: self.outer, state: State::Bare })
    }

    #[inline]
    fn serialize_struct_variant(self, _name: &'static str, _index: u32,
                                _variant: &'static str, _len: usize)
                                -> Result<Self::SerializeStructVariant>
    {
        Err(Error::UnrepresentableType("struct variant"))
    }
}
