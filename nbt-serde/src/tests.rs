#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate nbt;
extern crate nbt_serde;

use serde::Serialize;

use nbt_serde::encode::Serializer;

#[derive(Serialize)]
struct Herobrine {
    name: String,
    health: i8,
    food: f32,
    emeralds: i16,
    timestamp: i32,
    data: Vec<i8>,
}

#[derive(Serialize)]
struct Simple {
    data: i8,
}

#[derive(Serialize)]
struct NewSimple(Simple);
    
#[test]
fn nbt_derive_basic_encoding() {
    let nbt = Herobrine {
        name: "Herobrine".to_string(),
        health: 100,
        food: 20.0,
        emeralds: 12345,
        timestamp: 1424778774,
        data: vec![1, 2, 3]
    };

    let mut dst = Vec::new();
    nbt.serialize(&mut Serializer::new(&mut dst, None)).ok().unwrap();

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

#[test]
fn serialize_newtype_struct() {
    let nbt = NewSimple(Simple { data: 100 });

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
