//! Primitive functions for serializing and deserializing NBT data.

use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};
use cesu8::{from_java_cesu8, to_java_cesu8};

use error::{Error, Result};

#[derive(Debug, Clone, Copy)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

pub(crate) struct RawWriter<W: io::Write> {
    inner: W,
    endian: Endianness,
}

impl<W> RawWriter<W>
    where W: io::Write,
{
    pub fn new(inner: W, endian: Endianness) -> Self {
        RawWriter { inner, endian }
    }

    /// A convenience function for closing NBT format objects.
    ///
    /// This function writes a single `0x00` byte to the `io::Write` destination,
    /// which in the NBT format indicates that an open Compound is now closed.
    pub fn close_nbt(&mut self) -> Result<()>
    {
        self.inner.write_u8(0x00).map_err(From::from)
    }

    #[inline]
    pub fn write_bare_byte(&mut self, value: i8) -> Result<()>
    {
        self.inner.write_i8(value).map_err(From::from)
    }

    #[inline]
    pub fn write_bare_short(&mut self, value: i16) -> Result<()>
    {
        match self.endian {
            Endianness::LittleEndian => self.inner.write_i16::<byteorder::LittleEndian>(value).map_err(From::from),
            Endianness::BigEndian => self.inner.write_i16::<byteorder::BigEndian>(value).map_err(From::from),
        }
    }

    #[inline]
    pub fn write_bare_int(&mut self, value: i32) -> Result<()>
    {
        match self.endian {
            Endianness::LittleEndian => self.inner.write_i32::<byteorder::LittleEndian>(value).map_err(From::from),
            Endianness::BigEndian => self.inner.write_i32::<byteorder::BigEndian>(value).map_err(From::from),
        }
    }

    #[inline]
    pub fn write_bare_long(&mut self, value: i64) -> Result<()>
    {
        match self.endian {
            Endianness::LittleEndian => self.inner.write_i64::<byteorder::LittleEndian>(value).map_err(From::from),
            Endianness::BigEndian => self.inner.write_i64::<byteorder::BigEndian>(value).map_err(From::from),
        }
    }

    #[inline]
    pub fn write_bare_float(&mut self, value: f32) -> Result<()>
    {
        match self.endian {
            Endianness::LittleEndian => self.inner.write_f32::<byteorder::LittleEndian>(value).map_err(From::from),
            Endianness::BigEndian => self.inner.write_f32::<byteorder::BigEndian>(value).map_err(From::from),
        }
    }

    #[inline]
    pub fn write_bare_double(&mut self, value: f64) -> Result<()>
    {
        match self.endian {
            Endianness::LittleEndian => self.inner.write_f64::<byteorder::LittleEndian>(value).map_err(From::from),
            Endianness::BigEndian => self.inner.write_f64::<byteorder::BigEndian>(value).map_err(From::from),
        }
    }

    #[inline]
    pub fn write_bare_byte_array(&mut self, value: &[i8]) -> Result<()>
    {
        self.write_bare_int(value.len() as i32)?;
        for &v in value {
            self.write_bare_byte(v)?;
        }
        Ok(())
    }

    #[inline]
    pub fn write_bare_int_array(&mut self, value: &[i32]) -> Result<()>
    {
        self.write_bare_int(value.len() as i32)?;
        for &v in value {
            self.write_bare_int(v)?;
        }
        Ok(())
    }

    #[inline]
    pub fn write_bare_long_array(&mut self, value: &[i64]) -> Result<()>
    {
        self.write_bare_int(value.len() as i32)?;
        for &v in value {
            self.write_bare_long(v)?;
        }
        Ok(())
    }

    #[inline]
    pub fn write_bare_string(&mut self, value: &str) -> Result<()>
    {
        let encoded = to_java_cesu8(value);
        self.write_bare_short(encoded.len() as i16)?;
        self.inner.write_all(&encoded).map_err(From::from)
    }

}

pub(crate) struct RawReader<R: io::Read> {
    inner: R,
    endian: Endianness,
}

impl<R> RawReader<R>
    where R: io::Read,
{
    pub fn new(inner: R, endian: Endianness) -> Self {
        RawReader { inner, endian }
    }

    /// Extracts the next header (tag and name) from an NBT format source.
    ///
    /// This function will also return the `TAG_End` byte and an empty name if it
    /// encounters it.
    pub fn emit_next_header(&mut self) -> Result<(i8, String)>
    {
        let tag  = self.inner.read_i8()?;

        match tag {
            0x00 => { Ok((tag, "".to_string())) },
            _    => {
                let name = self.read_bare_string()?;
                Ok((tag, name))
            },
        }
    }

    #[inline]
    pub fn read_bare_byte(&mut self) -> Result<i8>
    {
        self.inner.read_i8().map_err(From::from)
    }

    #[inline]
    pub fn read_bare_short(&mut self) -> Result<i16>
    {
        match self.endian {
            Endianness::LittleEndian => self.inner.read_i16::<byteorder::LittleEndian>().map_err(From::from),
            Endianness::BigEndian => self.inner.read_i16::<byteorder::BigEndian>().map_err(From::from),
        }
    }

    #[inline]
    pub fn read_bare_int(&mut self) -> Result<i32>
    {
        match self.endian {
            Endianness::LittleEndian => self.inner.read_i32::<byteorder::LittleEndian>().map_err(From::from),
            Endianness::BigEndian => self.inner.read_i32::<byteorder::BigEndian>().map_err(From::from),
        }
    }

    #[inline]
    pub fn read_bare_long(&mut self) -> Result<i64>
    {
        match self.endian {
            Endianness::LittleEndian => self.inner.read_i64::<byteorder::LittleEndian>().map_err(From::from),
            Endianness::BigEndian => self.inner.read_i64::<byteorder::BigEndian>().map_err(From::from),
        }
    }

    #[inline]
    pub fn read_bare_float(&mut self) -> Result<f32>
    {
        match self.endian {
            Endianness::LittleEndian => self.inner.read_f32::<byteorder::LittleEndian>().map_err(From::from),
            Endianness::BigEndian => self.inner.read_f32::<byteorder::BigEndian>().map_err(From::from),
        }
    }

    #[inline]
    pub fn read_bare_double(&mut self) -> Result<f64>
    {
        match self.endian {
            Endianness::LittleEndian => self.inner.read_f64::<byteorder::LittleEndian>().map_err(From::from),
            Endianness::BigEndian => self.inner.read_f64::<byteorder::BigEndian>().map_err(From::from),
        }
    }

    #[inline]
    pub fn read_bare_byte_array(&mut self) -> Result<Vec<i8>>
    {
        // FIXME: Is there a way to return [u8; len]?
        let len = self.read_bare_int()? as usize;
        let mut buf = Vec::with_capacity(len);
        // FIXME: Test performance vs transmute.
        for _ in 0..len {
            buf.push(self.read_bare_byte()?);
        }
        Ok(buf)
    }

    #[inline]
    pub fn read_bare_int_array(&mut self) -> Result<Vec<i32>>
    {
        // FIXME: Is there a way to return [i32; len]?
        let len = self.read_bare_int()? as usize;
        let mut buf = Vec::with_capacity(len);
        // FIXME: Test performance vs transmute.
        for _ in 0..len {
            buf.push(self.read_bare_int()?);
        }
        Ok(buf)
    }

    #[inline]
    pub fn read_bare_long_array(&mut self) -> Result<Vec<i64>>
    {
        let len = self.read_bare_int()? as usize;
        let mut buf = Vec::with_capacity(len);
        for _ in 0..len {
            buf.push(self.read_bare_long()?);
        }
        Ok(buf)
    }

    #[inline]
    pub fn read_bare_string(&mut self) -> Result<String>
    {
        let len = self.read_bare_short()? as usize;

        if len == 0 { return Ok("".to_string()); }

        let mut bytes = vec![0; len];
        let mut n_read = 0usize;
        while n_read < bytes.len() {
            match self.inner.read(&mut bytes[n_read..])? {
                0 => return Err(Error::IncompleteNbtValue),
                n => n_read += n
            }
        }

        let decoded = from_java_cesu8(&bytes)?;
        Ok(decoded.into_owned())
    }
}
