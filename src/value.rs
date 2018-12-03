use std::collections::HashMap;
use std::fmt;
use std::io;

use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

use error::{Error, Result};
use raw;

/// Values which can be represented in the Named Binary Tag format.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
    LongArray(Vec<i64>),
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
            Value::IntArray(_)  => 0x0b,
            Value::LongArray(_) => 0x0c,
        }
    }

    /// A string representation of this tag.
    pub fn tag_name(&self) -> &str {
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
            Value::IntArray(_)  => "TAG_IntArray",
            Value::LongArray(_) => "TAG_LongArray",
        }
    }

    /// Writes the payload of this `Value` to an `io::Write` destination.
    pub fn to_writer<W>(&self, mut dst: &mut W) -> Result<()>
        where W: io::Write
    {
        match *self {
            Value::Byte(val)   => raw::write_bare_byte(dst, val),
            Value::Short(val)  => raw::write_bare_short(dst, val),
            Value::Int(val)    => raw::write_bare_int(dst, val),
            Value::Long(val)   => raw::write_bare_long(dst, val),
            Value::Float(val)  => raw::write_bare_float(dst, val),
            Value::Double(val) => raw::write_bare_double(dst, val),
            Value::ByteArray(ref vals) => raw::write_bare_byte_array(dst, &vals[..]),
            Value::String(ref val) => raw::write_bare_string(dst, &val),
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
                        try!(nbt.to_writer(dst));
                    }
                }
                Ok(())
            },
            Value::Compound(ref vals)  => {
                for (name, ref nbt) in vals {
                    // Write the header for the tag.
                    dst.write_u8(nbt.id())?;
                    raw::write_bare_string(dst, name)?;
                    try!(nbt.to_writer(dst));
                }
                raw::close_nbt(&mut dst)
            },
            Value::IntArray(ref vals) => raw::write_bare_int_array(dst, &vals[..]),
            Value::LongArray(ref vals) => raw::write_bare_long_array(dst, &vals[..]),
        }
    }

    /// Reads the payload of an `Value` with a given type ID from an
    /// `io::Read` source.
    pub fn from_reader<R>(id: u8, src: &mut R) -> Result<Value>
        where R: io::Read
    {
        match id {
            0x01 => Ok(Value::Byte(raw::read_bare_byte(src)?)),
            0x02 => Ok(Value::Short(raw::read_bare_short(src)?)),
            0x03 => Ok(Value::Int(raw::read_bare_int(src)?)),
            0x04 => Ok(Value::Long(raw::read_bare_long(src)?)),
            0x05 => Ok(Value::Float(raw::read_bare_float(src)?)),
            0x06 => Ok(Value::Double(raw::read_bare_double(src)?)),
            0x07 => Ok(Value::ByteArray(raw::read_bare_byte_array(src)?)),
            0x08 => Ok(Value::String(raw::read_bare_string(src)?)),
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
                    let (id, name) = try!(raw::emit_next_header(src));
                    if id == 0x00 { break; }
                    let tag = try!(Value::from_reader(id, src));
                    buf.insert(name, tag);
                }
                Ok(Value::Compound(buf))
            },
            0x0b => Ok(Value::IntArray(raw::read_bare_int_array(src)?)),
            0x0c => Ok(Value::LongArray(raw::read_bare_long_array(src)?)),
            e => Err(Error::InvalidTypeId(e))
        }
    }

    pub fn print(&self, f: &mut fmt::Formatter, offset: usize) -> fmt::Result {
        match *self {
            Value::Byte(v)   => write!(f, "{}", v),
            Value::Short(v)  => write!(f, "{}", v),
            Value::Int(v)    => write!(f, "{}", v),
            Value::Long(v)   => write!(f, "{}", v),
            Value::Float(v)  => write!(f, "{}", v),
            Value::Double(v) => write!(f, "{}", v),
            Value::ByteArray(ref v) => write!(f, "{:?}", v),
            Value::String(ref v) => write!(f, "{}", v),
            Value::IntArray(ref v) => write!(f, "{:?}", v),
            Value::LongArray(ref v) => write!(f, "{:?}", v),
            Value::List(ref v) => {
                if v.len() == 0 {
                    write!(f, "zero entries")
                } else {
                    write!(f, "{} entries of type {}\n{:>width$}\n", v.len(), v[0].tag_name(), "{", width = offset + 1)?;
                    for tag in v {
                        let new_offset = offset + 2;
                        write!(f, "{:>width$}(None): ", tag.tag_name(), width = new_offset + tag.tag_name().len())?;
                        tag.print(f, new_offset)?;
                        write!(f, "\n")?;
                    }
                    write!(f, "{:>width$}", "}", width = offset + 1)
                }
            }
            Value::Compound(ref v) => {
                write!(f, "{} entry(ies)\n{:>width$}\n", v.len(), "{", width = offset + 1)?;
                for (name, tag) in v {
                    let new_offset = offset + 2;
                    write!(f, "{:>width$}({}): ", tag.tag_name(), name, width = new_offset + tag.tag_name().len())?;
                    tag.print(f, new_offset)?;
                    write!(f, "\n")?;
                }
                write!(f, "{:>width$}", "}", width = offset + 1)
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print(f, 0)
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

impl From<Vec<i64>> for Value {
    fn from(t: Vec<i64>) -> Value { Value::LongArray(t) }
}

impl<'a> From<&'a [i64]> for Value {
    fn from(t: &'a [i64]) -> Value { Value::LongArray(t.into()) }
}
