//! Crate for testing whether the deserialize codegen is capable of handling the
//! sample NBT files in the test/ directory, which include real
//! Minecraft-generated files.

#![feature(test)]
extern crate test;

#[macro_use] extern crate serde_derive;
extern crate serde;

extern crate nbt;

use std::fs::File;
use std::io::{self, Read};

use test::Bencher;

use nbt::de::from_gzip;
use nbt::ser::to_writer;

mod data {
    include!("../tests/data.rs.in");
}

#[bench]
fn deserialize_big1_as_struct(b: &mut Bencher) {
    let mut file = File::open("tests/big1.nbt").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    b.iter(|| {
        let mut src = std::io::Cursor::new(&contents[..]);
        let _: data::Big1 = from_gzip(&mut src).unwrap();
    });
}

#[bench]
fn deserialize_big1_as_blob(b: &mut Bencher) {
    let mut file = File::open("tests/big1.nbt").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    b.iter(|| {
        let mut src = std::io::Cursor::new(&contents[..]);
        nbt::Blob::from_gzip(&mut src).unwrap();
    });
}

#[bench]
fn serialize_big1_as_struct(b: &mut Bencher) {
    let mut file = File::open("tests/big1.nbt").unwrap();
    let nbt: data::Big1 = from_gzip(&mut file).unwrap();
    b.iter(|| {
        to_writer(&mut io::sink(), &nbt, None)
    });
}

#[bench]
fn serialize_big1_as_blob(b: &mut Bencher) {
    let mut file = File::open("tests/big1.nbt").unwrap();
    let nbt = nbt::Blob::from_gzip(&mut file).unwrap();
    b.iter(|| {
        nbt.write(&mut io::sink())
    });
}
#[bench]
fn deserialize_simple_player_as_struct(b: &mut Bencher) {
    let mut file = File::open("tests/simple_player.dat").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    b.iter(|| {
        let mut src = std::io::Cursor::new(&contents[..]);
        let _: data::PlayerData = from_gzip(&mut src).unwrap();
    });
}

#[bench]
fn deserialize_simple_player_as_blob(b: &mut Bencher) {
    let mut file = File::open("tests/simple_player.dat").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    b.iter(|| {
        let mut src = std::io::Cursor::new(&contents[..]);
        nbt::Blob::from_gzip(&mut src).unwrap();
    });
}

#[bench]
fn serialize_simple_player_as_struct(b: &mut Bencher) {
    let mut file = File::open("tests/simple_player.dat").unwrap();
    let nbt: data::PlayerData = from_gzip(&mut file).unwrap();
    b.iter(|| {
        to_writer(&mut io::sink(), &nbt, None)
    });
}

#[bench]
fn serialize_simple_player_as_blob(b: &mut Bencher) {
    let mut file = File::open("tests/simple_player.dat").unwrap();
    let nbt = nbt::Blob::from_gzip(&mut file).unwrap();
    b.iter(|| {
        nbt.write(&mut io::sink())
    });
}

#[bench]
fn deserialize_complex_player_as_struct(b: &mut Bencher) {
    let mut file = File::open("tests/complex_player.dat").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    b.iter(|| {
        let mut src = std::io::Cursor::new(&contents[..]);
        let _: data::PlayerData = from_gzip(&mut src).unwrap();
    });
}

#[bench]
fn deserialize_complex_player_as_blob(b: &mut Bencher) {
    let mut file = File::open("tests/complex_player.dat").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    b.iter(|| {
        let mut src = std::io::Cursor::new(&contents[..]);
        nbt::Blob::from_gzip(&mut src).unwrap();
    });
}

#[bench]
fn serialize_complex_player_as_struct(b: &mut Bencher) {
    let mut file = File::open("tests/complex_player.dat").unwrap();
    let nbt: data::PlayerData = from_gzip(&mut file).unwrap();
    b.iter(|| {
        to_writer(&mut io::sink(), &nbt, None)
    });
}

#[bench]
fn serialize_complex_player_as_blob(b: &mut Bencher) {
    let mut file = File::open("tests/complex_player.dat").unwrap();
    let nbt = nbt::Blob::from_gzip(&mut file).unwrap();
    b.iter(|| {
        nbt.write(&mut io::sink())
    });
}

#[bench]
fn deserialize_level_as_struct(b: &mut Bencher) {
    let mut file = File::open("tests/level.dat").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    b.iter(|| {
        let mut src = std::io::Cursor::new(&contents[..]);
        let _: data::Level = from_gzip(&mut src).unwrap();
    });
}

#[bench]
fn deserialize_level_as_blob(b: &mut Bencher) {
    let mut file = File::open("tests/level.dat").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    b.iter(|| {
        let mut src = std::io::Cursor::new(&contents[..]);
        nbt::Blob::from_gzip(&mut src).unwrap();
    });
}

#[bench]
fn serialize_level_as_struct(b: &mut Bencher) {
    let mut file = File::open("tests/level.dat").unwrap();
    let nbt: data::Level = from_gzip(&mut file).unwrap();
    b.iter(|| {
        to_writer(&mut io::sink(), &nbt, None)
    });
}

#[bench]
fn serialize_level_as_blob(b: &mut Bencher) {
    let mut file = File::open("tests/level.dat").unwrap();
    let nbt = nbt::Blob::from_gzip(&mut file).unwrap();
    b.iter(|| {
        nbt.write(&mut io::sink())
    });
}
