//! Primitive functions for serializing and deserializing NBT data.

use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use cesu8::{from_java_cesu8, to_java_cesu8};

use error::{Error, Result};

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
    dst.write_i32::<BigEndian>(value.len() as i32)?;
    for &v in value {
        dst.write_i8(v)?;
    }
    Ok(())
}

#[inline]
pub fn write_bare_int_array<W>(dst: &mut W, value: &[i32]) -> Result<()>
   where W: io::Write
{
    dst.write_i32::<BigEndian>(value.len() as i32)?;
    for &v in value {
        dst.write_i32::<BigEndian>(v)?;
    }
    Ok(())
}

#[inline]
pub fn write_bare_long_array<W>(dst: &mut W, value: &[i64]) -> Result<()>
   where W: io::Write
{
    dst.write_i32::<BigEndian>(value.len() as i32)?;
    for &v in value {
        dst.write_i64::<BigEndian>(v)?;
    }
    Ok(())
}

#[inline]
pub fn write_bare_string<W>(dst: &mut W, value: &str) -> Result<()>
   where W: io::Write
{    
    let encoded = to_java_cesu8(value);
    dst.write_u16::<BigEndian>(encoded.len() as u16)?;
    dst.write_all(&encoded).map_err(From::from)
}

/// Extracts the next header (tag and name) from an NBT format source.
///
/// This function will also return the `TAG_End` byte and an empty name if it
/// encounters it.
pub fn emit_next_header<R>(src: &mut R) -> Result<(u8, String)>
    where R: io::Read
{
    let tag  = src.read_u8()?;

    match tag {
        0x00 => { Ok((tag, "".to_string())) },
        _    => {
            let name = read_bare_string(src)?;
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
    let len = src.read_i32::<BigEndian>()? as usize;
    let mut buf = Vec::with_capacity(len);
    // FIXME: Test performance vs transmute.
    for _ in 0..len {
        buf.push(src.read_i8()?);
    }
    Ok(buf)
}

#[inline]
pub fn read_bare_int_array<R>(src: &mut R) -> Result<Vec<i32>>
    where R: io::Read
{
    // FIXME: Is there a way to return [i32; len]?
    let len = src.read_i32::<BigEndian>()? as usize;
    let mut buf = Vec::with_capacity(len);
    // FIXME: Test performance vs transmute.
    for _ in 0..len {
        buf.push(src.read_i32::<BigEndian>()?);
    }
    Ok(buf)
}

#[inline]
pub fn read_bare_long_array<R>(src: &mut R) -> Result<Vec<i64>>
    where R: io::Read
{
    let len = src.read_i32::<BigEndian>()? as usize;
    let mut buf = Vec::with_capacity(len);
    for _ in 0..len {
        buf.push(src.read_i64::<BigEndian>()?);
    }
    Ok(buf)
}

#[inline]
pub fn read_bare_string<R>(src: &mut R) -> Result<String>
    where R: io::Read
{
    let len = src.read_u16::<BigEndian>()? as usize;

    if len == 0 { return Ok("".to_string()); }

    let mut bytes = vec![0; len];
    let mut n_read = 0usize;
    while n_read < bytes.len() {
        match src.read(&mut bytes[n_read..])? {
            0 => return Err(Error::IncompleteNbtValue),
            n => n_read += n
        }
    }

    let decoded = from_java_cesu8(&bytes)?;
    Ok(decoded.into_owned())
}
