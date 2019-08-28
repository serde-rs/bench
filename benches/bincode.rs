#![feature(test)]

extern crate test;

use serde::{Deserialize, Serialize};
use test::Bencher;

#[derive(Serialize, Deserialize)]
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

#[bench]
fn bincode_deserialize(b: &mut Bencher) {
    let foo = Foo::default();
    let bytes = bincode::serialize(&foo).unwrap();

    b.iter(|| bincode::deserialize::<Foo>(&bytes).unwrap());
}

#[bench]
fn bincode_serialize(b: &mut Bencher) {
    let foo = Foo::default();

    b.iter(|| {
        let mut bytes = Vec::with_capacity(128);
        bincode::serialize_into(&mut bytes, &foo).unwrap()
    });
}

#[bench]
fn serde_deserialize(b: &mut Bencher) {
    let foo = Foo::default();
    let mut bytes = Vec::new();
    serde_bench::serialize(&mut bytes, &foo).unwrap();

    b.iter(|| serde_bench::deserialize::<Foo>(&bytes).unwrap());
}

#[bench]
fn serde_serialize(b: &mut Bencher) {
    let foo = Foo::default();

    b.iter(|| {
        let mut bytes = Vec::with_capacity(128);
        serde_bench::serialize(&mut bytes, &foo).unwrap()
    });
}
