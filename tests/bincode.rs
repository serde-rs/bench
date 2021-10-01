#![allow(clippy::blacklisted_name, clippy::unseparated_literal_suffix)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Foo {
    bar: String,
    baz: u64,
    derp: bool,
}

impl Default for Foo {
    fn default() -> Self {
        Foo {
            bar: "hello".into(),
            baz: 1337u64,
            derp: true,
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
