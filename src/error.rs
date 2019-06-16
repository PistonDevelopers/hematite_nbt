use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::io::ErrorKind::InvalidInput;
use std::result::Result as StdResult;

#[cfg(feature = "serde")]
use serde;

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
    /// Wraps errors emitted during (de-)serialization with `serde`.
    #[cfg(feature = "serde")]
    Serde(String),
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
    /// An error encountered when deserializing a boolean from an invalid byte.
    NonBooleanByte(i8),
    /// An error encountered when serializing a Rust type with no meaningful NBT
    /// representation.
    UnrepresentableType(&'static str),
    /// An error encountered when trying to (de)serialize a map key with a
    /// non-string type.
    NonStringMapKey,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::IoError(ref e)     => e.fmt(f),
            #[cfg(feature = "serde")]
            &Error::Serde(ref msg)     => write!(f, "{}", msg),
            &Error::InvalidTypeId(t)   => write!(f, "invalid NBT tag byte: '{}'", t),
            &Error::TagMismatch(a, b)  => write!(f, "encountered NBT tag '{}' but expected '{}'", a, b),
            &Error::NonBooleanByte(b)  => write!(f, "encountered a byte value '{}' inside a boolean", b),
            &Error::UnexpectedField(ref name) => write!(f, "encountered an unexpected field '{}'", name),
            &Error::UnrepresentableType(ref name) => write!(f, "encountered type '{}', which has no meaningful NBT representation", name),
            // Static messages should suffice for the remaining errors.
            other => write!(f, "{}", other.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref e)     => e.description(),
            #[cfg(feature = "serde")]
            Error::Serde(ref msg)     => &msg[..],
            Error::InvalidTypeId(_)   => "invalid NBT tag byte",
            Error::HeterogeneousList  => "values in NBT Lists must be homogeneous",
            Error::NoRootCompound     => "the root value must be Compound-like (tag = 0x0a)",
            Error::InvalidUtf8        => "a string is not valid UTF-8",
            Error::IncompleteNbtValue => "data does not represent a complete NbtValue",
            Error::NonStringMapKey    => "encountered a non-string map key",
            Error::TagMismatch(_, _)  => "encountered one NBT tag but expected another",
            Error::UnexpectedField(_) => "encountered an unexpected field",
            Error::NonBooleanByte(_)  => "encountered a non-boolean byte value inside a boolean",
            Error::UnrepresentableType(_) => "encountered a type with no meaningful NBT representation",
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
                    InvalidUtf8, IncompleteNbtValue, TagMismatch, UnexpectedField, NonBooleanByte,
                    UnrepresentableType};

        match (self, other) {
            (&IoError(_), &IoError(_))                 => true,
            #[cfg(feature = "serde")]
            (&Error::Serde(_), &Error::Serde(_))       => true,
            (&InvalidTypeId(a), &InvalidTypeId(b))     => a == b,
            (&HeterogeneousList, &HeterogeneousList)   => true,
            (&NoRootCompound, &NoRootCompound)         => true,
            (&InvalidUtf8, &InvalidUtf8)               => true,
            (&IncompleteNbtValue, &IncompleteNbtValue) => true,
            (&TagMismatch(a, b), &TagMismatch(c, d))   => a == c && b == d,
            (&UnexpectedField(ref a), &UnexpectedField(ref b)) => a == b,
            (&NonBooleanByte(a), &NonBooleanByte(b))   => a == b,
            (&UnrepresentableType(ref a), &UnrepresentableType(ref b)) => a == b,
            _ => false
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        use std::io::ErrorKind;

        if e.kind() == ErrorKind::UnexpectedEof {
            return Error::IncompleteNbtValue;
        }
        Error::IoError(e)
    }
}

impl From<cesu8::Cesu8DecodingError> for Error {
    fn from(_: cesu8::Cesu8DecodingError) -> Error {
        Error::InvalidUtf8
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

#[cfg(feature = "serde")]
impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Serde(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Serde(msg.to_string())
    }
}
