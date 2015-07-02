use std::collections::HashMap;
use std::fmt;
use std::io;

use byteorder::{ByteOrder, BigEndian, WriteBytesExt, ReadBytesExt};

use error::{Error, Result};

/// Values which can be represented in the Named Binary Tag format.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Value>),
    Compound(HashMap<String, Value>),
    IntArray(Vec<i32>),
}

impl Value {
    /// The type ID of this `Value`, which is a single byte in the range
    /// `0x01` to `0x0b`.
    pub fn id(&self) -> u8 {
        match *self {
            Value::Byte(_)      => 0x01,
            Value::Short(_)     => 0x02,
            Value::Int(_)       => 0x03,
            Value::Long(_)      => 0x04,
            Value::Float(_)     => 0x05,
            Value::Double(_)    => 0x06,
            Value::ByteArray(_) => 0x07,
            Value::String(_)    => 0x08,
            Value::List(_)      => 0x09,
            Value::Compound(_)  => 0x0a,
            Value::IntArray(_)  => 0x0b
        }
    }

    /// A string representation of this tag.
    fn tag_name(&self) -> &str {
        match *self {
            Value::Byte(_)      => "TAG_Byte",
            Value::Short(_)     => "TAG_Short",
            Value::Int(_)       => "TAG_Int",
            Value::Long(_)      => "TAG_Long",
            Value::Float(_)     => "TAG_Float",
            Value::Double(_)    => "TAG_Double",
            Value::ByteArray(_) => "TAG_ByteArray",
            Value::String(_)    => "TAG_String",
            Value::List(_)      => "TAG_List",
            Value::Compound(_)  => "TAG_Compound",
            Value::IntArray(_)  => "TAG_IntArray"
        }
    }

    /// The length of the payload of this `Value`, in bytes.
    pub fn len(&self) -> usize {
        match *self {
            Value::Byte(_)            => 1,
            Value::Short(_)           => 2,
            Value::Int(_)             => 4,
            Value::Long(_)            => 8,
            Value::Float(_)           => 4,
            Value::Double(_)          => 8,
            Value::ByteArray(ref val) => 4 + val.len(), // size + bytes
            Value::String(ref val)    => 2 + val.len(), // size + bytes
            Value::List(ref vals)     => {
                // tag + size + payload for each element
                5 + vals.iter().map(|x| x.len()).fold(0, |acc, item| acc + item)
            },
            Value::Compound(ref vals) => {
                vals.iter().map(|(name, nbt)| {
                    // tag + name + payload for each entry
                    3 + name.len() + nbt.len()
                }).fold(0, |acc, item| acc + item) + 1 // + u8 for the Tag_End
            },
            Value::IntArray(ref val)  => 4 + 4 * val.len(),
        }
    }

    /// Writes the header (that is, the value's type ID and optionally a title)
    /// of this `Value` to an `io::Write` destination.
    pub fn write_header(&self, mut dst: &mut io::Write, title: &str) -> Result<()> {
        try!(dst.write_u8(self.id()));
        try!(dst.write_u16::<BigEndian>(title.len() as u16));
        try!(dst.write_all(title.as_bytes()));
        Ok(())
    }

    /// Writes the payload of this `Value` to an `io::Write` destination.
    pub fn write(&self, mut dst: &mut io::Write) -> Result<()> {
        match *self {
            Value::Byte(val)   => try!(dst.write_i8(val)),
            Value::Short(val)  => try!(dst.write_i16::<BigEndian>(val)),
            Value::Int(val)    => try!(dst.write_i32::<BigEndian>(val)),
            Value::Long(val)   => try!(dst.write_i64::<BigEndian>(val)),
            Value::Float(val)  => try!(dst.write_f32::<BigEndian>(val)),
            Value::Double(val) => try!(dst.write_f64::<BigEndian>(val)),
            Value::ByteArray(ref vals) => {
                try!(dst.write_i32::<BigEndian>(vals.len() as i32));
                for &byte in vals {
                    try!(dst.write_i8(byte));
                }
            },
            Value::String(ref val) => {
                try!(dst.write_u16::<BigEndian>(val.len() as u16));
                try!(dst.write_all(val.as_bytes()));
            },
            Value::List(ref vals) => {
                // This is a bit of a trick: if the list is empty, don't bother
                // checking its type.
                if vals.len() == 0 {
                    try!(dst.write_u8(1));
                    try!(dst.write_i32::<BigEndian>(0));
                } else {
                    // Otherwise, use the first element of the list.
                    let first_id = vals[0].id();
                    try!(dst.write_u8(first_id));
                    try!(dst.write_i32::<BigEndian>(vals.len() as i32));
                    for nbt in vals {
                        // Ensure that all of the tags are the same type.
                        if nbt.id() != first_id {
                            return Err(Error::HeterogeneousList);
                        }
                        try!(nbt.write(dst));
                    }
                }
            },
            Value::Compound(ref vals)  => {
                for (name, ref nbt) in vals {
                    // Write the header for the tag.
                    try!(nbt.write_header(dst, &name));
                    try!(nbt.write(dst));
                }
                // Write the marker for the end of the Compound.
                try!(dst.write_u8(0x00))
            }
            Value::IntArray(ref vals) => {
                try!(dst.write_i32::<BigEndian>(vals.len() as i32));
                for &nbt in vals {
                    try!(dst.write_i32::<BigEndian>(nbt));
                }
            },
        };
        Ok(())
    }

    /// Reads any valid `Value` header (that is, a type ID and a title of
    /// arbitrary UTF-8 bytes) from an `io::Read` source.
    pub fn read_header(mut src: &mut io::Read) -> Result<(u8, String)> {
        let id = try!(src.read_u8());
        if id == 0x00 { return Ok((0x00, "".to_string())); }
        // Extract the name.
        let name_len = try!(src.read_u16::<BigEndian>());
        let name = if name_len != 0 {
            try!(read_utf8(src, name_len as usize))
        } else {
            "".to_string()
        };
        Ok((id, name))
    }

    /// Reads the payload of an `Value` with a given type ID from an
    /// `io::Read` source.
    pub fn from_reader(id: u8, mut src: &mut io::Read) -> Result<Value> {
        match id {
            0x01 => Ok(Value::Byte(try!(src.read_i8()))),
            0x02 => Ok(Value::Short(try!(src.read_i16::<BigEndian>()))),
            0x03 => Ok(Value::Int(try!(src.read_i32::<BigEndian>()))),
            0x04 => Ok(Value::Long(try!(src.read_i64::<BigEndian>()))),
            0x05 => Ok(Value::Float(try!(src.read_f32::<BigEndian>()))),
            0x06 => Ok(Value::Double(try!(src.read_f64::<BigEndian>()))),
            0x07 => { // ByteArray
                let len = try!(src.read_i32::<BigEndian>()) as usize;
                let mut buf = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(try!(src.read_i8()));
                }
                Ok(Value::ByteArray(buf))
            },
            0x08 => { // String
                let len = try!(src.read_u16::<BigEndian>()) as usize;
                Ok(Value::String(try!(read_utf8(src, len))))
            },
            0x09 => { // List
                let id = try!(src.read_u8());
                let len = try!(src.read_i32::<BigEndian>()) as usize;
                let mut buf = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(try!(Value::from_reader(id, src)));
                }
                Ok(Value::List(buf))
            },
            0x0a => { // Compound
                let mut buf = HashMap::new();
                loop {
                    let (id, name) = try!(Value::read_header(src));
                    if id == 0x00 { break; }
                    let tag = try!(Value::from_reader(id, src));
                    buf.insert(name, tag);
                }
                Ok(Value::Compound(buf))
            },
            0x0b => { // IntArray
                let len = try!(src.read_i32::<BigEndian>()) as usize;
                let mut buf = Vec::with_capacity(len);
                for _ in 0..len {
                    buf.push(try!(src.read_i32::<BigEndian>()));
                }
                Ok(Value::IntArray(buf))
            },
            e => Err(Error::InvalidTypeId(e))
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Byte(v)   => write!(f, "{}", v),
            Value::Short(v)  => write!(f, "{}", v),
            Value::Int(v)    => write!(f, "{}", v),
            Value::Long(v)   => write!(f, "{}", v),
            Value::Float(v)  => write!(f, "{}", v),
            Value::Double(v) => write!(f, "{}", v),
            Value::ByteArray(ref v) => write!(f, "{:?}", v),
            Value::String(ref v) => write!(f, "{}", v),
            Value::List(ref v) => {
                if v.len() == 0 {
                    write!(f, "zero entries")
                } else {
                    try!(write!(f, "{} entries of type {}\n{{\n", v.len(), v[0].tag_name()));
                    for tag in v {
                        try!(write!(f, "{}(None): {}\n", tag.tag_name(), tag));
                    }
                    try!(write!(f, "}}"));
                    Ok(())
                }
            }
            Value::Compound(ref v) => {
                try!(write!(f, "{} entry(ies)\n{{\n", v.len()));
                for (name, tag) in v {
                    try!(write!(f, "{}(\"{}\"): {}\n", tag.tag_name(), name, tag));
                }
                try!(write!(f, "}}"));
                Ok(())
            }
            Value::IntArray(ref v) => write!(f, "{:?}", v)
        }
    }
}

impl From<i8> for Value {
    fn from(t: i8) -> Value { Value::Byte(t) }
}

impl From<i16> for Value {
    fn from(t: i16) -> Value { Value::Short(t) }
}

impl From<i32> for Value {
    fn from(t: i32) -> Value { Value::Int(t) }
}

impl From<i64> for Value {
    fn from(t: i64) -> Value { Value::Long(t) }
}

impl From<f32> for Value {
    fn from(t: f32) -> Value { Value::Float(t) }
}

impl From<f64> for Value {
    fn from(t: f64) -> Value { Value::Double(t) }
}

impl<'a> From<&'a str> for Value {
    fn from(t: &'a str) -> Value { Value::String(t.into()) }
}

impl From<String> for Value {
    fn from(t: String) -> Value { Value::String(t) }
}

impl From<Vec<i8>> for Value {
    fn from(t: Vec<i8>) -> Value { Value::ByteArray(t) }
}

impl<'a> From<&'a [i8]> for Value {
    fn from(t: &'a [i8]) -> Value { Value::ByteArray(t.into()) }
}

impl From<Vec<i32>> for Value {
    fn from(t: Vec<i32>) -> Value { Value::IntArray(t) }
}

impl<'a> From<&'a [i32]> for Value {
    fn from(t: &'a [i32]) -> Value { Value::IntArray(t.into()) }
}

/// Returns a `Vec<u8>` containing the next `len` bytes in the reader.
///
/// Adapted from `byteorder::read_full`.
fn read_utf8(mut src: &mut io::Read, len: usize) -> Result<String> {
    let mut bytes = vec![0; len];
    let mut n_read = 0usize;
    while n_read < bytes.len() {
        match try!(src.read(&mut bytes[n_read..])) {
            0 => return Err(Error::IncompleteNbtValue),
            n => n_read += n
        }
    }
    Ok(try!(String::from_utf8(bytes)))
}
