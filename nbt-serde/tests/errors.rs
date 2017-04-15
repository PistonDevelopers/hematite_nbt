#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate nbt;
extern crate nbt_serde;

use nbt_serde::error::{Error, Result};
use nbt_serde::encode::to_writer;
use nbt_serde::decode::from_reader;

#[test]
fn no_root_compound() {
    let nbt: i8 = 100;

    let mut dst = Vec::new();
    let write = to_writer(&mut dst, &nbt, None);

    assert!(write.is_err());
    match write.unwrap_err() {
        Error::NoRootCompound => (),
        _ => panic!("encountered an unexpected error"),
    }
}

#[derive(Debug, Deserialize)]
struct ByteNbt {
    data: i8,
}

#[test]
fn incomplete_nbt() {
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x01
    ];

    let read: Result<ByteNbt> = from_reader(&bytes[..]);

    assert!(read.is_err());
    match read.unwrap_err() {
        Error::Nbt(err) =>
            assert_eq!(err.to_string(),
                       "data does not represent a complete NbtValue"),
        _ => panic!("encountered an unexpected error"),
    }
}

#[test]
fn unknown_tag() {
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x0f,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x01,
        0x00
    ];

    let read: Result<ByteNbt> = from_reader(&bytes[..]);

    assert!(read.is_err());
    match read.unwrap_err() {
        Error::UnknownTag(t) => assert_eq!(t, 0x0f),
        _ => panic!("encountered an unexpected error"),
    }
}

#[test]
fn deserialized_wrong_type() {
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x08,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x00, 0x00,
        0x00
    ];

    let read: Result<ByteNbt> = from_reader(&bytes[..]);

    assert!(read.is_err());
    match read.unwrap_err() {
        Error::Serde(msg) =>
            assert_eq!(&msg, "invalid type: string \"\", expected i8"),
        _ => panic!("encountered an unexpected error"),
    }
}

#[derive(Debug, Deserialize)]
struct BoolNbt {
    data: bool,
}

#[test]
fn non_boolean_byte() {
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x02,
        0x00
    ];

    let read: Result<BoolNbt> = from_reader(&bytes[..]);

    assert!(read.is_err());
    match read.unwrap_err() {
        Error::NonBooleanByte(v) => assert_eq!(v, 0x02),
        _ => panic!("encountered an unexpected error"),
    }
}
