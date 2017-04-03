use std::error;
use std::fmt;
use std::io;
use std::result::Result as StdResult;

use serde;
use nbt;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Nbt(nbt::Error),
    Io(io::Error),
    NoRootCompound,
    UnrepresentableType(&'static str),
    Message(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> StdResult<(), fmt::Error> {
        error::Error::description(self).fmt(f)
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
            Error::Nbt(_) => "NBT error",
            Error::Io(_) => "IO error",
            Error::NoRootCompound => "no root compound",
            Error::UnrepresentableType(_) => "unrepresentable type",
            Error::Message(ref msg) => &msg[..],
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Message(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Message(msg.to_string())
    }
}
