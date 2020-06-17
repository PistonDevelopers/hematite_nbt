//! MC Named Binary Tag type.

extern crate byteorder;
extern crate cesu8;
extern crate flate2;

/* Re-export the core API from submodules. */
pub use blob::Blob;
pub use error::{Error, Result};
pub use value::Value;

#[cfg(feature = "serde")]
#[doc(inline)]
pub use de::{from_gzip_reader, from_reader, from_zlib_reader};
#[cfg(feature = "serde")]
#[doc(inline)]
pub use ser::{i32_array, i64_array, i8_array};
#[cfg(feature = "serde")]
#[doc(inline)]
pub use ser::{to_gzip_writer, to_writer, to_zlib_writer};

mod blob;
mod error;
mod raw;
mod value;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(feature = "serde")]
#[macro_use]
mod macros;
#[cfg(feature = "serde")]
pub mod de;
#[cfg(feature = "serde")]
pub mod ser;

#[cfg(test)]
mod tests;
