#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate nbt;
extern crate nbt_serde;

use nbt_serde::encode::to_writer;
use nbt_serde::decode::from_reader;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ByteNbt {
    data: i8,
}

#[test]
fn serialize_byte() {
    let nbt = ByteNbt { data: 100 };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x64,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: ByteNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ShortNbt {
    data: i16,
}

#[test]
fn serialize_short() {
    let nbt = ShortNbt { data: 12345 };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x02,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x30, 0x39,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: ShortNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct IntNbt {
    data: i32,
}

#[test]
fn serialize_int() {
    let nbt = IntNbt { data: 100 };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x03,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x00, 0x00, 0x00, 0x64,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: IntNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct LongNbt {
    data: i64,
}

#[test]
fn serialize_long() {
    let nbt = LongNbt { data: 100 };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x04,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: LongNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FloatNbt {
    data: f32,
}

#[test]
fn serialize_float() {
    let nbt = FloatNbt { data: 20.0 };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x05,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x41, 0xa0, 0x00, 0x00,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: FloatNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DoubleNbt {
    data: f64,
}

#[test]
fn serialize_double() {
    let nbt = DoubleNbt { data: 20.0 };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x06,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x40, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: DoubleNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct StringNbt {
    data: String,
}

#[test]
fn serialize_string() {
    let nbt = StringNbt { data: "Herobrine".to_string() };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

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

    assert_eq!(bytes, dst);

    let read: StringNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BasicListNbt {
    data: Vec<i8>,
}

#[test]
fn serialize_basic_list() {
    let nbt = BasicListNbt { data: vec![1, 2, 3] };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

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

    assert_eq!(bytes, dst);

    let read: BasicListNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BoolNbt {
    data: bool,
}

#[test]
fn serialize_bool() {
    let nbt = BoolNbt { data: true };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x01,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: BoolNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct OptionNbt {
    data: Option<i8>,
}

#[test]
fn serialize_some() {
    let nbt = OptionNbt { data: Some(100) };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x64,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: OptionNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[test]
fn serialize_none() {
    let nbt = OptionNbt { data: None };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            // Not included.
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: OptionNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct UnitNbt {
    data: (),
}

#[test]
fn serialize_unit() {
    let nbt = UnitNbt { data: () };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            // Not included.
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: UnitNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct UnitStructNbt;

#[test]
fn serialize_unit_struct() {
    let nbt = UnitStructNbt;

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: UnitStructNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NewByteNbt(ByteNbt);

#[test]
fn serialize_newtype_struct() {
    let nbt = NewByteNbt(ByteNbt { data: 100 });

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x64,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: NewByteNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NestedByteNbt {
    data: ByteNbt,
}

#[test]
fn serialize_nested() {
    let nbt = NestedByteNbt { data: ByteNbt { data: 100 } };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x0a,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x01,
                    0x00, 0x04,
                    0x64, 0x61, 0x74, 0x61,
                    0x64,
            0x00,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: NestedByteNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NestedUnitStructNbt {
    data: UnitStructNbt,
}

#[test]
fn serialize_nested_unit_struct() {
    let nbt = NestedUnitStructNbt { data: UnitStructNbt };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x0a,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                // No content.
            0x00,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: NestedUnitStructNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NestedNewByteNbt {
    data: NewByteNbt,
}

#[test]
fn serialize_nested_newtype_struct() {
    let nbt = NestedNewByteNbt { data: NewByteNbt(ByteNbt { data: 100 }) };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x0a,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x01,
                    0x00, 0x04,
                    0x64, 0x61, 0x74, 0x61,
                    0x64,
            0x00,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: NestedNewByteNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}
