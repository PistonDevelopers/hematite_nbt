//! Serialize a Rust data structure into Named Binary Tag data.

use std::io;

use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use serde;
use serde::ser;

use raw;

use error::{Error, Result};
use serde::ser::Error as SerError;

/// Encode `value` in Named Binary Tag format to the given `io::Write`
/// destination, with an optional header.
#[inline]
pub fn to_writer<'a, W, T>(dst: &mut W, value: &T, header: Option<&'a str>) -> Result<()>
where
    W: ?Sized + io::Write,
    T: ?Sized + ser::Serialize,
{
    let mut encoder = Encoder::new(dst, header);
    value.serialize(&mut encoder)
}

/// Encode `value` in Named Binary Tag format to the given `io::Write`
/// destination, with an optional header.
pub fn to_gzip_writer<'a, W, T>(dst: &mut W, value: &T, header: Option<&'a str>) -> Result<()>
where
    W: ?Sized + io::Write,
    T: ?Sized + ser::Serialize,
{
    let mut encoder = Encoder::new(GzEncoder::new(dst, Compression::default()), header);
    value.serialize(&mut encoder)
}

/// Encode `value` in Named Binary Tag format to the given `io::Write`
/// destination, with an optional header.
pub fn to_zlib_writer<'a, W, T>(dst: &mut W, value: &T, header: Option<&'a str>) -> Result<()>
where
    W: ?Sized + io::Write,
    T: ?Sized + ser::Serialize,
{
    let mut encoder = Encoder::new(ZlibEncoder::new(dst, Compression::default()), header);
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

impl<'a, W> Encoder<'a, W>
where
    W: io::Write,
{
    /// Create an encoder with optional `header` from a given Writer.
    pub fn new(writer: W, header: Option<&'a str>) -> Self {
        Encoder { writer, header }
    }

    /// Write the NBT tag and an optional header to the underlying writer.
    #[inline]
    fn write_header(&mut self, tag: i8, header: Option<&str>) -> Result<()> {
        raw::write_bare_byte(&mut self.writer, tag)?;
        match header {
            None => raw::write_bare_short(&mut self.writer, 0).map_err(From::from),
            Some(h) => raw::write_bare_string(&mut self.writer, h).map_err(From::from),
        }
    }
}

/// "Inner" version of the NBT encoder, capable of serializing bare types.
struct InnerEncoder<'a, 'b: 'a, W: 'a> {
    outer: &'a mut Encoder<'b, W>,
}

impl<'a, 'b, W> InnerEncoder<'a, 'b, W>
where
    W: io::Write,
{
    pub fn from_outer(outer: &'a mut Encoder<'b, W>) -> Self {
        InnerEncoder { outer }
    }
}

#[doc(hidden)]
pub struct Compound<'a, 'b: 'a, W: 'a> {
    outer: &'a mut Encoder<'b, W>,
    length: i32,
    sigil: bool,
}

impl<'a, 'b, W> Compound<'a, 'b, W>
where
    W: io::Write,
{
    fn from_outer(outer: &'a mut Encoder<'b, W>) -> Self {
        Compound {
            outer,
            length: 0,
            sigil: false,
        }
    }

    fn for_seq(outer: &'a mut Encoder<'b, W>, length: i32, array: bool) -> Result<Self> {
        if length == 0 || array {
            // Write sigil for empty list or typed array, because SerializeSeq::serialize_element is never called
            if !array {
                // For an empty list, write TAG_End as the tag type.
                raw::write_bare_byte(&mut outer.writer, 0x00)?;
            }
            // Write list/array length
            raw::write_bare_int(&mut outer.writer, length)?;
        }
        Ok(Compound {
            outer,
            length,
            sigil: false,
        })
    }
}

impl<'a, 'b, W> ser::SerializeSeq for Compound<'a, 'b, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        if !self.sigil {
            value.serialize(&mut TagEncoder::from_outer(
                self.outer,
                Option::<String>::None,
            ))?;
            raw::write_bare_int(&mut self.outer.writer, self.length)?;
            self.sigil = true;
        }
        value.serialize(&mut InnerEncoder::from_outer(self.outer))
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b, W> ser::SerializeTupleStruct for Compound<'a, 'b, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut InnerEncoder::from_outer(self.outer))
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b, W> ser::SerializeStruct for Compound<'a, 'b, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut TagEncoder::from_outer(self.outer, Some(key)))?;
        value.serialize(&mut InnerEncoder::from_outer(self.outer))
    }

    fn end(self) -> Result<()> {
        raw::close_nbt(&mut self.outer.writer)
    }
}

impl<'a, 'b, W> ser::SerializeMap for Compound<'a, 'b, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        unimplemented!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: serde::Serialize,
        V: serde::Serialize,
    {
        value.serialize(&mut TagEncoder::from_outer(self.outer, Some(key)))?;
        value.serialize(&mut InnerEncoder::from_outer(self.outer))
    }

    fn end(self) -> Result<()> {
        raw::close_nbt(&mut self.outer.writer)
    }
}

impl<'a, 'b, W> serde::Serializer for &'a mut Encoder<'b, W>
where
    W: io::Write,
{
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
        self.write_header(0x0a, header)?;
        raw::close_nbt(&mut self.writer).map_err(From::from)
    }

    /// Serialize newtype structs by their underlying type. Note that this will
    /// only be successful if the underyling type is a struct or a map.
    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    /// Serialize maps as `Tag_Compound` data.
    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if matches!(len, Some(0)) {
            self.write_header(0, None)?;
            return Ok(Compound::from_outer(self));
        }

        let header = self.header; // Circumvent strange borrowing errors.
        self.write_header(0x0a, header)?;
        Ok(Compound::from_outer(self))
    }

    /// Serialize structs as `Tag_Compound` data.
    #[inline]
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        if len == 0 {
            self.write_header(0, None)?;
            return Ok(Compound::from_outer(self));
        }

        let header = self.header; // Circumvent strange borrowing errors.
        self.write_header(0x0a, header)?;
        Ok(Compound::from_outer(self))
    }
}

impl<'a, 'b, W> serde::Serializer for &'a mut InnerEncoder<'a, 'b, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, 'b, W>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = Compound<'a, 'b, W>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = Compound<'a, 'b, W>;
    type SerializeStruct = Compound<'a, 'b, W>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    unrepresentable!(
        u8 u16 u32 u64 char unit newtype_variant tuple
            tuple_variant struct_variant
    );

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        self.serialize_i8(value as i8)
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        raw::write_bare_byte(&mut self.outer.writer, value).map_err(From::from)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        raw::write_bare_short(&mut self.outer.writer, value).map_err(From::from)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        raw::write_bare_int(&mut self.outer.writer, value).map_err(From::from)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        raw::write_bare_long(&mut self.outer.writer, value).map_err(From::from)
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<()> {
        raw::write_bare_float(&mut self.outer.writer, value).map_err(From::from)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<()> {
        raw::write_bare_double(&mut self.outer.writer, value).map_err(From::from)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        raw::write_bare_string(&mut self.outer.writer, value).map_err(From::from)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
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
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        raw::close_nbt(&mut self.outer.writer).map_err(From::from)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(l) = len {
            Compound::for_seq(self.outer, l as i32, false)
        } else {
            Err(Error::UnrepresentableType("unsized list"))
        }
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Compound::from_outer(self.outer))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(Compound::from_outer(self.outer))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        match name {
            "__hematite_nbt_i8_array__"
            | "__hematite_nbt_i32_array__"
            | "__hematite_nbt_i64_array__" => Compound::for_seq(self.outer, len as i32, true),
            _ => Err(Error::UnrepresentableType(stringify!(tuple_struct))),
        }
    }
}

/// A serializer for valid map keys, i.e. strings.
struct MapKeyEncoder<'a, 'b: 'a, W: 'a> {
    outer: &'a mut Encoder<'b, W>,
}

impl<'a, 'b: 'a, W: 'a> MapKeyEncoder<'a, 'b, W>
where
    W: io::Write,
{
    pub fn from_outer(outer: &'a mut Encoder<'b, W>) -> Self {
        MapKeyEncoder { outer }
    }
}

impl<'a, 'b: 'a, W: 'a> serde::Serializer for &'a mut MapKeyEncoder<'a, 'b, W>
where
    W: io::Write,
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
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        raw::write_bare_string(&mut self.outer.writer, value)
    }
}

/// A serializer for valid map keys.
struct TagEncoder<'a, 'b: 'a, W: 'a, K> {
    outer: &'a mut Encoder<'b, W>,
    key: Option<K>,
}

impl<'a, 'b: 'a, W: 'a, K> TagEncoder<'a, 'b, W, K>
where
    W: io::Write,
    K: serde::Serialize,
{
    fn from_outer(outer: &'a mut Encoder<'b, W>, key: Option<K>) -> Self {
        TagEncoder { outer, key }
    }

    fn write_header(&mut self, tag: i8) -> Result<()> {
        use serde::Serialize;
        raw::write_bare_byte(&mut self.outer.writer, tag)?;
        self.key
            .serialize(&mut MapKeyEncoder::from_outer(self.outer))
    }
}

impl<'a, 'b: 'a, W: 'a, K> serde::Serializer for &'a mut TagEncoder<'a, 'b, W, K>
where
    W: io::Write,
    K: serde::Serialize,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = NoOp;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = NoOp;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = NoOp;
    type SerializeStruct = NoOp;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    unrepresentable!(
        u8 u16 u32 u64 char unit newtype_variant tuple
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
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
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
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.write_header(0x0a)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if len.is_some() {
            self.write_header(0x09)?;
            Ok(NoOp)
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
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.write_header(0x0a)?;
        Ok(NoOp)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        match name {
            "__hematite_nbt_i8_array__" => self.write_header(0x07)?,
            "__hematite_nbt_i32_array__" => self.write_header(0x0b)?,
            "__hematite_nbt_i64_array__" => self.write_header(0x0c)?,
            _ => return Err(Error::UnrepresentableType("tuple struct")),
        }

        Ok(NoOp)
    }
}

/// This empty serializer provides a way to serialize only headers/tags for
/// sequences, maps, and structs.
struct NoOp;

impl ser::SerializeSeq for NoOp {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for NoOp {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStruct for NoOp {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
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
    where
        T: serde::Serialize,
    {
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

/// This function provides serde serialization support for NBT type `ByteArray`.
///
/// It should be used in conjunction with serde's field annotation `serialize_with`.
/// In the following example `byte_data` will be serialized as a `ByteArray`
/// instead of a `List` of `Byte`s:
///
/// ```
/// extern crate serde;
/// use nbt::to_writer;
/// use serde::Serialize;
///
/// let mut serialized = Vec::new();
///
/// // Declare your struct
/// #[derive(Serialize)]
/// struct Sheep {
///     #[serde(serialize_with="nbt::i8_array")]
///     byte_data: Vec<i8>,
/// }
///
/// // Serialize to NBT!
/// to_writer(
///     &mut serialized,
///     &Sheep {
///         byte_data: vec![0x62, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x20, 0x73, 0x68, 0x65, 0x65, 0x70],
///     },
///     None
/// ).unwrap();
///
/// print!("Serialized: {:?}", serialized);
/// ```
pub fn i8_array<T, S>(array: T, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    T: IntoIterator,
    <T as IntoIterator>::Item: std::borrow::Borrow<i8>,
    S: serde::ser::Serializer,
{
    array_serializer!("i8_array", array, serializer)
}

/// This function provides serde serialization support for NBT type `IntArray`.
///
/// It should be used in conjunction with serde's field annotation `serialize_with`.
/// In the following example `int_data` will be serialized as an `IntArray`
/// instead of a `List` of `Int`s:
///
/// ```
/// extern crate serde;
/// use nbt::to_writer;
/// use serde::Serialize;
///
/// let mut serialized = Vec::new();
///
/// // Declare your struct
/// #[derive(Serialize)]
/// struct Cow {
///     #[serde(serialize_with="nbt::i32_array")]
///     int_data: Vec<i32>,
/// }
///
/// // Serialize to NBT!
/// to_writer(
///     &mut serialized,
///     &Cow {
///         int_data: vec![1, 8, 64, 512, 4096, 32768, 262144],
///     },
///     None
/// ).unwrap();
///
/// print!("Serialized: {:?}", serialized);
/// ```
pub fn i32_array<T, S>(array: T, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    T: IntoIterator,
    <T as IntoIterator>::Item: std::borrow::Borrow<i32>,
    S: serde::ser::Serializer,
{
    array_serializer!("i32_array", array, serializer)
}

/// This function provides serde serialization support for NBT type `LongArray`.
///
/// It should be used in conjunction with serde's field annotation `serialize_with`.
/// In the following example `int_data` will be serialized as a `LongArray`
/// instead of a `List` of `Int`s:
///
/// ```
/// extern crate serde;
/// use nbt::to_writer;
/// use serde::Serialize;
///
/// let mut serialized = Vec::new();
///
/// // Declare your struct
/// #[derive(Serialize)]
/// struct Enderman {
///     #[serde(serialize_with="nbt::i64_array")]
///     long_data: Vec<i64>,
/// }
///
/// // Serialize to NBT!
/// to_writer(
///     &mut serialized,
///     &Enderman {
///         long_data: vec![0x1848ccd2157df10e, 0x64c5efff28280e9a],
///     },
///     None
/// ).unwrap();
///
/// print!("Serialized: {:?}", serialized);
/// ```
pub fn i64_array<T, S>(array: T, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    T: IntoIterator,
    <T as IntoIterator>::Item: std::borrow::Borrow<i64>,
    S: serde::ser::Serializer,
{
    array_serializer!("i64_array", array, serializer)
}
