#[macro_use] extern crate serde;
extern crate nbt;

pub use error::{Error, Result};
pub use encode::Encoder;
pub use decode::Decoder;

pub mod error;
pub mod encode;
pub mod decode;
