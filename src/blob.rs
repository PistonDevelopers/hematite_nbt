use std::collections::HashMap;
use std::fmt;
use std::io;
use std::ops::Index;

use byteorder::WriteBytesExt;
use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;

use error::{Error, Result};
use raw;
use value::Value;

/// A generic, complete object in Named Binary Tag format.
///
/// This is essentially a map of names to `Value`s, with an optional top-level
/// name of its own. It can be created in a similar way to a `HashMap`, or read
/// from an `io::Read` source, and its binary representation can be written to
/// an `io::Write` destination.
///
/// These read and write methods support both uncompressed and compressed
/// (through Gzip or zlib compression) methods.
///
/// ```rust
/// use nbt::{Blob, Value};
///
/// // Create a `Blob` from key/value pairs.
/// let mut nbt = Blob::new();
/// nbt.insert("name", "Herobrine").unwrap();
/// nbt.insert("health", 100i8).unwrap();
/// nbt.insert("food", 20.0f32).unwrap();
///
/// // Write a compressed binary representation to a byte array.
/// let mut dst = Vec::new();
/// nbt.to_zlib_writer(&mut dst).unwrap();
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Blob {
    title: String,
    content: HashMap<String, Value>,
}

impl Blob {
    /// Create a new NBT file format representation with an empty name.
    pub fn new() -> Blob {
        Blob {
            title: "".to_string(),
            content: HashMap::new(),
        }
    }

    /// Create a new NBT file format representation with the given name.
    pub fn named<S>(name: S) -> Blob
    where
        S: Into<String>,
    {
        Blob {
            title: name.into(),
            content: HashMap::new(),
        }
    }

    /// Extracts an `Blob` object from an `io::Read` source.
    pub fn from_reader<R>(src: &mut R) -> Result<Blob>
    where
        R: io::Read,
    {
        let (tag, title) = raw::emit_next_header(src)?;
        // Although it would be possible to read NBT format files composed of
        // arbitrary objects using the current API, by convention all files
        // have a top-level Compound.
        if tag != 0x0a {
            return Err(Error::NoRootCompound);
        }
        let content = Value::from_reader(tag, src)?;
        match content {
            Value::Compound(map) => Ok(Blob {
                title,
                content: map,
            }),
            _ => Err(Error::NoRootCompound),
        }
    }

    /// Extracts an `Blob` object from an `io::Read` source that is
    /// compressed using the Gzip format.
    pub fn from_gzip_reader<R>(src: &mut R) -> Result<Blob>
    where
        R: io::Read,
    {
        // Reads the gzip header, and fails if it is incorrect.
        let mut data = GzDecoder::new(src)?;
        Blob::from_reader(&mut data)
    }

    /// Extracts an `Blob` object from an `io::Read` source that is
    /// compressed using the zlib format.
    pub fn from_zlib_reader<R>(src: &mut R) -> Result<Blob>
    where
        R: io::Read,
    {
        Blob::from_reader(&mut ZlibDecoder::new(src))
    }

    /// Writes the binary representation of this `Blob` to an `io::Write`
    /// destination.
    pub fn to_writer<W>(&self, mut dst: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        dst.write_u8(0x0a)?;
        raw::write_bare_string(&mut dst, &self.title)?;
        for (name, ref nbt) in self.content.iter() {
            dst.write_u8(nbt.id())?;
            raw::write_bare_string(&mut dst, name)?;
            nbt.to_writer(&mut dst)?;
        }
        raw::close_nbt(&mut dst)
    }

    /// Writes the binary representation of this `Blob`, compressed using
    /// the Gzip format, to an `io::Write` destination.
    pub fn to_gzip_writer<W>(&self, dst: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        self.to_writer(&mut GzEncoder::new(dst, Compression::Default))
    }

    /// Writes the binary representation of this `Blob`, compressed using
    /// the Zlib format, to an `io::Write` dst.
    pub fn to_zlib_writer<W>(&self, dst: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        self.to_writer(&mut ZlibEncoder::new(dst, Compression::Default))
    }

    /// Insert an `Value` with a given name into this `Blob` object. This
    /// method is just a thin wrapper around the underlying `HashMap` method of
    /// the same name.
    ///
    /// This method will also return an error if a `Value::List` with
    /// heterogeneous elements is passed in, because this is illegal in the NBT
    /// file format.
    pub fn insert<S, V>(&mut self, name: S, value: V) -> Result<()>
    where
        S: Into<String>,
        V: Into<Value>,
    {
        // The follow prevents `List`s with heterogeneous tags from being
        // inserted into the file.
        let nvalue = value.into();
        if let Value::List(ref vals) = nvalue {
            if !vals.is_empty() {
                let first_id = vals[0].id();
                for nbt in vals {
                    if nbt.id() != first_id {
                        return Err(Error::HeterogeneousList);
                    }
                }
            }
        }
        self.content.insert(name.into(), nvalue);
        Ok(())
    }

    /// Tries to get a named `Value` in the blob.
    pub fn get<S>(&self, name: S) -> Option<&Value>
    where
        S: Into<&'static str>,
    {
        self.content.get(name.into())
    }

    /// The number of bytes this blob will serialize to, before compression
    pub fn len_bytes(&self) -> usize {
        /* compound tag + name length + TAG_End = 4 */
        4 + self.title.len()
            + self
                .content
                .iter()
                .map(Value::size_of_compound_entry)
                .sum::<usize>()
    }
}

impl<'a> Index<&'a str> for Blob {
    type Output = Value;

    fn index<'b>(&'b self, s: &'a str) -> &'b Value {
        self.content.get(s).unwrap()
    }
}

impl fmt::Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TAG_Compound(\"{}\"): {} entry(ies)\n{{\n",
            self.title,
            self.content.len()
        )?;
        for (name, tag) in self.content.iter() {
            write!(f, "  {}(\"{}\"): ", tag.tag_name(), name)?;
            tag.print(f, 2)?;
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}

#[cfg(feature = "serde")]
use serde::{self, ser::SerializeMap};

#[cfg(feature = "serde")]
impl serde::Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> serde::export::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        // No support for named Blobs.
        let mut state = serializer.serialize_map(Some(self.content.len()))?;
        for (k, v) in &self.content {
            state.serialize_entry(&k, &v)?;
        }
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Blob {
    fn deserialize<D>(deserializer: D) -> serde::export::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        // No support for named Blobs.
        let map: HashMap<String, Value> = serde::de::Deserialize::deserialize(deserializer)?;
        Ok(Blob {
            title: "".to_string(),
            content: map,
        })
    }
}
