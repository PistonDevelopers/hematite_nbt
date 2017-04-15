use std::error;
use std::fmt;
use std::io;
use std::result;

use serde;
use nbt;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Nbt(nbt::Error),
    Io(io::Error),
    Serde(String),
    NoRootCompound,
    UnknownTag(u8),
    NonBooleanByte(i8),
    UnexpectedTag(u8, u8),
    UnrepresentableType(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Error::Nbt(ref err) => fmt::Display::fmt(err, f),
            Error::Io(ref err) => fmt::Display::fmt(err, f),
            Error::Serde(ref msg) => f.write_str(msg),
            Error::NoRootCompound => {
                f.write_str("all values must have a root compound")
            },
            Error::UnknownTag(t) => {
                write!(f, "unknown tag: {}", t)
            },
            Error::NonBooleanByte(b) => {
                write!(f, "boolean bytes must be 0 or 1, found {}", b)
            },
            Error::UnexpectedTag(a, b) => {
                write!(f, "unexpected tag: {}, expecting: {}", a, b)
            },
            Error::UnrepresentableType(t) => {
                write!(f, "cannot represent {} in NBT format", t)
            },
        }
    }
}

impl From<nbt::Error> for Error {
    fn from(err: nbt::Error) -> Error {
        Error::Nbt(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "IO error",
            Error::Nbt(_) => "NBT error",
            Error::Serde(ref msg) => &msg[..],
            Error::NoRootCompound => "all values must have a root compound",
            Error::UnknownTag(_) => "unknown tag",
            Error::NonBooleanByte(_) =>
                "encountered a non-0 or 1 byte for a boolean",
            Error::UnexpectedTag(_, _) => "unexpected tag",
            Error::UnrepresentableType(_) => "unrepresentable type",
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Serde(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Serde(msg.to_string())
    }
}
