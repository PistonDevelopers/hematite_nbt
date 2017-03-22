use std::io;

use serde;
use serde::ser;

use error::{Error, Result};

pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W> where W: io::Write {
    pub fn new(writer: W) -> Self {
        Serializer { writer: writer }
    }

    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

pub struct Compound<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
}

impl<'a, W> ser::SerializeSeq for Compound<'a, W>
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

impl<'a, W> ser::SerializeTuple for Compound<'a, W>
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

impl<'a, W> ser::SerializeTupleStruct for Compound<'a, W>
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

impl<'a, W> ser::SerializeTupleVariant for Compound<'a, W>
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

impl<'a, W> ser::SerializeMap for Compound<'a, W>
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

impl<'a, W> ser::SerializeStruct for Compound<'a, W>
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

impl<'a, W> ser::SerializeStructVariant for Compound<'a, W>
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

impl<'a, W> serde::Serializer for &'a mut Serializer<W> where W: io::Write {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
        where T: ser::Serialize
    {
        unimplemented!()
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn serialize_unit_variant(self, name: &'static str, variant_index: usize,
                              variant: &'static str) -> Result<()>
    {
        unimplemented!()
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T)
                                           -> Result<()>
        where T: ser::Serialize
    {
        unimplemented!()
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
        unimplemented!()
    }

    #[inline]
    fn serialize_seq_fixed_size(self, size: usize) -> Result<Self::SerializeSeq>
    {
        unimplemented!()
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
    fn serialize_struct(self, name: &'static str, len: usize)
                        -> Result<Self::SerializeStruct>
    {
        unimplemented!()
    }

    #[inline]
    fn serialize_struct_variant(self, name: &'static str, variant_index: usize,
                                variant: &'static str, len: usize)
                                -> Result<Self::SerializeStructVariant>
    {
        unimplemented!()
    }
}
