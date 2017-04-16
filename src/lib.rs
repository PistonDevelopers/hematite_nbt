//! MC Named Binary Tag type.

extern crate byteorder;
extern crate flate2;

/* Re-export the core API from submodules. */
pub use blob::Blob;
pub use error::{Error, Result};
pub use value::Value;

pub mod serialize;
pub mod raw;

mod blob;
mod error;
mod value;

#[cfg(test)] mod tests;
