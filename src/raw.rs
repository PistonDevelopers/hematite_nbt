//! Contains raw, primitive functions for serializing and deserializing basic
//! Named Binary Tag types.
//!
//! This submodule is not intended for general use, but is exposed for those
//! interested in writing fast NBT encoding/decoding by hand, where it may be
//! quite useful.
//!
//! For the higher-level abstraction over these primitives, see the
//! [`NbtFmt`](../trait.NbtFmt.html) trait in the parent module.

use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use error::{Error, Result};
use serialize::NbtFmt;

/// A convenience function for closing NBT format objects.
///
/// This function writes a single `0x00` byte to the `io::Write` destination,
/// which in the NBT format indicates that an open Compound is now closed.
pub fn close_nbt<W>(dst: &mut W) -> Result<()>
    where W: io::Write {

    dst.write_u8(0x00).map_err(From::from)
}

#[inline]
pub fn write_bare_byte<W>(dst: &mut W, value: i8) -> Result<()>
   where W: io::Write
{
    dst.write_i8(value).map_err(From::from)
}

#[inline]
pub fn write_bare_short<W>(dst: &mut W, value: i16) -> Result<()>
   where W: io::Write
{
    dst.write_i16::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_int<W>(dst: &mut W, value: i32) -> Result<()>
   where W: io::Write
{
    dst.write_i32::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_long<W>(dst: &mut W, value: i64) -> Result<()>
   where W: io::Write
{
    dst.write_i64::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_float<W>(dst: &mut W, value: f32) -> Result<()>
   where W: io::Write
{
    dst.write_f32::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_double<W>(dst: &mut W, value: f64) -> Result<()>
   where W: io::Write
{
    dst.write_f64::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_byte_array<W>(dst: &mut W, value: &[i8]) -> Result<()>
   where W: io::Write
{
    try!(dst.write_i32::<BigEndian>(value.len() as i32));
    for &v in value {
        try!(dst.write_i8(v));
    }
    Ok(())
}

#[inline]
pub fn write_bare_int_array<W>(dst: &mut W, value: &[i32]) -> Result<()>
   where W: io::Write
{
    try!(dst.write_i32::<BigEndian>(value.len() as i32));
    for &v in value {
        try!(dst.write_i32::<BigEndian>(v));
    }
    Ok(())
}

#[inline]
pub fn write_bare_string<W>(dst: &mut W, value: &str) -> Result<()>
   where W: io::Write
{    
    try!(dst.write_u16::<BigEndian>(value.len() as u16));
    dst.write_all(value.as_bytes()).map_err(From::from)
}

#[inline]
pub fn write_bare_list<'a, W, I, T>(dst: &mut W, values: I) -> Result<()>
   where W: io::Write,
         I: Iterator<Item=&'a T> + ExactSizeIterator,
         T: 'a + NbtFmt
{
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
pub fn write_bare_compound<'a, W, I, T, S>(dst: &mut W, values: I) -> Result<()>
   where W: io::Write,
         I: Iterator<Item=(&'a S, &'a T)>,
         S: 'a + AsRef<str>,
         T: 'a + NbtFmt
{
    for (key, ref value) in values {
        try!(value.to_nbt(dst, key.as_ref()));
    }
    
    // Write the marker for the end of the Compound.
    close_nbt(dst)
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
            let name = try!(read_bare_string(src));
            Ok((tag, name))
        },
    }
}

#[inline]
pub fn read_bare_byte<R>(src: &mut R) -> Result<i8>
    where R: io::Read
{
    src.read_i8().map_err(From::from)
}

#[inline]
pub fn read_bare_short<R>(src: &mut R) -> Result<i16>
    where R: io::Read
{
    src.read_i16::<BigEndian>().map_err(From::from)
}

#[inline]
pub fn read_bare_int<R>(src: &mut R) -> Result<i32>
    where R: io::Read
{
    src.read_i32::<BigEndian>().map_err(From::from)
}

#[inline]
pub fn read_bare_long<R>(src: &mut R) -> Result<i64>
    where R: io::Read
{
    src.read_i64::<BigEndian>().map_err(From::from)
}

#[inline]
pub fn read_bare_float<R>(src: &mut R) -> Result<f32>
    where R: io::Read
{
    src.read_f32::<BigEndian>().map_err(From::from)
}

#[inline]
pub fn read_bare_double<R>(src: &mut R) -> Result<f64>
    where R: io::Read
{
    src.read_f64::<BigEndian>().map_err(From::from)
}

#[inline]
pub fn read_bare_byte_array<R>(src: &mut R) -> Result<Vec<i8>>
    where R: io::Read
{
    // FIXME: Is there a way to return [u8; len]?
    let len = try!(src.read_i32::<BigEndian>()) as usize;
    let mut buf = Vec::with_capacity(len);
    // FIXME: Test performance vs transmute.
    for _ in 0..len {
        buf.push(try!(src.read_i8()));
    }
    Ok(buf)
}

#[inline]
pub fn read_bare_int_array<R>(src: &mut R) -> Result<Vec<i32>>
    where R: io::Read
{
    // FIXME: Is there a way to return [i32; len]?
    let len = try!(src.read_i32::<BigEndian>()) as usize;
    let mut buf = Vec::with_capacity(len);
    // FIXME: Test performance vs transmute.
    for _ in 0..len {
        buf.push(try!(src.read_i32::<BigEndian>()));
    }
    Ok(buf)
}

#[inline]
pub fn read_bare_string<R>(src: &mut R) -> Result<String>
    where R: io::Read
{
    let len = try!(src.read_u16::<BigEndian>()) as usize;

    if len == 0 { return Ok("".to_string()); }

    let mut bytes = vec![0; len];
    let mut n_read = 0usize;
    while n_read < bytes.len() {
        match try!(src.read(&mut bytes[n_read..])) {
            0 => return Err(Error::IncompleteNbtValue),
            n => n_read += n
        }
    }

    String::from_utf8(bytes).map_err(From::from)
}

#[inline]
pub fn read_bare_list<R, T>(src: &mut R) -> Result<Vec<T>>
    where R: io::Read,
          T: NbtFmt<Into=T>
{
    // Note: This assumes the first (type) byte has already been read.
    let len = try!(src.read_i32::<BigEndian>()) as usize;
    let mut buf = Vec::with_capacity(len);
    for _ in 0..len {
        buf.push(try!(T::read_bare_nbt(src)));
    }
    Ok(buf)
}
