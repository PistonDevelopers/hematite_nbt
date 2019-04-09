//! MC Named Binary Tag type.

extern crate byteorder;
extern crate flate2;
extern crate linked_hash_map;

/* Re-export the core API from submodules. */
pub use blob::Blob;
pub use error::{Error, Result};
pub use value::Value;

#[cfg(feature = "serde")]
#[doc(inline)]
pub use de::{from_reader, from_gzip_reader, from_zlib_reader};
#[cfg(feature = "serde")]
#[doc(inline)]
pub use ser::{to_writer, to_gzip_writer, to_zlib_writer};

mod raw;
mod blob;
mod error;
mod value;

#[cfg(feature = "serde")] #[macro_use] extern crate serde;

#[cfg(feature = "serde")] #[macro_use] mod macros;
#[cfg(feature = "serde")] pub mod de;
#[cfg(feature = "serde")] pub mod ser;

#[cfg(test)] mod tests;
