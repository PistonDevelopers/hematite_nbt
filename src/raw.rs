//! Primitive functions for serializing and deserializing NBT data.

use std::{
    borrow::Cow,
    io::{self, Cursor},
    usize,
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use cesu8::{from_java_cesu8, to_java_cesu8};

use error::{Error, Result};

/// A convenience function for closing NBT format objects.
///
/// This function writes a single `0x00` byte to the `io::Write` destination,
/// which in the NBT format indicates that an open Compound is now closed.
pub fn close_nbt<W>(dst: &mut W) -> Result<()>
where
    W: io::Write,
{
    dst.write_u8(0x00).map_err(From::from)
}

#[inline]
pub fn write_bare_byte<W>(dst: &mut W, value: i8) -> Result<()>
where
    W: io::Write,
{
    dst.write_i8(value).map_err(From::from)
}

#[inline]
pub fn write_bare_short<W>(dst: &mut W, value: i16) -> Result<()>
where
    W: io::Write,
{
    dst.write_i16::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_int<W>(dst: &mut W, value: i32) -> Result<()>
where
    W: io::Write,
{
    dst.write_i32::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_long<W>(dst: &mut W, value: i64) -> Result<()>
where
    W: io::Write,
{
    dst.write_i64::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_float<W>(dst: &mut W, value: f32) -> Result<()>
where
    W: io::Write,
{
    dst.write_f32::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_double<W>(dst: &mut W, value: f64) -> Result<()>
where
    W: io::Write,
{
    dst.write_f64::<BigEndian>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_byte_array<W>(dst: &mut W, value: &[i8]) -> Result<()>
where
    W: io::Write,
{
    dst.write_i32::<BigEndian>(value.len() as i32)?;
    for &v in value {
        dst.write_i8(v)?;
    }
    Ok(())
}

#[inline]
pub fn write_bare_int_array<W>(dst: &mut W, value: &[i32]) -> Result<()>
where
    W: io::Write,
{
    dst.write_i32::<BigEndian>(value.len() as i32)?;
    for &v in value {
        dst.write_i32::<BigEndian>(v)?;
    }
    Ok(())
}

#[inline]
pub fn write_bare_long_array<W>(dst: &mut W, value: &[i64]) -> Result<()>
where
    W: io::Write,
{
    dst.write_i32::<BigEndian>(value.len() as i32)?;
    for &v in value {
        dst.write_i64::<BigEndian>(v)?;
    }
    Ok(())
}

#[inline]
pub fn write_bare_string<W>(dst: &mut W, value: &str) -> Result<()>
where
    W: io::Write,
{
    let encoded = to_java_cesu8(value);
    dst.write_u16::<BigEndian>(encoded.len() as u16)?;
    dst.write_all(&encoded).map_err(From::from)
}

pub trait Read<'de> {
    /// Extracts the next header (tag and name) from an NBT format source.
    ///
    /// This function will also return the `TAG_End` byte and an empty name if it
    /// encounters it.
    fn emit_next_header<'s>(
        &mut self,
        scratch: Option<&'s mut Vec<u8>>,
    ) -> Result<(u8, Reference<'de, 's, str>)> {
        let tag = self.read_id()?;

        match tag {
            0x00 => Ok((tag, Reference::Borrowed(""))),
            _ => {
                let name = self.read_bare_string(scratch)?;
                Ok((tag, name))
            }
        }
    }

    fn read_id(&mut self) -> Result<u8>;
    fn read_length(&mut self) -> Result<i32>;
    fn read_bare_byte(&mut self) -> Result<i8>;
    fn read_bare_short(&mut self) -> Result<i16>;
    fn read_bare_int(&mut self) -> Result<i32>;
    fn read_bare_long(&mut self) -> Result<i64>;
    fn read_bare_float(&mut self) -> Result<f32>;
    fn read_bare_double(&mut self) -> Result<f64>;
    fn read_bare_byte_array(&mut self) -> Result<Vec<i8>>;
    fn read_bare_int_array(&mut self) -> Result<Vec<i32>>;
    fn read_bare_long_array(&mut self) -> Result<Vec<i64>>;
    fn read_bare_string<'s>(
        &mut self,
        scratch: Option<&'s mut Vec<u8>>,
    ) -> Result<Reference<'de, 's, str>>;
}

pub enum Reference<'b, 'c, T>
where
    T: ?Sized + ToOwned + 'static,
{
    Borrowed(&'b T),
    Copied(&'c T),
    Owned(T::Owned),
}

impl<T> Reference<'_, '_, T>
where
    T: ?Sized + ToOwned + 'static,
{
    pub fn into_owned(self) -> T::Owned {
        match self {
            Reference::Borrowed(r) => r.to_owned(),
            Reference::Copied(r) => r.to_owned(),
            Reference::Owned(o) => o,
        }
    }
}

pub struct SliceRead<'de> {
    cursor: Cursor<&'de [u8]>,
}

impl<'de> SliceRead<'de> {
    pub fn new(data: &'de [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    pub fn get_inner(&self) -> &'de [u8] {
        &self.cursor.get_ref()[self.cursor.position() as usize..]
    }
}

impl<'de> Read<'de> for SliceRead<'de> {
    #[inline]
    fn read_id(&mut self) -> Result<u8> {
        self.cursor.read_u8().map_err(From::from)
    }

    #[inline]
    fn read_length(&mut self) -> Result<i32> {
        self.cursor.read_i32::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_byte(&mut self) -> Result<i8> {
        self.cursor.read_i8().map_err(From::from)
    }

    #[inline]
    fn read_bare_short(&mut self) -> Result<i16> {
        self.cursor.read_i16::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_int(&mut self) -> Result<i32> {
        self.cursor.read_i32::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_long(&mut self) -> Result<i64> {
        self.cursor.read_i64::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_float(&mut self) -> Result<f32> {
        self.cursor.read_f32::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_double(&mut self) -> Result<f64> {
        self.cursor.read_f64::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_byte_array<'s>(&mut self) -> Result<Vec<i8>> {
        // FIXME: Is there a way to return [u8; len]?
        let len = self.cursor.read_i32::<BigEndian>()? as usize;
        let pos = self.cursor.position();
        let buf = &self.cursor.get_ref()[pos as usize..];
        if buf.len() < len {
            return Err(Error::IncompleteNbtValue);
        }
        self.cursor.set_position(pos + len as u64);
        let buf = &buf[..len];

        let b = unsafe { std::slice::from_raw_parts(buf.as_ptr().cast(), len) };
        Ok(b.to_vec())
    }

    #[inline]
    fn read_bare_int_array(&mut self) -> Result<Vec<i32>> {
        // FIXME: Is there a way to return [i32; len]?
        let len = self.cursor.read_i32::<BigEndian>()? as usize;
        let mut buf = Vec::with_capacity(len);
        // FIXME: Test performance vs transmute.
        for _ in 0..len {
            buf.push(self.cursor.read_i32::<BigEndian>()?);
        }
        Ok(buf)
    }

    #[inline]
    fn read_bare_long_array(&mut self) -> Result<Vec<i64>> {
        let len = self.cursor.read_i32::<BigEndian>()? as usize;
        let mut buf = Vec::with_capacity(len);
        for _ in 0..len {
            buf.push(self.cursor.read_i64::<BigEndian>()?);
        }
        Ok(buf)
    }

    #[inline]
    fn read_bare_string<'s>(
        &mut self,
        _scratch: Option<&'s mut Vec<u8>>,
    ) -> Result<Reference<'de, 's, str>> {
        let len = self.cursor.read_u16::<BigEndian>()? as usize;

        if len == 0 {
            return Ok(Reference::Borrowed(""));
        }

        let pos = self.cursor.position();
        let bytes = &self.cursor.get_ref()[pos as usize..];
        if bytes.len() < len {
            return Err(Error::IncompleteNbtValue);
        }
        let bytes = &bytes[..len];
        self.cursor.set_position(pos + len as u64);

        let decoded = from_java_cesu8(bytes)?;
        let reference = match decoded {
            Cow::Borrowed(s) => Reference::Borrowed(s),
            Cow::Owned(s) => Reference::Owned(s),
        };
        Ok(reference)
    }
}

impl<'de, T: io::Read> Read<'de> for T {
    #[inline]
    fn read_id(&mut self) -> Result<u8> {
        self.read_u8().map_err(From::from)
    }

    #[inline]
    fn read_length(&mut self) -> Result<i32> {
        self.read_i32::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_byte(&mut self) -> Result<i8> {
        self.read_i8().map_err(From::from)
    }

    #[inline]
    fn read_bare_short(&mut self) -> Result<i16> {
        self.read_i16::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_int(&mut self) -> Result<i32> {
        self.read_i32::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_long(&mut self) -> Result<i64> {
        self.read_i64::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_float(&mut self) -> Result<f32> {
        self.read_f32::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_double(&mut self) -> Result<f64> {
        self.read_f64::<BigEndian>().map_err(From::from)
    }

    #[inline]
    fn read_bare_byte_array(&mut self) -> Result<Vec<i8>> {
        // FIXME: Is there a way to return [u8; len]?
        let len = self.read_i32::<BigEndian>()? as usize;
        let mut buf = Vec::with_capacity(len);
        // FIXME: Test performance vs transmute.
        for _ in 0..len {
            buf.push(self.read_i8()?);
        }
        Ok(buf)
    }

    #[inline]
    fn read_bare_int_array(&mut self) -> Result<Vec<i32>> {
        // FIXME: Is there a way to return [i32; len]?
        let len = self.read_i32::<BigEndian>()? as usize;
        let mut buf = Vec::with_capacity(len);
        // FIXME: Test performance vs transmute.
        for _ in 0..len {
            buf.push(self.read_i32::<BigEndian>()?);
        }
        Ok(buf)
    }

    #[inline]
    fn read_bare_long_array(&mut self) -> Result<Vec<i64>> {
        let len = self.read_i32::<BigEndian>()? as usize;
        let mut buf = Vec::with_capacity(len);
        for _ in 0..len {
            buf.push(self.read_i64::<BigEndian>()?);
        }
        Ok(buf)
    }

    #[inline]
    fn read_bare_string<'s>(
        &mut self,
        scratch: Option<&'s mut Vec<u8>>,
    ) -> Result<Reference<'de, 's, str>> {
        let len = self.read_u16::<BigEndian>()? as usize;

        if len == 0 {
            return Ok(Reference::Borrowed(""));
        }

        if let Some(scratch) = scratch {
            scratch.resize(len, 0);
            self.read_exact(scratch)
                .map_err(|_| Error::IncompleteNbtValue)?;

            let decoded = from_java_cesu8(scratch)?;
            let reference = match decoded {
                Cow::Borrowed(s) => Reference::Copied(s),
                Cow::Owned(s) => Reference::Owned(s),
            };
            Ok(reference)
        } else {
            let mut buf = vec![0; len];
            self.read_exact(&mut buf)
                .map_err(|_| Error::IncompleteNbtValue)?;

            let decoded = from_java_cesu8(&buf)?;
            Ok(Reference::Owned(decoded.into_owned()))
        }
    }
}
