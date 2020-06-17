//! Crate for testing whether the deserialize codegen is capable of handling the
//! sample NBT files in the test/ directory, which include real
//! Minecraft-generated files.

extern crate criterion;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate nbt;

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use nbt::de::from_gzip_reader;
use nbt::ser::to_writer;

mod data {
    include!("../tests/data.rs.in");
}

fn bench_serialize<T>(filename: &str, c: &mut Criterion)
where
    T: serde::de::DeserializeOwned + serde::ser::Serialize,
{
    let mut file = File::open(filename).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = std::io::Cursor::new(&contents[..]);
    file.seek(SeekFrom::Start(0)).unwrap();
    let nbt_struct: T = from_gzip_reader(&mut file).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    let nbt_blob = nbt::Blob::from_gzip_reader(&mut file).unwrap();

    let mut group = c.benchmark_group(filename);
    group.throughput(Throughput::Bytes(contents.len() as u64));
    group.bench_function("Deserialize As Struct", |b| {
        b.iter(|| {
            src.seek(SeekFrom::Start(0)).unwrap();
            let _: T = from_gzip_reader(&mut src).unwrap();
        })
    });
    group.bench_function("Deserialize As Blob", |b| {
        b.iter(|| {
            src.seek(SeekFrom::Start(0)).unwrap();
            nbt::Blob::from_gzip_reader(&mut src).unwrap();
        })
    });
    group.bench_function("Serialize As Struct", |b| {
        b.iter(|| {
            to_writer(&mut io::sink(), &nbt_struct, None).unwrap();
        })
    });
    group.bench_function("Serialize As Blob", |b| {
        b.iter(|| {
            nbt_blob.to_writer(&mut io::sink()).unwrap();
        })
    });
    group.finish();
}

fn bench(c: &mut Criterion) {
    bench_serialize::<data::Big1>("tests/big1.nbt", c);
    bench_serialize::<data::PlayerData>("tests/simple_player.dat", c);
    bench_serialize::<data::PlayerData>("tests/complex_player.dat", c);
    bench_serialize::<data::Level>("tests/level.dat", c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
