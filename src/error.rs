use std::io;
use std::io::ErrorKind::InvalidInput;
use std::string;

use byteorder;

/// Errors that may be encountered when constructing, parsing, or encoding
/// `NbtValue` and `NbtBlob` objects.
///
/// `Error`s can be seamlessly converted to more general `io::Error` objects
/// using the `FromError` trait.
#[derive(Debug)]
pub enum Error {
    /// Wraps errors emitted by methods during I/O operations.
    IoError(io::Error),
    /// An error for when an unknown type ID is encountered in decoding NBT
    /// binary representations. Includes the ID in question.
    InvalidTypeId(u8),
    /// An error emitted when trying to create `NbtBlob`s with incorrect lists.
    HeterogeneousList,
    /// An error for when NBT binary representations do not begin with an
    /// `NbtValue::Compound`.
    NoRootCompound,
    /// An error for when NBT binary representations contain invalid UTF-8
    /// strings.
    InvalidUtf8,
    /// An error for when NBT binary representations are missing end tags,
    /// contain fewer bytes than advertised, or are otherwise incomplete.
    IncompleteNbtValue,
}

// Implement PartialEq manually, since std::io::Error is not PartialEq.
impl PartialEq<Error> for Error {
    fn eq(&self, other: &Error) -> bool {
        use Error::{IoError, InvalidTypeId, HeterogeneousList, NoRootCompound,
                       InvalidUtf8, IncompleteNbtValue};

        match (self, other) {
            (&IoError(_), &IoError(_))                 => true,
            (&InvalidTypeId(a), &InvalidTypeId(b))     => a == b,
            (&HeterogeneousList, &HeterogeneousList)   => true,
            (&NoRootCompound, &NoRootCompound)         => true,
            (&InvalidUtf8, &InvalidUtf8)               => true,
            (&IncompleteNbtValue, &IncompleteNbtValue) => true,
            _ => false
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(_: string::FromUtf8Error) -> Error {
        Error::InvalidUtf8
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        match err {
            // Promote byteorder's I/O errors to Error's I/O errors.
            byteorder::Error::Io(e) => Error::IoError(e),
            // Anything else is really an incomplete value.
            byteorder::Error::UnexpectedEOF => Error::IncompleteNbtValue
        }
    }
}

impl From<Error> for io::Error {
    fn from(e: Error) -> io::Error {
        match e {
            Error::IoError(e) => e,
            Error::InvalidTypeId(id) =>
                io::Error::new(InvalidInput, &format!("invalid NBT value type: {}", id)[..]),
            Error::HeterogeneousList =>
                io::Error::new(InvalidInput, "List values must be homogeneous"),
            Error::NoRootCompound =>
                io::Error::new(InvalidInput, "root value must be a Compound (0x0a)"),
            Error::InvalidUtf8 =>
                io::Error::new(InvalidInput, "string is not UTF-8"),
            Error::IncompleteNbtValue =>
                io::Error::new(InvalidInput, "data does not represent a complete NbtValue"),
        }
    }
}
