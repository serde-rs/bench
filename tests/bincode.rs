use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Foo {
    some_str: String,
    some_u8: u8,
    some_u16: u16,
    some_u32: u32,
    some_u64: u64,
    some_u128: u128,
    some_i8: i8,
    some_i16: i16,
    some_i32: i32,
    some_i64: i64,
    some_i128: i128,
    some_bool: bool,
}

impl Default for Foo {
    fn default() -> Self {
        Foo {
            some_str: "hello".into(),
            some_u8: 42u8,
            some_u16: 1337u16,
            some_u32: 1337u32,
            some_u64: 1337u64,
            some_u128: 1337u128,
            some_i8: 42i8,
            some_i16: 1337i16,
            some_i32: 1337i32,
            some_i64: 1337i64,
            some_i128: 1337i128,
            some_bool: true,
        }
    }
}

#[test]
fn test_ser() {
    let foo = Foo::default();

    let bincode_bytes = bincode::serialize(&foo).unwrap();

    let mut serde_bytes = Vec::new();
    serde_bench::serialize(&mut serde_bytes, &foo).unwrap();

    assert_eq!(bincode_bytes, serde_bytes);
}

#[test]
fn test_de() {
    let foo = Foo::default();
    let mut bytes = Vec::new();
    serde_bench::serialize(&mut bytes, &foo).unwrap();

    let bincode_foo = bincode::deserialize::<Foo>(&bytes).unwrap();
    assert_eq!(bincode_foo, foo);

    let serde_foo = serde_bench::deserialize::<Foo>(&bytes).unwrap();
    assert_eq!(serde_foo, foo);
}
