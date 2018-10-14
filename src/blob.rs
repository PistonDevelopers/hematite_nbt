use std::collections::HashMap;
use std::fmt;
use std::io;
use std::ops::Index;

use flate2::Compression;
use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};

use error::{Error, Result};
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
/// let mut nbt = Blob::new("".to_string());
/// nbt.insert("name".to_string(), "Herobrine").unwrap();
/// nbt.insert("health".to_string(), 100i8).unwrap();
/// nbt.insert("food".to_string(), 20.0f32).unwrap();
///
/// // Write a compressed binary representation to a byte array.
/// let mut dst = Vec::new();
/// nbt.write_zlib(&mut dst).unwrap();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Blob {
    title: String,
    content: Value
}

impl Blob {
    /// Create a new NBT file format representation with the given name.
    pub fn new(title: String) -> Blob {
        let map: HashMap<String, Value> = HashMap::new();
        Blob { title: title, content: Value::Compound(map) }
    }

    /// Extracts an `Blob` object from an `io::Read` source.
    pub fn from_reader(src: &mut io::Read) -> Result<Blob> {
        let header = try!(Value::read_header(src));
        // Although it would be possible to read NBT format files composed of
        // arbitrary objects using the current API, by convention all files
        // have a top-level Compound.
        if header.0 != 0x0a {
            return Err(Error::NoRootCompound);
        }
        let content = try!(Value::from_reader(header.0, src));
        Ok(Blob { title: header.1, content: content })
    }

    /// Extracts an `Blob` object from an `io::Read` source that is
    /// compressed using the Gzip format.
    pub fn from_gzip(src: &mut io::Read) -> Result<Blob> {
        // Reads the gzip header, and fails if it is incorrect.
        let mut data = try!(GzDecoder::new(src));
        Blob::from_reader(&mut data)
    }

    /// Extracts an `Blob` object from an `io::Read` source that is
    /// compressed using the zlib format.
    pub fn from_zlib(src: &mut io::Read) -> Result<Blob> {
        Blob::from_reader(&mut ZlibDecoder::new(src))
    }

    /// Writes the binary representation of this `Blob` to an `io::Write`
    /// destination.
    pub fn write(&self, dst: &mut io::Write) -> Result<()> {
        try!(self.content.write_header(dst, &self.title));
        self.content.write(dst)
    }

    /// Writes the binary representation of this `Blob`, compressed using
    /// the Gzip format, to an `io::Write` destination.
    pub fn write_gzip(&self, dst: &mut io::Write) -> Result<()> {
        self.write(&mut GzEncoder::new(dst, Compression::Default))
    }

    /// Writes the binary representation of this `Blob`, compressed using
    /// the Zlib format, to an `io::Write` dst.
    pub fn write_zlib(&self, dst: &mut io::Write) -> Result<()> {
        self.write(&mut ZlibEncoder::new(dst, Compression::Default))
    }

    /// Insert an `Value` with a given name into this `Blob` object. This
    /// method is just a thin wrapper around the underlying `HashMap` method of
    /// the same name.
    ///
    /// This method will also return an error if a `Value::List` with
    /// heterogeneous elements is passed in, because this is illegal in the NBT
    /// file format.
    pub fn insert<V>(&mut self, name: String, value: V) -> Result<()>
           where V: Into<Value> {
        // The follow prevents `List`s with heterogeneous tags from being
        // inserted into the file. It would be nicer to return an error, but
        // this would depart from the `HashMap` API for `insert`.
        let nvalue = value.into();
        if let Value::List(ref vals) = nvalue {
            if vals.len() != 0 {
                let first_id = vals[0].id();
                for nbt in vals {
                    if nbt.id() != first_id {
                        return Err(Error::HeterogeneousList)
                    }
                }
            }
        }
        if let Value::Compound(ref mut v) = self.content {
            v.insert(name, nvalue);
        } else {
            unreachable!();
        }
        Ok(())
    }

    /// The uncompressed length of this `Blob`, in bytes.
    pub fn len(&self) -> usize {
        // tag + name + content
        1 + 2 + self.title.len() + self.content.len()
    }
}

impl<'a> Index<&'a str> for Blob {
    type Output = Value;

    fn index<'b>(&'b self, s: &'a str) -> &'b Value {
        match self.content {
            Value::Compound(ref v) => v.get(s).unwrap(),
            _ => unreachable!()
        }
    }
}

impl fmt::Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TAG_Compound(\"{}\"): {}", self.title, self.content)
    }
}
