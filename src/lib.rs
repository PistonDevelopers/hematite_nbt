//! MC Named Binary Tag type.

#![feature(iter_arith)]
#![cfg_attr(test, feature(test))]

extern crate byteorder;
extern crate flate2;
#[cfg(test)] extern crate test;

/* Re-export the core API from submodules. */
pub use blob::Blob;
pub use error::{Error, Result};
pub use value::Value;

pub mod serialize;

mod blob;
mod error;
mod value;

#[cfg(test)] mod tests;
