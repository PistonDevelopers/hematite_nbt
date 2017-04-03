//! Crate for testing whether the deserialize codegen is capable of handling the
//! sample NBT files in the test/ directory.

#![feature(test)]
extern crate test;

#[macro_use] extern crate serde_derive;
extern crate serde;

extern crate nbt;
extern crate nbt_serde;

use std::fs::File;

use nbt_serde::decode::{from_reader, from_gzip};

#[derive(Debug, PartialEq, Deserialize)]
struct Small1 {
    name: String
}

#[test]
fn deserialize_small1() {
    let nbt = Small1 { name: "Bananrama".to_string() };
    let mut file = File::open("../tests/small1.nbt").unwrap();
    let read: Small1 = from_reader(&mut file).unwrap();
    assert_eq!(nbt, read)
}

#[derive(Debug, PartialEq, Deserialize)]
struct Small2Sub {
    #[serde(rename = "1")] one: i8,
    #[serde(rename = "2")] two: i16,
    #[serde(rename = "3")] three: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Small2 {
    aaa: Small2Sub,
    bbb: Small2Sub,
}

#[test]
fn deserialize_small2() {
    let nbt = Small2 {
        aaa: Small2Sub { one: 17, two: 4386, three: 287454020 },
        bbb: Small2Sub { one: 17, two: 4386, three: 287454020 }
    };
    let mut file = File::open("../tests/small2.nbt").unwrap();
    let read: Small2 = from_reader(&mut file).unwrap();
    assert_eq!(nbt, read)
}

#[derive(Debug, PartialEq, Deserialize)]
struct Small3Sub {
    ccc: i32,
    name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Small3 {
    bbb: Vec<Small3Sub>,
}

#[test]
fn deserialize_small3() {
    let nbt = Small3 {
        bbb: vec![
            Small3Sub { ccc: 287454020, name: "wololo".to_string() },
            Small3Sub { ccc: 287454020, name: "wololo".to_string() }
        ]
    };
    let mut file = File::open("../tests/small3.nbt").unwrap();
    let read: Small3 = from_reader(&mut file).unwrap();
    assert_eq!(nbt, read)
}

#[derive(Debug, PartialEq, Deserialize)]
struct Small4Sub {
    aaa: i8,
    bbb: i8,
    ccc: i8,
    ddd: i8,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Small4 {
    c1: Small4Sub,
    c2: Small4Sub,
}

#[test]
fn deserialize_small4() {
    let nbt = Small4 {
        c1: Small4Sub { aaa: 17, bbb: 34, ccc: 51, ddd: 68 },
        c2: Small4Sub { aaa: 17, bbb: 34, ccc: 51, ddd: 68 }
    };
    let mut file = File::open("../tests/small4.nbt").unwrap();
    let read: Small4 = from_reader(&mut file).unwrap();
    assert_eq!(nbt, read)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Big1Sub1 {
    name: String,
    #[serde(rename = "created-on")] created_on: i64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Big1Sub2 {
    name: String,
    value: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Big1Sub3 {
    ham: Big1Sub2,
    egg: Big1Sub2,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Big1 {
    #[serde(rename = "listTest (compound)")] list_test_compound: Vec<Big1Sub1>,
    #[serde(rename = "longTest")] long_test: i64,
    #[serde(rename = "shortTest")] short_test: i32,
    #[serde(rename = "byteTest")] byte_test: i8,
    #[serde(rename = "floatTest")] float_test: i64,
    #[serde(rename = "nested compound test")] nested_compound_test: Big1Sub3,
    #[serde(rename = "byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))")]
    byte_array_test: Vec<i8>, // [i8; 1000] does not implement PartialEq.
    #[serde(rename = "stringTest")] string_test: String,
    #[serde(rename = "listTest (long)")]
    list_test_long: [i64; 5], // Vec<i64> also works.
    #[serde(rename = "doubleTest")] double_test: f64,
    #[serde(rename = "intTest")] int_test: i32,
}

#[test]
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
        float_test: 0,
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
    let mut file = File::open("../tests/big1.nbt").unwrap();
    let read: Big1 = from_gzip(&mut file).unwrap();
    assert_eq!(nbt, read)
}

mod bench {
    use std::io;
    use test::Bencher;
    use nbt_serde::encode::to_writer;
    // use serde::Serialize;
    use super::*;

    #[bench]
    #[ignore]
    fn deserialize_big1_as_struct(b: &mut Bencher) {
        b.iter(|| {
            let mut file = File::open("../tests/big1.nbt").unwrap();
            let _: Big1 = from_gzip(&mut file).unwrap();
        });
    }

    #[bench]
    #[ignore]
    fn deserialize_big1_as_blob(b: &mut Bencher) {
        b.iter(|| {
            let mut file = File::open("../tests/big1.nbt").unwrap();
            nbt::Blob::from_gzip(&mut file).unwrap();
        });
    }
    
    #[bench]
    fn serialize_big1_as_struct(b: &mut Bencher) {
        let mut file = File::open("../tests/big1.nbt").unwrap();
        let nbt: Big1 = from_gzip(&mut file).unwrap();
        b.iter(|| {
            to_writer(&mut io::sink(), &nbt, None)
        });
    }

    #[bench]
    fn serialize_big1_as_blob(b: &mut Bencher) {
        let mut file = File::open("../tests/big1.nbt").unwrap();
        let nbt = nbt::Blob::from_gzip(&mut file).unwrap();
        b.iter(|| {
            nbt.write(&mut io::sink())
        });
    }
}
