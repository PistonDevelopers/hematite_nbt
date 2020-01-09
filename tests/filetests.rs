//! Crate for testing whether the deserialize codegen is capable of handling the
//! sample NBT files in the test/ directory, which include real
//! Minecraft-generated files.

#[macro_use] extern crate serde_derive;
extern crate serde;

extern crate nbt;

use std::fs::File;

use nbt::de::{from_reader, from_gzip_reader};
use nbt::Endianness;

// Include structure definitions.
include!("data.rs.in");

#[test]
fn deserialize_small1() {
    let nbt = Small1 { name: "Bananrama".to_string() };
    let mut file = File::open("tests/small1.nbt").unwrap();
    let read: Small1 = from_reader(&mut file, Endianness::BigEndian).unwrap();
    assert_eq!(nbt, read)
}

#[test]
fn deserialize_small2() {
    let nbt = Small2 {
        aaa: Small2Sub { one: 17, two: 4386, three: 287454020 },
        bbb: Small2Sub { one: 17, two: 4386, three: 287454020 }
    };
    let mut file = File::open("tests/small2.nbt").unwrap();
    let read: Small2 = from_reader(&mut file, Endianness::BigEndian).unwrap();
    assert_eq!(nbt, read)
}

#[test]
fn deserialize_small3() {
    let nbt = Small3 {
        bbb: vec![
            Small3Sub { ccc: 287454020, name: "wololo".to_string() },
            Small3Sub { ccc: 287454020, name: "wololo".to_string() }
        ]
    };
    let mut file = File::open("tests/small3.nbt").unwrap();
    let read: Small3 = from_reader(&mut file, Endianness::BigEndian).unwrap();
    assert_eq!(nbt, read)
}

#[test]
fn deserialize_small4() {
    let nbt = Small4 {
        c1: Small4Sub { aaa: 17, bbb: 34, ccc: 51, ddd: 68 },
        c2: Small4Sub { aaa: 17, bbb: 34, ccc: 51, ddd: 68 }
    };
    let mut file = File::open("tests/small4.nbt").unwrap();
    let read: Small4 = from_reader(&mut file, Endianness::BigEndian).unwrap();
    assert_eq!(nbt, read)
}

#[test]
#[ignore]
fn deserialize_big1() {
    let nbt = Big1 {
        list_test_compound: vec![
            Big1Sub1 { name: "Compound tag #0".to_string(),
                       created_on: 1264099775885 },
            Big1Sub1 { name: "Compound tag #1".to_string(),
                       created_on: 1264099775885 },
        ],
        long_test: 9223372036854775807,
        short_test: 32767,
        byte_test: 127,
        float_test: 0.0,
        nested_compound_test: Big1Sub3 {
            ham: Big1Sub2 { name: "Hampus".to_string(), value: 0.75 },
            egg: Big1Sub2 { name: "Eggbert".to_string(), value: 0.5 }
        },
        // Thanks, Notch...
        byte_array_test: vec![0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4,
                              86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50,
                              62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54,
                              86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50,
                              12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54,
                              36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0,
                              12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36,
                              78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62,
                              34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78,
                              80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84,
                              16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28,
                              80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84,
                              66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28,
                              30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34,
                              66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30,
                              92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16,
                              8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92,
                              14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58,
                              10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42,
                              14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58,
                              60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42,
                              64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8,
                              60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64,
                              46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10,
                              22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46,
                              88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72,
                              44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96,
                              88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72,
                              94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96,
                              38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22,
                              94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38,
                              40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44,
                              76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40,
                              2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26,
                              18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2,
                              24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26,
                              68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90,
                              52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76,
                              68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52,
                              74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18,
                              70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74,
                              56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18,
                              20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24,
                              56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68,
                              20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52,
                              24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68,
                              70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74,
                              6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70,
                              32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56,
                              48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20,
                              32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56,
                              98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20,
                              82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6,
                              98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82,
                              4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48,
                              0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4,
                              86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50,
                              62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54,
                              86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50,
                              12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54,
                              36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0,
                              12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36,
                              78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62,
                              34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78,
                              80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84,
                              16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28,
                              80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84,
                              66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28,
                              30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34,
                              66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30,
                              92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16,
                              8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92,
                              14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58,
                              10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42,
                              14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58,
                              60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42,
                              64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8,
                              60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64,
                              46, 38, 40, 52, 74, 6, 48],
        string_test: "HELLO WORLD THIS IS A TEST STRING ÅÄÖ!".to_string(),
        list_test_long: [11, 12, 13, 14, 15],
        double_test: 0.4931287132182315,
        int_test: 2147483647,
    };
    let mut file = File::open("tests/big1.nbt").unwrap();
    let read: Big1 = from_gzip_reader(&mut file, Endianness::BigEndian).unwrap();
    assert_eq!(nbt, read)
}

#[test]
fn deserialize_simple_player() {
    let mut file = File::open("tests/simple_player.dat").unwrap();
    let _: PlayerData = from_gzip_reader(&mut file, Endianness::BigEndian).unwrap();
}

#[test]
fn deserialize_complex_player() {
    let mut file = File::open("tests/complex_player.dat").unwrap();
    let _: PlayerData = from_gzip_reader(&mut file, Endianness::BigEndian).unwrap();
}

#[test]
fn deserialize_level() {
    let mut file = File::open("tests/level.dat").unwrap();
    let _: Level = from_gzip_reader(&mut file, Endianness::BigEndian).unwrap();
}
