#![feature(plugin, test, custom_derive, custom_attribute)]
#![plugin(nbt_macros)]

extern crate nbt;
extern crate test;

use nbt::serialize;

#[derive(NbtFmt, PartialEq, Debug)]
struct TestStruct {
    name: String,
    health: i8,
    food: f32,
    emeralds: i16,
    timestamp: i32
}

#[derive(NbtFmt, PartialEq, Debug)]
struct ByteArrayStruct {
    #[nbt_byte_array]
    array: Vec<i8>
}

#[test]
fn nbt_test_struct_serialize() {
    let test = TestStruct {
        name: "Herobrine".to_string(),
        health: 100, food: 20.0, emeralds: 12345, timestamp: 1424778774
    };

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x08,
                0x00, 0x04,
                0x6e, 0x61, 0x6d, 0x65,
                0x00, 0x09,
                0x48, 0x65, 0x72, 0x6f, 0x62, 0x72, 0x69, 0x6e, 0x65,
            0x01,
                0x00, 0x06,
                0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
                0x64,
            0x05,
                0x00, 0x04,
                0x66, 0x6f, 0x6f, 0x64,
                0x41, 0xa0, 0x00, 0x00,
            0x02,
                0x00, 0x08,
                0x65, 0x6d, 0x65, 0x72, 0x61, 0x6c, 0x64, 0x73,
                0x30, 0x39,
            0x03,
                0x00, 0x09,
                0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
                0x54, 0xec, 0x66, 0x16,
        0x00
    ];


    let mut dst = Vec::new();
    serialize::to_writer(&mut dst, &test).unwrap();

    let mut src = std::io::Cursor::new(&bytes[..]);
    let result: TestStruct = serialize::from_reader(&mut src).unwrap();

    assert_eq!(&bytes[..], &dst[..]);
    assert_eq!(&test, &result);
}

#[test]
fn nbt_test_byte_array() {
    let test = ByteArrayStruct { array: vec![0, 1, 2, 3, 4, 5] };

    let mut blob = nbt::Blob::new("".to_string());
    blob.insert("array".to_string(), nbt::Value::ByteArray(vec![0, 1, 2, 3, 4, 5]));

    let mut dst = Vec::new();
    blob.write(&mut dst).unwrap();

    let mut src = std::io::Cursor::new(&dst[..]);
    let result: ByteArrayStruct = serialize::from_reader(&mut src).unwrap();

    assert_eq!(&test, &result);
}
