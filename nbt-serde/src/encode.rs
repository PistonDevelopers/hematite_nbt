use std::io;

use serde;
use serde::ser;

use nbt::serialize::close_nbt;
use nbt::serialize::raw;

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

impl<'a, 'b, W> ser::SerializeTuple for Compound<'a, 'b, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where T: serde::Serialize
    {
        unimplemented!();
    }

    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a, 'b, W> ser::SerializeTupleStruct for Compound<'a, 'b, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where T: serde::Serialize
    {
        unimplemented!();
    }

    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a, 'b, W> ser::SerializeTupleVariant for Compound<'a, 'b, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where T: serde::Serialize
    {
        unimplemented!();
    }

    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a, 'b, W> ser::SerializeMap for Compound<'a, 'b, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
        where T: serde::Serialize
    {
        unimplemented!();
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where T: serde::Serialize
    {
        unimplemented!();
    }

    fn end(self) -> Result<()> {
        unimplemented!();
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
        close_nbt(&mut self.outer.writer).map_err(From::from)
    }
}

impl<'a, 'b, W> ser::SerializeStructVariant for Compound<'a, 'b, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T)
                                  -> Result<()>
        where T: serde::Serialize
    {
        unimplemented!();
    }

    fn end(self) -> Result<()> {
        unimplemented!();
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
    type SerializeStructVariant = Compound<'a, 'b, W>;

    #[inline]
    fn serialize_bool(self, _: bool) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_i8(self, _: i8) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_i16(self, _: i16) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_i32(self, _: i32) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_i64(self, _: i64) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_u8(self, _: u8) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_u16(self, _: u16) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_u32(self, _: u32) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_u64(self, _: u64) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_f32(self, _: f32) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_f64(self, _: f64) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_char(self, _: char) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_str(self, _: &str) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_bytes(self, _: &[u8]) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<()>
        where T: ser::Serialize
    {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        let header = self.header; // Circumvent strange borrowing errors.
        try!(self.write_header(0x0a, header));
        close_nbt(&mut self.writer).map_err(From::from)
    }

    #[inline]
    fn serialize_unit_variant(self, _: &'static str, _: usize,
                              _: &'static str) -> Result<()>
    {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, value: &T)
                                           -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str,
                                            variant_index: usize,
                                            variant: &'static str,
                                            value: &T) -> Result<()>
        where T: ser::Serialize
    {
        unimplemented!()
    }

    #[inline]
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_seq_fixed_size(self, _: usize) -> Result<Self::SerializeSeq>
    {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_tuple_struct(self, _: &'static str, _: usize)
                              -> Result<Self::SerializeTupleStruct>
    {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_tuple_variant(self, _: &'static str, _: usize,
                               _: &'static str, _: usize)
                               -> Result<Self::SerializeTupleVariant>
    {
        Err(Error::NoRootCompound)
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    #[inline]
    fn serialize_struct(self, _: &'static str, _: usize)
                        -> Result<Self::SerializeStruct>
    {
        let header = self.header; // Circumvent strange borrowing errors.
        try!(self.write_header(0x0a, header));
        Ok(Compound { outer: self, state: State::Bare })
    }

    #[inline]
    fn serialize_struct_variant(self, name: &'static str, variant_index: usize,
                                variant: &'static str, len: usize)
                                -> Result<Self::SerializeStructVariant>
    {
        unimplemented!()
    }
}

impl<'a, 'b, W> serde::Serializer for &'a mut InnerEncoder<'a, 'b, W> where W: io::Write {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, 'b, W>;
    type SerializeTuple = Compound<'a, 'b, W>;
    type SerializeTupleStruct = Compound<'a, 'b, W>;
    type SerializeTupleVariant = Compound<'a, 'b, W>;
    type SerializeMap = Compound<'a, 'b, W>;
    type SerializeStruct = Compound<'a, 'b, W>;
    type SerializeStructVariant = Compound<'a, 'b, W>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_i8(v as i8)
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<()> {
        try!(self.write_header(0x01));
        raw::write_bare_byte(&mut self.outer.writer, v).map_err(From::from)
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<()> {
        try!(self.write_header(0x02));
        raw::write_bare_short(&mut self.outer.writer, v).map_err(From::from)
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<()> {
        try!(self.write_header(0x03));
        raw::write_bare_int(&mut self.outer.writer, v).map_err(From::from)
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<()> {
        try!(self.write_header(0x04));
        raw::write_bare_long(&mut self.outer.writer, v).map_err(From::from)
    }

    #[inline]
    fn serialize_u8(self, _: u8) -> Result<()> {
        Err(Error::UnrepresentableType("u8"))
    }

    #[inline]
    fn serialize_u16(self, _: u16) -> Result<()> {
        Err(Error::UnrepresentableType("u16"))
    }

    #[inline]
    fn serialize_u32(self, _: u32) -> Result<()> {
        Err(Error::UnrepresentableType("u32"))
    }

    #[inline]
    fn serialize_u64(self, _: u64) -> Result<()> {
        Err(Error::UnrepresentableType("u64"))
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<()> {
        try!(self.write_header(0x05));
        raw::write_bare_float(&mut self.outer.writer, v).map_err(From::from)
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<()> {
        try!(self.write_header(0x06));
        raw::write_bare_double(&mut self.outer.writer, v).map_err(From::from)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<()> {
        try!(self.write_header(0x08));
        raw::write_bare_string(&mut self.outer.writer, v).map_err(From::from)
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        unimplemented!()
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
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        try!(self.write_header(0x0a));
        close_nbt(&mut self.outer.writer).map_err(From::from)
    }

    #[inline]
    fn serialize_unit_variant(self, name: &'static str, variant_index: usize,
                              variant: &'static str) -> Result<()>
    {
        unimplemented!()
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, value: &T)
                                           -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str,
                                            variant_index: usize,
                                            variant: &'static str,
                                            value: &T) -> Result<()>
        where T: ser::Serialize
    {
        unimplemented!()
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
    fn serialize_seq_fixed_size(self, len: usize) -> Result<Self::SerializeSeq>
    {
        try!(self.write_header(0x09));
        Ok(Compound { outer: self.outer, state: State::ListHead(len) })
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    #[inline]
    fn serialize_tuple_struct(self, name: &'static str, len: usize)
                              -> Result<Self::SerializeTupleStruct>
    {
        unimplemented!()
    }

    #[inline]
    fn serialize_tuple_variant(self, name: &'static str, variant_index: usize,
                               variant: &'static str, len: usize)
                               -> Result<Self::SerializeTupleVariant>
    {
        unimplemented!()
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    #[inline]
    fn serialize_struct(self, _: &'static str, _: usize)
                        -> Result<Self::SerializeStruct>
    {
        try!(self.write_header(0x0a));
        Ok(Compound { outer: self.outer, state: State::Bare })
    }

    #[inline]
    fn serialize_struct_variant(self, name: &'static str, variant_index: usize,
                                variant: &'static str, len: usize)
                                -> Result<Self::SerializeStructVariant>
    {
        unimplemented!()
    }
}
