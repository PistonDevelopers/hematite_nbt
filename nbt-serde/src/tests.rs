#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate nbt;
extern crate nbt_serde;

use serde::Serialize;

use nbt_serde::encode::Serializer;

#[derive(Serialize)]
struct ByteNbt {
    data: i8,
}

#[test]
fn serialize_byte() {
    let nbt = ByteNbt { data: 100 };

    let mut dst = Vec::new();
    nbt.serialize(&mut Serializer::new(&mut dst, None)).ok().unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x64,
        0x00
    ];

    assert_eq!(bytes, dst)
}

#[derive(Serialize)]
struct ShortNbt {
    data: i16,
}

#[test]
fn serialize_short() {
    let nbt = ShortNbt { data: 12345 };

    let mut dst = Vec::new();
    nbt.serialize(&mut Serializer::new(&mut dst, None)).ok().unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x02,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x30, 0x39,
        0x00
    ];

    assert_eq!(bytes, dst)
}

#[derive(Serialize)]
struct IntNbt {
    data: i32,
}

#[test]
fn serialize_int() {
    let nbt = IntNbt { data: 100 };

    let mut dst = Vec::new();
    nbt.serialize(&mut Serializer::new(&mut dst, None)).ok().unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x03,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x00, 0x00, 0x00, 0x64,
        0x00
    ];

    assert_eq!(bytes, dst)
}

#[derive(Serialize)]
struct LongNbt {
    data: i64,
}

#[test]
fn serialize_long() {
    let nbt = LongNbt { data: 100 };

    let mut dst = Vec::new();
    nbt.serialize(&mut Serializer::new(&mut dst, None)).ok().unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x04,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64,
        0x00
    ];

    assert_eq!(bytes, dst)
}

#[derive(Serialize)]
struct FloatNbt {
    data: f32,
}

#[test]
fn serialize_float() {
    let nbt = FloatNbt { data: 20.0 };

    let mut dst = Vec::new();
    nbt.serialize(&mut Serializer::new(&mut dst, None)).ok().unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x05,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x41, 0xa0, 0x00, 0x00,
        0x00
    ];

    assert_eq!(bytes, dst)
}

#[derive(Serialize)]
struct DoubleNbt {
    data: f64,
}

#[test]
fn serialize_double() {
    let nbt = DoubleNbt { data: 20.0 };

    let mut dst = Vec::new();
    nbt.serialize(&mut Serializer::new(&mut dst, None)).ok().unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x06,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x40, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00
    ];

    assert_eq!(bytes, dst)
}

#[derive(Serialize)]
struct StringNbt {
    data: String,
}

#[test]
fn serialize_string() {
    let nbt = StringNbt { data: "Herobrine".to_string() };

    let mut dst = Vec::new();
    nbt.serialize(&mut Serializer::new(&mut dst, None)).ok().unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x08,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x00, 0x09,
                0x48, 0x65, 0x72, 0x6f, 0x62, 0x72, 0x69, 0x6e, 0x65,
        0x00
    ];

    assert_eq!(bytes, dst)
}

#[derive(Serialize)]
struct BasicListNbt {
    data: Vec<i8>,
}

#[test]
fn serialize_basic_list() {
    let nbt = BasicListNbt { data: vec![1, 2, 3] };

    let mut dst = Vec::new();
    nbt.serialize(&mut Serializer::new(&mut dst, None)).ok().unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x09,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x01, // List type.
                0x00, 0x00, 0x00, 0x03, // Length.
                0x01, 0x02, 0x03, // Content.
        0x00
    ];

    assert_eq!(bytes, dst)
}

#[derive(Serialize)]
struct BoolNbt {
    data: bool,
}

#[test]
fn serialize_bool() {
    let nbt = BoolNbt { data: true };

    let mut dst = Vec::new();
    nbt.serialize(&mut Serializer::new(&mut dst, None)).ok().unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x01,
        0x00
    ];

    assert_eq!(bytes, dst)
}

#[derive(Serialize)]
struct NewByteNbt(ByteNbt);

#[test]
fn serialize_newtype_struct() {
    let nbt = NewByteNbt(ByteNbt { data: 100 });

    let mut dst = Vec::new();
    nbt.serialize(&mut Serializer::new(&mut dst, None)).ok().unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x64,
        0x00
    ];

    assert_eq!(bytes, dst)
}
