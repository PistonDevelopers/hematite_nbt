#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate nbt;

use std::collections::HashMap;

use serde::{Serialize, Serializer};

/// Helper function that asserts data of type T can be serialized into and
/// deserialized from `bytes`. `name` is an optional header for the top-level
/// NBT compound.
fn assert_roundtrip_eq<T>(nbt: T, bytes: &[u8], name: Option<&str>)
where
    for<'de> T: serde::Serialize + serde::Deserialize<'de> + PartialEq + std::fmt::Debug,
{
    let mut dst = Vec::with_capacity(bytes.len());

    nbt::ser::to_writer(&mut dst, &nbt, name).expect("NBT serialization.");
    assert_eq!(bytes, &dst[..]);

    let read: T = nbt::de::from_reader(bytes).expect("NBT deserialization.");
    assert_eq!(read, nbt);
}

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

    #[rustfmt::skip]
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

    assert_roundtrip_eq(nbt, &bytes, Some("data"));
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BasicListNbt {
    data: Vec<i16>,
}

#[test]
fn roundtrip_basic_list() {
    let nbt = BasicListNbt {
        data: vec![1, 2, 3],
    };

    #[rustfmt::skip]
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x09,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x02, // List type.
                0x00, 0x00, 0x00, 0x03, // Length.
                0x00, 0x01,
                0x00, 0x02,
                0x00, 0x03,
        0x00
    ];

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[test]
fn roundtrip_empty_list() {
    let nbt = BasicListNbt { data: vec![] };

    #[rustfmt::skip]
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

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NestedListNbt {
    data: Vec<Vec<i16>>,
}

#[test]
fn roundtrip_nested_list() {
    let nbt = NestedListNbt {
        data: vec![vec![1, 2], vec![3, 4]],
    };

    #[rustfmt::skip]
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

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NestedArrayNbt {
    #[serde(serialize_with = "nested_i32_array")]
    data: Vec<Vec<i32>>,
}

fn nested_i32_array<S>(outer_arr: &[Vec<i32>], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Debug)]
    struct Wrapper<'a>(&'a Vec<i32>);

    impl<'a> Serialize for Wrapper<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            nbt::i32_array(self.0, serializer)
        }
    }

    serializer.collect_seq(outer_arr.iter().map(|vec| Wrapper(vec)))
}

#[test]
fn roundtrip_nested_array() {
    let nbt = NestedArrayNbt {
        data: vec![vec![1, 2], vec![3, 4]],
    };

    #[rustfmt::skip]
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x09,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x0b, // Int array.
                0x00, 0x00, 0x00, 0x02,
                // First array.
                0x00, 0x00, 0x00, 0x02,
                0x00, 0x00, 0x00, 0x01,
                0x00, 0x00, 0x00, 0x02,
                // Second array.
                0x00, 0x00, 0x00, 0x02,
                0x00, 0x00, 0x00, 0x03,
                0x00, 0x00, 0x00, 0x04,
        0x00
    ];

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ByteArrayNbt {
    #[serde(serialize_with = "nbt::i8_array")]
    data: Vec<i8>,
}

#[test]
fn roundtrip_byte_array() {
    let nbt = ByteArrayNbt {
        data: vec![1, 2, 3],
    };

    #[rustfmt::skip]
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

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct IntArrayNbt {
    #[serde(serialize_with = "nbt::i32_array")]
    data: Vec<i32>,
}

#[test]
fn roundtrip_empty_array() {
    let nbt = IntArrayNbt { data: vec![] };

    #[rustfmt::skip]
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x0b,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x00, 0x00, 0x00, 0x00, // Length.
        0x00
    ];

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[test]
fn roundtrip_int_array() {
    let nbt = IntArrayNbt {
        data: vec![1, 2, 3],
    };

    #[rustfmt::skip]
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

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct LongArrayNbt {
    #[serde(serialize_with = "nbt::ser::i64_array")]
    data: Vec<i64>,
}

#[test]
fn roundtrip_long_array() {
    let nbt = LongArrayNbt {
        data: vec![1, 2, 3],
    };

    #[rustfmt::skip]
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

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize)]
struct CustomSerializerArrayNbt {
    #[serde(serialize_with = "shift_right_serializer")]
    data: Vec<i16>,
}

// We want to serialize an i16 vector as a ByteArray by shifting every element right by 8 bits
fn shift_right_serializer<S>(original_array: &[i16], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    nbt::i8_array(original_array.iter().map(|&i| (i >> 8) as i8), serializer)
}

#[test]
fn serialize_custom_serializer_array() {
    let nbt = CustomSerializerArrayNbt {
        data: vec![0xAABBu16 as i16, 0x3400, 0x1234, 0x0012],
    };

    #[rustfmt::skip]
    let bytes = vec![
        0x0a,
        0x00, 0x00,
        0x07,
        0x00, 0x04,
        0x64, 0x61, 0x74, 0x61,
        0x00, 0x00, 0x00, 0x04, // Length.
        0xAA, 0x34, 0x12, 0x00, // Content.
        0x00
    ];

    let mut dst = Vec::with_capacity(bytes.len());

    nbt::ser::to_writer(&mut dst, &nbt, None).expect("NBT serialization.");
    assert_eq!(bytes, &dst[..]);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BoolNbt {
    data: bool,
}

#[test]
fn roundtrip_bool() {
    let nbt = BoolNbt { data: true };

    #[rustfmt::skip]
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x01,
        0x00
    ];

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct OptionNbt {
    data: Option<i8>,
}

#[test]
fn roundtrip_some() {
    let nbt = OptionNbt { data: Some(100) };

    #[rustfmt::skip]
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x64,
        0x00
    ];

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[test]
fn roundtrip_none() {
    let nbt = OptionNbt { data: None };

    #[rustfmt::skip]
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            // Not included.
        0x00
    ];

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct UnitStructNbt;

#[test]
fn roundtrip_unit_struct() {
    let nbt = UnitStructNbt;

    #[rustfmt::skip]
    let bytes = vec![
        0x0a,
            0x00, 0x00,
        0x00
    ];

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NewByteNbt(ByteNbt);

#[test]
fn roundtrip_newtype_struct() {
    let nbt = NewByteNbt(ByteNbt { data: 100 });

    #[rustfmt::skip]
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x64,
        0x00
    ];

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NestedByteNbt {
    data: ByteNbt,
}

#[test]
fn roundtrip_nested() {
    let nbt = NestedByteNbt {
        data: ByteNbt { data: 100 },
    };

    #[rustfmt::skip]
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

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NestedUnitStructNbt {
    data: UnitStructNbt,
}

#[test]
fn roundtrip_nested_unit_struct() {
    let nbt = NestedUnitStructNbt {
        data: UnitStructNbt,
    };

    #[rustfmt::skip]
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

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NestedNewByteNbt {
    data: NewByteNbt,
}

#[test]
fn roundtrip_nested_newtype_struct() {
    let nbt = NestedNewByteNbt {
        data: NewByteNbt(ByteNbt { data: 100 }),
    };

    #[rustfmt::skip]
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

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[test]
fn roundtrip_hashmap() {
    let mut nbt = HashMap::new();
    nbt.insert("data".to_string(), 100i8);

    #[rustfmt::skip]
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x01,
                0x00, 0x04,
                0x64, 0x61, 0x74, 0x61,
                0x64,
        0x00
    ];

    assert_roundtrip_eq(nbt, &bytes, None);
}

#[test]
fn ser_blob_array() {
    let mut blob = nbt::Blob::new();
    blob.insert("larr", nbt::Value::LongArray(vec![456, 123])).unwrap();
    blob.insert("iarr", nbt::Value::IntArray(vec![123, 456])).unwrap();

    #[rustfmt::skip]
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x0c,
                0x00, 0x04,
                0x6c, 0x61, 0x72, 0x72,
                0x00, 0x00, 0x00, 0x02,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xc8,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7b,
            0x0b,
                0x00, 0x04,
                0x69, 0x61, 0x72, 0x72,
                0x00, 0x00, 0x00, 0x02,
                0x00, 0x00, 0x00, 0x7b,
                0x00, 0x00, 0x01, 0xc8,
        0x00
    ];

    let mut dst = Vec::with_capacity(bytes.len());

    nbt::ser::to_writer(&mut dst, &blob, None).expect("NBT serialization.");
    assert_eq!(bytes, &dst[..]);
}
