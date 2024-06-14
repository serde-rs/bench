#![no_main]
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Foo {
    some_str: String,
    some_u8: u8,
    some_u16: u16,
    some_u32: u32,
    some_u64: u64,
    some_i8: i8,
    some_i16: i16,
    some_i32: i32,
    some_i64: i64,
    some_bool: bool,
}

fn serialize(foo: &Foo) -> Vec<u8> {
    let bincode_bytes = bincode::serialize(&foo).unwrap();

    let mut serde_bytes = Vec::new();
    serde_bench::serialize(&mut serde_bytes, &foo).unwrap();

    assert_eq!(bincode_bytes, serde_bytes);

    serde_bytes
}

fn deserialize(bytes: &Vec<u8>) -> Foo {
    let bincode_foo = bincode::deserialize::<Foo>(&bytes).unwrap();

    let serde_foo = serde_bench::deserialize::<Foo>(&bytes).unwrap();

    assert_eq!(serde_foo, bincode_foo);

    serde_foo
}

fn extract(data: &[u8], cursor: &mut usize, len: usize) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(len);
    let (left, right) = if len + *cursor < data.len() {
        (len, 0)
    } else {
        (data.len() - *cursor, len - (data.len() - *cursor))
    };
    v.extend(&data[*cursor..(*cursor + left)]);
    *cursor += left;
    v.extend(std::iter::repeat(0u8).take(right));

    assert_eq!(v.len(), len);

    v
}

fn extract_u8(data: &[u8], cursor: &mut usize) -> u8 {
    extract(&data, cursor, 1).pop().unwrap()
}

fn extract_u16(data: &[u8], cursor: &mut usize) -> u16 {
    let v = extract(&data, cursor, 2);
    u16::from_ne_bytes([v[0], v[1]])
}

fn extract_u32(data: &[u8], cursor: &mut usize) -> u32 {
    let v = extract(&data, cursor, 4);
    u32::from_ne_bytes([v[0], v[1], v[2], v[3]])
}

fn extract_u64(data: &[u8], cursor: &mut usize) -> u64 {
    let v = extract(&data, cursor, 8);
    u64::from_ne_bytes([v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]])
}

fn extract_i8(data: &[u8], cursor: &mut usize) -> i8 {
    extract(&data, cursor, 1).pop().unwrap() as i8
}

fn extract_i16(data: &[u8], cursor: &mut usize) -> i16 {
    let v = extract(&data, cursor, 2);
    i16::from_ne_bytes([v[0], v[1]])
}

fn extract_i32(data: &[u8], cursor: &mut usize) -> i32 {
    let v = extract(&data, cursor, 4);
    i32::from_ne_bytes([v[0], v[1], v[2], v[3]])
}

fn extract_i64(data: &[u8], cursor: &mut usize) -> i64 {
    let v = extract(&data, cursor, 8);
    i64::from_ne_bytes([v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]])
}

fn extract_bool(data: &[u8], cursor: &mut usize) -> bool {
    extract(&data, cursor, 1).pop().unwrap() != 0
}

fn extract_remainder_as_string(data: &[u8], cursor: &mut usize) -> String {
    let (_, right) = data.split_at(*cursor);
    String::from_utf8_lossy(right).into()
}

fuzz_target!(|data: &[u8]| {
    if data.len() == 0 {
        return;
    }
    let mut cursor: usize = 0;

    let foo = Foo {
        some_u8: extract_u8(&data, &mut cursor),
        some_u16: extract_u16(&data, &mut cursor),
        some_u32: extract_u32(&data, &mut cursor),
        some_u64: extract_u64(&data, &mut cursor),
        some_i8: extract_i8(&data, &mut cursor),
        some_i16: extract_i16(&data, &mut cursor),
        some_i32: extract_i32(&data, &mut cursor),
        some_i64: extract_i64(&data, &mut cursor),
        some_bool: extract_bool(&data, &mut cursor),
        some_str: extract_remainder_as_string(&data, &mut cursor),
    };

    let bytes = serialize(&foo);
    let foo_serde = deserialize(&bytes);
    assert_eq!(foo, foo_serde);
});
