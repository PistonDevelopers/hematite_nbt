use std::error;
use std::fmt;
use std::result::Result as StdResult;

use serde;
use nbt;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Nbt(nbt::Error),
    NoRootCompound,
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

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Nbt(_) => "NBT error",
            Error::NoRootCompound => "no root compound",
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        unimplemented!()
    }
}
