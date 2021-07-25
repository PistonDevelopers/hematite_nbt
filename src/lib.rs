//! MC Named Binary Tag type.

#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    unused,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::all
)]
#![warn(clippy::pedantic, missing_docs)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::similar_names,
    clippy::pub_enum_variant_names
)]

/* Re-export the core API from submodules. */
pub use blob::Blob;
pub use error::{Error, Result};
pub use value::Value;

#[cfg(feature = "preserve_order")]
pub use indexmap::IndexMap as Map;
#[cfg(not(feature = "preserve_order"))]
pub use std::collections::HashMap as Map;

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
