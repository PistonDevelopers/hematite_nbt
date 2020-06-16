//! Deserialize Named Binary Tag data to a Rust data structure.

use std::io;

use flate2::read;
use serde::de;

use raw;

use error::{Error, Result};

/// Decode an object from Named Binary Tag (NBT) format.
///
/// Note that only maps and structs can be decoded, because the NBT format does
/// not support bare types. Other types will return `Error::NoRootCompound`.
pub fn from_reader<R, T>(src: R) -> Result<T>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let mut decoder = Decoder::new(src);
    de::Deserialize::deserialize(&mut decoder)
}

/// Decode an object from Named Binary Tag (NBT) format.
///
/// Note that only maps and structs can be decoded, because the NBT format does
/// not support bare types. Other types will return `Error::NoRootCompound`.
pub fn from_gzip_reader<R, T>(src: R) -> Result<T>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let gzip = read::GzDecoder::new(src)?;
    from_reader(gzip)
}

/// Decode an object from Named Binary Tag (NBT) format.
///
/// Note that only maps and structs can be decoded, because the NBT format does
/// not support bare types. Other types will return `Error::NoRootCompound`.
pub fn from_zlib_reader<R, T>(src: R) -> Result<T>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let zlib = read::ZlibDecoder::new(src);
    from_reader(zlib)
}

/// Decode objects from Named Binary Tag (NBT) format.
///
/// Note that only maps and structs can be decoded, because the NBT format does
/// not support bare types. Other types will return `Error::NoRootCompound`.
pub struct Decoder<R> {
    reader: R,
}

impl<R> Decoder<R>
where
    R: io::Read,
{
    /// Create an NBT Decoder from a given `io::Read` source.
    pub fn new(src: R) -> Self {
        Decoder { reader: src }
    }
}

impl<'de: 'a, 'a, R: io::Read> de::Deserializer<'de> for &'a mut Decoder<R> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // The decoder cannot deserialize types by default. It can only handle
        // maps and structs.
        Err(Error::NoRootCompound)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    /// Deserialize newtype structs by their underlying types.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // Ignore the header (if there is one).
        let (tag, _) = raw::emit_next_header(&mut self.reader)?;

        match tag {
            0x0a => visitor.visit_map(MapDecoder::new(self)),
            _ => Err(Error::NoRootCompound),
        }
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string bytes byte_buf
        unit seq tuple_struct tuple option enum identifier ignored_any
    }
}

/// Decoder for map-like types.
struct MapDecoder<'a, R: io::Read + 'a> {
    outer: &'a mut Decoder<R>,
    tag: Option<u8>,
}

impl<'a, R> MapDecoder<'a, R>
where
    R: io::Read,
{
    fn new(outer: &'a mut Decoder<R>) -> Self {
        MapDecoder { outer, tag: None }
    }
}

impl<'de: 'a, 'a, R: io::Read + 'a> de::MapAccess<'de> for MapDecoder<'a, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        let tag = raw::read_bare_byte(&mut self.outer.reader)?;

        // NBT indicates the end of a compound type with a 0x00 tag.
        if tag == 0x00 {
            return Ok(None);
        }

        // Keep track of the tag so that we can decode the field correctly.
        self.tag = Some(tag as u8);

        // TODO: Enforce that keys must be String. This is a bit of a hack.
        let mut de = InnerDecoder {
            outer: self.outer,
            tag: 0x08,
        };

        Ok(Some(seed.deserialize(&mut de)?))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let mut de = match self.tag {
            Some(tag) => InnerDecoder {
                outer: self.outer,
                tag,
            },
            None => unimplemented!(),
        };
        Ok(seed.deserialize(&mut de)?)
    }
}

/// Decoder for list-like types.
struct SeqDecoder<'a, R: io::Read + 'a> {
    outer: &'a mut Decoder<R>,
    tag: u8,
    length: i32,
    current: i32,
}

impl<'a, R> SeqDecoder<'a, R>
where
    R: io::Read,
{
    fn list(outer: &'a mut Decoder<R>) -> Result<Self> {
        let tag = raw::read_bare_byte(&mut outer.reader)?;
        let length = raw::read_bare_int(&mut outer.reader)?;
        Ok(SeqDecoder {
            outer,
            tag: tag as u8,
            length,
            current: 0,
        })
    }

    fn byte_array(outer: &'a mut Decoder<R>) -> Result<Self> {
        let length = raw::read_bare_int(&mut outer.reader)?;
        Ok(SeqDecoder {
            outer,
            tag: 0x01,
            length,
            current: 0,
        })
    }

    fn int_array(outer: &'a mut Decoder<R>) -> Result<Self> {
        let length = raw::read_bare_int(&mut outer.reader)?;
        Ok(SeqDecoder {
            outer,
            tag: 0x03,
            length,
            current: 0,
        })
    }

    fn long_array(outer: &'a mut Decoder<R>) -> Result<Self> {
        let length = raw::read_bare_int(&mut outer.reader)?;
        Ok(SeqDecoder {
            outer,
            tag: 0x04,
            length,
            current: 0,
        })
    }
}

impl<'de: 'a, 'a, R: io::Read + 'a> de::SeqAccess<'de> for SeqDecoder<'a, R> {
    type Error = Error;

    fn next_element_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.current == self.length {
            return Ok(None);
        }

        let mut de = InnerDecoder {
            outer: self.outer,
            tag: self.tag,
        };
        let value = seed.deserialize(&mut de)?;

        self.current += 1;

        Ok(Some(value))
    }

    /// We always know the length of an NBT list in advance.
    fn size_hint(&self) -> Option<usize> {
        Some(self.length as usize)
    }
}

/// Private inner decoder, for decoding raw (i.e. non-Compound) types.
struct InnerDecoder<'a, R: io::Read + 'a> {
    outer: &'a mut Decoder<R>,
    tag: u8,
}

impl<'a, 'b: 'a, 'de, R: io::Read> de::Deserializer<'de> for &'b mut InnerDecoder<'a, R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let outer = &mut self.outer;

        match self.tag {
            0x01 => visitor.visit_i8(raw::read_bare_byte(&mut outer.reader)?),
            0x02 => visitor.visit_i16(raw::read_bare_short(&mut outer.reader)?),
            0x03 => visitor.visit_i32(raw::read_bare_int(&mut outer.reader)?),
            0x04 => visitor.visit_i64(raw::read_bare_long(&mut outer.reader)?),
            0x05 => visitor.visit_f32(raw::read_bare_float(&mut outer.reader)?),
            0x06 => visitor.visit_f64(raw::read_bare_double(&mut outer.reader)?),
            0x07 => visitor.visit_seq(SeqDecoder::byte_array(outer)?),
            0x08 => visitor.visit_string(raw::read_bare_string(&mut outer.reader)?),
            0x09 => visitor.visit_seq(SeqDecoder::list(outer)?),
            0x0a => visitor.visit_map(MapDecoder::new(outer)),
            0x0b => visitor.visit_seq(SeqDecoder::int_array(outer)?),
            0x0c => visitor.visit_seq(SeqDecoder::long_array(outer)?),
            t => Err(Error::InvalidTypeId(t)),
        }
    }

    /// Deserialize bool values from a byte. Fail if that byte is not 0 or 1.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.tag {
            0x01 => {
                let reader = &mut self.outer.reader;
                let value = raw::read_bare_byte(reader)?;
                match value {
                    0 => visitor.visit_bool(false),
                    1 => visitor.visit_bool(true),
                    b => Err(Error::NonBooleanByte(b)),
                }
            }
            _ => Err(Error::TagMismatch(self.tag, 0x01)),
        }
    }

    /// Interpret missing values as None.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    /// Deserialize newtype structs by their underlying types.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    forward_to_deserialize_any! {
        u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string bytes byte_buf seq
        map tuple_struct struct tuple enum identifier ignored_any
    }
}
