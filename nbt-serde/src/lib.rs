#[macro_use] extern crate serde;
extern crate flate2;
extern crate nbt;

#[macro_use] mod macros;

pub use error::{Error, Result};
pub use encode::Encoder;
pub use decode::Decoder;

pub mod error;
pub mod encode;
pub mod decode;
