use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::io::ErrorKind::InvalidInput;
use std::result::Result as StdResult;
use std::string;

use byteorder;

/// A convenient alias type for results when reading/writing the Named Binary
/// Tag format.
pub type Result<T> = StdResult<T, Error>;

/// Errors that may be encountered when constructing, parsing, or encoding
/// `NbtValue` and `NbtBlob` objects.
///
/// `Error`s can be seamlessly converted to more general `io::Error` objects
/// using `std::convert::From::from()`.
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
    /// An error encountered when parsing NBT binary representations, where
    /// deserialization encounters a different tag than expected.
    TagMismatch(u8, u8),
    /// An error encountered when parsing NBT binary representations, where
    /// deserialization encounters a field name it is not expecting.
    UnexpectedField(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::IoError(ref e) => e.fmt(f),
            other                 => write!(f, "{}", other.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref e)     => e.description(),
            Error::InvalidTypeId(_)   => "invalid NBT tag byte",
            Error::HeterogeneousList  => "values in NBT Lists must be homogeneous",
            Error::NoRootCompound     => "the root value must be Compound-like (tag = 0x0a)",
            Error::InvalidUtf8        => "a string is not valid UTF-8",
            Error::IncompleteNbtValue => "data does not represent a complete NbtValue",
            Error::TagMismatch(_, _)  => "encountered one NBT tag but expected another",
            Error::UnexpectedField(_) => "encountered an unexpected field",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::IoError(ref e) => e.cause(),
            _ => None
        }
    }
}

// Implement PartialEq manually, since std::io::Error is not PartialEq.
impl PartialEq<Error> for Error {
    fn eq(&self, other: &Error) -> bool {
        use Error::{IoError, InvalidTypeId, HeterogeneousList, NoRootCompound,
                    InvalidUtf8, IncompleteNbtValue, TagMismatch, UnexpectedField};

        match (self, other) {
            (&IoError(_), &IoError(_))                 => true,
            (&InvalidTypeId(a), &InvalidTypeId(b))     => a == b,
            (&HeterogeneousList, &HeterogeneousList)   => true,
            (&NoRootCompound, &NoRootCompound)         => true,
            (&InvalidUtf8, &InvalidUtf8)               => true,
            (&IncompleteNbtValue, &IncompleteNbtValue) => true,
            (&TagMismatch(a, b), &TagMismatch(c, d))   => a == c && b == d,
            (&UnexpectedField(ref a), &UnexpectedField(ref b)) => a == b,
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
                io::Error::new(InvalidInput, &format!("invalid NBT tag byte: {}", id)[..]),
            Error::TagMismatch(a, b) =>
                io::Error::new(InvalidInput, &format!("encountered NBT tag {} \
                                                       but expected {}", a, b)[..]),
            Error::UnexpectedField(f) =>
                io::Error::new(InvalidInput, &format!("encountered unexpected field \
                                                       with name {}", f)[..]),
            other => io::Error::new(InvalidInput, other.description()),
        }
    }
}
