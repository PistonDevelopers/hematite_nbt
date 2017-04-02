use std::io;

use serde::de;

use nbt::serialize::{emit_next_header, raw};

use error::{Error, Result};

/// Decode an object from Named Binary Tag (NBT) format.
///
/// Note that only maps and structs can be decoded, because the NBT format does
/// not support bare types. Other types will return `Error::NoRootCompound`.
pub fn from_reader<R, T>(src: R) -> Result<T>
    where R: io::Read,
          T: de::Deserialize,
{
    let mut decoder = Decoder::new(src);
    de::Deserialize::deserialize(&mut decoder)
}

/// Decode objects from Named Binary Tag (NBT) format.
///
/// Note that only maps and structs can be decoded, because the NBT format does
/// not support bare types. Other types will return `Error::NoRootCompound`.
pub struct Decoder<R> {
    reader: R,
}

impl<R> Decoder<R> where R: io::Read {

    /// Create an NBT Decoder from a given `io::Read` source.
    pub fn new(src: R) -> Self {
        Decoder { reader: src }
    }
}

impl<'a, R: io::Read> de::Deserializer for &'a mut Decoder<R> {
    type Error = Error;

    fn deserialize<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        // The decoder cannot deserialize types by default. It can only handle
        // maps and structs.
        Err(Error::NoRootCompound)
    }

    fn deserialize_struct<V>(self, _name: &'static str,
                             _fields: &'static [&'static str], visitor: V)
                             -> Result<V::Value>
        where V: de::Visitor
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V)
                                  -> Result<V::Value>
        where V: de::Visitor
    {
        visitor.visit_unit()
    }

    /// Deserialize newtype structs by their underlying types.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V)
                                     -> Result<V::Value>
        where V: de::Visitor
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        // Ignore the header (if there is one).
        let (tag, _) = try!(emit_next_header(&mut self.reader));

        match tag {
            0x0a => visitor.visit_map(MapDecoder::new(self)),
            _ => Err(Error::NoRootCompound)
        }
    }

    forward_to_deserialize! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char
        str string bytes byte_buf unit seq seq_fixed_size
        tuple_struct struct_field tuple option enum
        ignored_any
    }
}

/// Decoder for map-like types.
struct MapDecoder<'a, R: io::Read + 'a> {
    outer: &'a mut Decoder<R>,
    tag: Option<u8>,
}

impl<'a, R> MapDecoder<'a, R> where R: io::Read {

    fn new(outer: &'a mut Decoder<R>) -> Self {
        MapDecoder { outer: outer, tag: None }
    }
}

impl<'a, R: io::Read + 'a> de::MapVisitor for MapDecoder<'a, R> {
    type Error = Error;

    fn visit_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
        where K: de::DeserializeSeed
    {
        let tag = try!(raw::read_bare_byte(&mut self.outer.reader));

        // NBT indicates the end of a compound type with a 0x00 tag.
        if tag == 0x00 {
            return Ok(None);
        }

        // Keep track of the tag so that we can decode the field correctly.
        self.tag = Some(tag as u8);

        // TODO: Enforce that keys must be String. This is a bit of a hack.
        let mut de = InnerDecoder { outer: self.outer, tag: 0x08 };

        Ok(Some(seed.deserialize(&mut de)?))
    }

    fn visit_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
        where V: de::DeserializeSeed
    {
        let mut de = match self.tag {
            Some(tag) => InnerDecoder { outer: self.outer, tag: tag },
            None => unimplemented!(),
        };
        Ok(seed.deserialize(&mut de)?)
    }
}

/// Private inner decoder, for decoding raw (i.e. non-Compound) types.
struct InnerDecoder<'a, R: io::Read + 'a> {
    outer: &'a mut Decoder<R>,
    tag: u8,
}

impl<'a, 'b: 'a, R: io::Read> de::Deserializer for &'b mut InnerDecoder<'a, R> {
    type Error = Error;

    fn deserialize<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        let ref mut outer = self.outer;

        match self.tag {
            0x01 => visitor.visit_i8(raw::read_bare_byte(&mut outer.reader)?),
            0x02 => visitor.visit_i16(raw::read_bare_short(&mut outer.reader)?),
            0x03 => visitor.visit_i32(raw::read_bare_int(&mut outer.reader)?),
            0x04 => visitor.visit_i64(raw::read_bare_long(&mut outer.reader)?),
            0x05 => visitor.visit_f32(raw::read_bare_float(&mut outer.reader)?),
            0x06 => visitor.visit_f64(raw::read_bare_double(&mut outer.reader)?),
            0x07 => unimplemented!(), // Byte array.
            0x08 => visitor.visit_string(raw::read_bare_string(&mut outer.reader)?),
            0x09 => unimplemented!(), // List.
            0x0a => visitor.visit_map(MapDecoder::new(outer)),
            0x0b => unimplemented!(), // Int array.
            _    => unimplemented!(),
        }
    }

    /// Deserialize bool values from a byte. Fail if that byte is not 0 or 1.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        match self.tag {
            0x01 => {
                let ref mut reader = self.outer.reader;
                let value = try!(raw::read_bare_byte(reader));
                match value {
                    0 => visitor.visit_bool(false),
                    1 => visitor.visit_bool(true),
                    // FIXME: Handle non-conforming bool values.
                    _ => unimplemented!(),
                }
            },
            // FIXME: Handle mismatched tags.
            _ => unimplemented!(),
        }
    }

    /// Interpret missing values as None.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V)
                                  -> Result<V::Value>
        where V: de::Visitor
    {
        visitor.visit_unit()
    }

    /// Deserialize newtype structs by their underlying types.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V)
                                     -> Result<V::Value>
        where V: de::Visitor
    {
        visitor.visit_newtype_struct(self)
    }

    forward_to_deserialize! {
        u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char
        str string bytes byte_buf seq seq_fixed_size map
        tuple_struct struct struct_field tuple enum
        ignored_any
    }
}
