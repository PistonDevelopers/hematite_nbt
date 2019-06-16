use std::collections::HashMap;
use std::io;
use std::fs::File;

//use test::Bencher;

use blob::Blob;
use error::Error;
use value::Value;

#[test]
fn nbt_nonempty() {
    let mut nbt = Blob::new();
    nbt.insert("name", "Herobrine").unwrap();
    nbt.insert("health", 100i8).unwrap();
    nbt.insert("food", 20.0f32).unwrap();
    nbt.insert("emeralds", 12345i16).unwrap();
    nbt.insert("timestamp", 1424778774i32).unwrap();

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

    // Test correct length.
    let mut dst = Vec::new();
    nbt.to_writer(&mut dst).unwrap();
    assert_eq!(bytes.len(), dst.len());

    // We can only test if the decoded bytes match, since the HashMap does
    // not guarantee order (and so encoding is likely to be different, but
    // still correct).
    let mut src = io::Cursor::new(bytes);
    let file = Blob::from_reader(&mut src).unwrap();
    assert_eq!(&file, &nbt);
}

#[test]
fn nbt_empty_nbtfile() {
    let nbt = Blob::new();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
        0x00
    ];

    // Test encoding.
    let mut dst = Vec::new();
    nbt.to_writer(&mut dst).unwrap();
    assert_eq!(&dst, &bytes);

    // Test decoding.
    let mut src = io::Cursor::new(bytes);
    let file = Blob::from_reader(&mut src).unwrap();
    assert_eq!(&file, &nbt);
}

#[test]
fn nbt_nested_compound() {
    let mut inner = HashMap::new();
    inner.insert("test".to_string(), Value::Byte(123));
    let mut nbt = Blob::new();
    nbt.insert("inner", Value::Compound(inner)).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x0a,
                0x00, 0x05,
                0x69, 0x6e, 0x6e, 0x65, 0x72,
                0x01,
                0x00, 0x04,
                0x74, 0x65, 0x73, 0x74,
                0x7b,
            0x00,
        0x00
    ];

    // Test encoding.
    let mut dst = Vec::new();
    nbt.to_writer(&mut dst).unwrap();
    assert_eq!(&dst, &bytes);

    // Test decoding.
    let mut src = io::Cursor::new(bytes);
    let file = Blob::from_reader(&mut src).unwrap();
    assert_eq!(&file, &nbt);
}

#[test]
fn nbt_empty_list() {
    let mut nbt = Blob::new();
    nbt.insert("list", Value::List(Vec::new())).unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x09,
                0x00, 0x04,
                0x6c, 0x69, 0x73, 0x74,
                0x01,
                0x00, 0x00, 0x00, 0x00,
        0x00
    ];

    // Test encoding.
    let mut dst = Vec::new();
    nbt.to_writer(&mut dst).unwrap();
    assert_eq!(&dst, &bytes);

    // Test decoding.
    let mut src = io::Cursor::new(bytes);
    let file = Blob::from_reader(&mut src).unwrap();
    assert_eq!(&file, &nbt);
}

#[test]
fn nbt_no_root() {
    let bytes = vec![0x00];
    // Will fail, because the root is not a compound.
    assert_eq!(Blob::from_reader(&mut io::Cursor::new(&bytes[..])),
            Err(Error::NoRootCompound));
}

#[test]
fn nbt_no_end_tag() {
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x09,
                0x00, 0x04,
                0x6c, 0x69, 0x73, 0x74,
                0x01,
                0x00, 0x00, 0x00, 0x00
    ];

    // Will fail, because there is no end tag.
    assert_eq!(Blob::from_reader(&mut io::Cursor::new(&bytes[..])),
            Err(Error::IncompleteNbtValue));
}

#[test]
fn nbt_invalid_id() {
    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x0f, // No tag associated with 0x0f.
                0x00, 0x04,
                0x6c, 0x69, 0x73, 0x74,
                0x01,
        0x00
    ];
    assert_eq!(Blob::from_reader(&mut io::Cursor::new(&bytes[..])),
               Err(Error::InvalidTypeId(15)));
}

#[test]
fn nbt_invalid_list() {
    let mut nbt = Blob::new();
    let mut badlist = Vec::new();
    badlist.push(Value::Byte(1));
    badlist.push(Value::Short(1));
    // Will fail to insert, because the List is heterogeneous.
    assert_eq!(nbt.insert("list", Value::List(badlist)),
               Err(Error::HeterogeneousList));
}

#[test]
fn nbt_bad_compression() {
    // These aren't in the zlib or gzip format, so they'll fail.
    let bytes = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    assert!(Blob::from_gzip_reader(&mut io::Cursor::new(&bytes[..])).is_err());
    assert!(Blob::from_zlib_reader(&mut io::Cursor::new(&bytes[..])).is_err());
}

#[test]
fn nbt_compression() {
    // Create a non-trivial Blob.
    let mut nbt = Blob::new();
    nbt.insert("name", Value::String("Herobrine".to_string())).unwrap();
    nbt.insert("health", Value::Byte(100)).unwrap();
    nbt.insert("food", Value::Float(20.0)).unwrap();
    nbt.insert("emeralds", Value::Short(12345)).unwrap();
    nbt.insert("timestamp", Value::Int(1424778774)).unwrap();

    // Test zlib encoding/decoding.
    let mut zlib_dst = Vec::new();
    nbt.to_zlib_writer(&mut zlib_dst).unwrap();
    let zlib_file = Blob::from_zlib_reader(&mut io::Cursor::new(zlib_dst)).unwrap();
    assert_eq!(&nbt, &zlib_file);

    // Test gzip encoding/decoding.
    let mut gzip_dst = Vec::new();
    nbt.to_gzip_writer(&mut gzip_dst).unwrap();
    let gz_file = Blob::from_gzip_reader(&mut io::Cursor::new(gzip_dst)).unwrap();
    assert_eq!(&nbt, &gz_file);
}

#[test]
fn nbt_bigtest() {
    let mut bigtest_file = File::open("tests/big1.nbt").unwrap();
    let bigtest = Blob::from_gzip_reader(&mut bigtest_file).unwrap();
    // This is a pretty indirect way of testing correctness.
    let mut dst = Vec::new();
    bigtest.to_writer(&mut dst).unwrap();
    assert_eq!(1544, dst.len());
}

#[test]
fn nbt_arrays() {
    let mut arrays_file = File::open("tests/arrays.nbt").unwrap();
    let arrays = Blob::from_reader(&mut arrays_file).unwrap();
    match &arrays["ia"] {
        &Value::IntArray(ref arr) => assert_eq!(&[-2, -1, 0, 1, 2], &**arr),
        _ => panic!("ia was not TAG_IntArray"),
    }

    match &arrays["ba"] {
        &Value::ByteArray(ref arr) => assert_eq!(&[-2, -1, 0, 1, 2], &**arr),
        _ => panic!("ba was not TAG_ByteArray"),
    }

    match &arrays["la"] {
        &Value::LongArray(ref arr) => assert_eq!(&[-2, -1, 0, 1, 2], &**arr),
        _ => panic!("la was not TAG_LongArray"),
    }
}

#[test]
#[cfg(feature = "serde")]
fn serde_blob() {
    use de::from_reader;
    use ser::to_writer;

    let mut nbt = Blob::new();
    nbt.insert("name", "Herobrine").unwrap();
    nbt.insert("health", 100i8).unwrap();
    nbt.insert("food", 20.0f32).unwrap();
    nbt.insert("emeralds", 12345i16).unwrap();
    nbt.insert("timestamp", 1424778774i32).unwrap();

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

    // Roundtrip.

    let mut src = io::Cursor::new(bytes.clone());
    let file: Blob = from_reader(&mut src).unwrap();
    assert_eq!(&file, &nbt);
    let mut dst = Vec::new();
    to_writer(&mut dst, &nbt, None).unwrap();
    // We can only test if the decoded bytes match, since the HashMap does
    // not guarantee order (and so encoding is likely to be different, but
    // still correct).
    assert_eq!(&bytes.len(), &dst.len());
}

#[test]
fn nbt_modified_utf8() {
    let mut nbt = Blob::new();
    // These strings are taken from the cesu8 documentation.
    nbt.insert("\u{10401}", "\0\0").unwrap();

    let bytes = vec![
        0x0a,
            0x00, 0x00,
            0x08,
                0x00, 0x06,
                0xed, 0xa0, 0x81, 0xed, 0xb0, 0x81,
                0x00, 0x04,
                0xc0, 0x80, 0xc0, 0x80,
        0x00
    ];

    // Test encoding.
    let mut dst = Vec::new();
    nbt.to_writer(&mut dst).unwrap();
    assert_eq!(&dst, &bytes);

    // Test decoding.
    let mut src = io::Cursor::new(bytes);
    let file = Blob::from_reader(&mut src).unwrap();
    assert_eq!(&file, &nbt);
}
