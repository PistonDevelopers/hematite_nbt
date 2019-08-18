#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate nbt;

use std::collections::HashMap;

use nbt::de::from_reader;
use nbt::ser::to_writer;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ByteNbt {
    data: i8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PrimitiveNbt {
    byte: i8,
    short: i16,
    int: i32,
    long: i64,
    float: f32,
    double: f64,
    string: String,
}

#[test]
fn roundtrip_primitives() {
    let nbt = PrimitiveNbt {
        byte: 100,
        short: 100,
        int: 100,
        long: 100,
        float: 20.0,
        double: 20.0,
        string: "Herobrine".to_string(),
    };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, Some("data")).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x04, // Header: "data"
            0x64, 0x61, 0x74, 0x61,
            0x01,
                0x00, 0x04,
                0x62, 0x79, 0x74, 0x65,
                0x64,
            0x02,
                0x00, 0x05,
                0x73, 0x68, 0x6f, 0x72, 0x74,
                0x00, 0x64,
            0x03,
                0x00, 0x03,
                0x69, 0x6e, 0x74,
                0x00, 0x00, 0x00, 0x64,
            0x04,
                0x00, 0x04,
                0x6c, 0x6f, 0x6e, 0x67,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64,
            0x05,
                0x00, 0x05,
                0x66, 0x6c, 0x6f, 0x61, 0x74,
                0x41, 0xa0, 0x00, 0x00,
            0x06,
                0x00, 0x06,
                0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65,
                0x40, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x08,
                0x00, 0x06,
                0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
                0x00, 0x09,
                0x48, 0x65, 0x72, 0x6f, 0x62, 0x72, 0x69, 0x6e, 0x65,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: PrimitiveNbt = from_reader(&bytes[..]).unwrap();
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

#[test]
fn serialize_empty_list() {
    let nbt = BasicListNbt { data: vec!() };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x09,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x00, // Empty list type.
                0x00, 0x00, 0x00, 0x00, // Length.
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: BasicListNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NestedListNbt {
    data: Vec<Vec<i16>>,
}

#[test]
fn serialize_nested_list() {
    let nbt = NestedListNbt { data: vec!(vec!(1, 2), vec!(3, 4)) };

    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x09,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x09, // Also a list.
                0x00, 0x00, 0x00, 0x02,
                0x02, // First list has type short.
                0x00, 0x00, 0x00, 0x02,
                0x00, 0x01, 0x00, 0x02,
                0x02, // Second list has type short.
                0x00, 0x00, 0x00, 0x02,
                0x00, 0x03, 0x00, 0x04,
        0x00
    ];

    assert_eq!(bytes, dst);

    let read: NestedListNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[test]
fn deserialize_byte_array() {
    let nbt = BasicListNbt { data: vec![1, 2, 3] };

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x07,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x00, 0x00, 0x00, 0x03, // Length.
                0x01, 0x02, 0x03, // Content.
        0x00
    ];

    let read: BasicListNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct IntListNbt {
    data: Vec<i32>,
}

#[test]
fn deserialize_int_array() {
    let nbt = IntListNbt { data: vec![1, 2, 3] };

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x0b,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x00, 0x00, 0x00, 0x03, // Length.
                // Content.
                0x00, 0x00, 0x00, 0x01,
                0x00, 0x00, 0x00, 0x02,
                0x00, 0x00, 0x00, 0x03,
        0x00
    ];

    let read: IntListNbt = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct LongListNbt {
    data: Vec<i64>,
}

#[test]
fn deserialize_long_array() {
    let nbt = LongListNbt { data: vec![1, 2, 3] };

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x0c,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x00, 0x00, 0x00, 0x03, // Length.
                // Content.
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
        0x00
    ];

    let read: LongListNbt = from_reader(&bytes[..]).unwrap();
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

#[test]
fn serialize_hashmap() {
    let mut nbt = HashMap::new();
    nbt.insert("data".to_string(), 100i8);

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

    let read: HashMap<String,i8> = from_reader(&bytes[..]).unwrap();
    assert_eq!(read, nbt)
}
