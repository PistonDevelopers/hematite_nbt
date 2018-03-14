use std::collections::HashMap;
use std::fmt;
use std::io;
use std::ops::{Index, IndexMut};

use flate2::Compression;
use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};

use error::{Error, Result};
use value::Value;

/// A blob of Named Binary Tag (NBT) data.
///
/// This struct provides methods to read, write, create, and modify data with a
/// well-defined NBT binary representation (that is, it contains only valid
/// [`Value`](enum.Value.html) entries). The API is similar to a `HashMap`.
///
/// # Reading NBT-encoded Data
///
/// Minecraft encodes many data files in the NBT format, which you can inspect
/// and modify using `Blob` objects. For example, to print out the contents of a
/// [`level.dat` file](http://minecraft.gamepedia.com/Level_format#level.dat_format),
/// one could use the following:
///
/// ```ignore
/// use std::fs;
/// use nbt::Blob;
///
/// let mut file = fs::File::open("level.dat").unwrap();
/// let level = Blob::from_gzip(&mut file).unwrap();
/// println!("File contents:\n{}", level); 
/// ```
///
/// # Creating or Modifying NBT-encoded Data
///
/// `Blob` objects have an API similar to `HashMap<String, Value>`, supporting
/// an `insert()` method for inserting new data and the index operator for
/// accessing or modifying this data.
///
/// ```rust
/// use nbt::{Blob, Value};
///
/// // Create a `Blob` from key/value pairs.
/// let mut nbt = Blob::new();
/// nbt.insert("player", "Herobrine"); // Implicit conversion to `Value`.
/// nbt.insert("score", Value::Int(1400)); // Explicit `Value` type.
///
/// // Modify a value using the Index operator.
/// nbt["score"] = Value::Int(1401);
/// ```
///
/// # Writing NBT-encoded Data
///
/// `Blob` provides methods for writing uncompressed and compressed binary NBT
/// data to arbitrary `io::Write` destinations. For example, to write compressed
/// data to a byte vector:
///
/// ```rust
/// # use nbt::Blob;
/// # let nbt = Blob::new();
/// let mut dst = Vec::new();
/// nbt.write_gzip(&mut dst);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Blob {
    header: Option<String>,
    content: Value
}

impl Blob {
    /// Create a new NBT file format representation with an empty header.
    pub fn new() -> Blob {
        let map: HashMap<String, Value> = HashMap::new();
        Blob { header: None, content: Value::Compound(map) }
    }

    /// Create a new NBT file format representation the given header.
    pub fn with_header<S>(header: S) -> Blob
        where S: Into<String>
    {
        let map: HashMap<String, Value> = HashMap::new();
        Blob { header: Some(header.into()), content: Value::Compound(map) }
    }

    /// Extracts an `Blob` object from an `io::Read` source.
    pub fn from_reader(mut src: &mut io::Read) -> Result<Blob> {
        let (tag, header) = try!(Value::read_header(src));
        // Although it would be possible to read NBT format files composed of
        // arbitrary objects using the current API, by convention all files
        // have a top-level Compound.
        if tag != 0x0a {
            return Err(Error::NoRootCompound);
        }
        let content = try!(Value::from_reader(tag, src));
        Ok(Blob { header: header, content: content })
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
        match self.header {
            None => try!(self.content.write_header(dst, None)),
            Some(ref h) => try!(self.content.write_header(dst, Some(&h[..]))),
        }
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
    pub fn insert<V, S>(&mut self, name: S, value: V) -> Result<()>
           where S: Into<String>, V: Into<Value> {
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
            v.insert(name.into(), nvalue);
        } else {
            unreachable!();
        }
        Ok(())
    }

    /// The uncompressed length of this `Blob`, in bytes.
    pub fn len(&self) -> usize {
        // tag + name + content
        match self.header {
            None => 1 + 2 + self.content.len(),
            Some(ref h) => 1 + 2 + h.len() + self.content.len(),
        }
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

impl<'a> IndexMut<&'a str> for Blob {
    fn index_mut<'b>(&'b mut self, s: &'a str) -> &'b mut Value {
        match self.content {
            Value::Compound(ref mut v) => v.get_mut(s).unwrap(),
            _ => unreachable!()
        }
    }
}

impl fmt::Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.header {
            Some(ref h) => write!(f, "TAG_Compound(\"{}\"): {}", h, self.content),
            None => write!(f, "TAG_Compound(): {}", self.content),
        }
    }
}
