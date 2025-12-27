#![feature(test)]
#![allow(clippy::elidable_lifetime_names)]

extern crate test;

mod flavor;

use crate::flavor::PreallocatedVec;
use serde::{Deserialize, Serialize};
use std::hint::black_box;
use test::Bencher;

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode)]
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
fn bincode_serde_deserialize(b: &mut Bencher) {
    let foo = Foo::default();
    let bytes = bincode::serde::encode_to_vec(&foo, bincode::config::standard()).unwrap();

    b.iter(|| {
        let bytes = black_box(&bytes);
        bincode::serde::decode_from_slice::<Foo, _>(bytes, bincode::config::standard()).unwrap()
    });
}

#[bench]
fn bincode_serde_serialize(b: &mut Bencher) {
    let foo = Foo::default();
    let mut bytes = Vec::with_capacity(128);

    b.iter(|| {
        let foo = black_box(&foo);
        bytes.clear();
        bincode::serde::encode_into_std_write(foo, &mut bytes, bincode::config::standard())
            .unwrap();
    });
}

#[bench]
fn bincode_decode(b: &mut Bencher) {
    let foo = Foo::default();
    let bytes = bincode::encode_to_vec(&foo, bincode::config::standard()).unwrap();

    b.iter(|| {
        let bytes = black_box(&bytes);
        bincode::decode_from_slice::<Foo, _>(bytes, bincode::config::standard()).unwrap()
    });
}

#[bench]
fn bincode_encode(b: &mut Bencher) {
    let foo = Foo::default();
    let mut bytes = Vec::with_capacity(128);

    b.iter(|| {
        let foo = black_box(&foo);
        bytes.clear();
        bincode::encode_into_std_write(foo, &mut bytes, bincode::config::standard()).unwrap();
    });
}

#[bench]
fn postcard_deserialize(b: &mut Bencher) {
    let foo = Foo::default();
    let bytes = postcard::to_stdvec(&foo).unwrap();

    b.iter(|| {
        let bytes = black_box(&bytes);
        postcard::from_bytes::<Foo>(bytes).unwrap()
    });
}

#[bench]
fn postcard_serialize(b: &mut Bencher) {
    let foo = Foo::default();
    let mut bytes = Vec::with_capacity(128);

    b.iter(|| {
        let foo = black_box(&foo);
        bytes.clear();
        postcard::serialize_with_flavor(foo, PreallocatedVec::new(&mut bytes)).unwrap();
    });
}

#[bench]
fn serde_deserialize(b: &mut Bencher) {
    let foo = Foo::default();
    let mut bytes = Vec::new();
    serde_bench::serialize(&mut bytes, &foo).unwrap();

    b.iter(|| {
        let bytes = black_box(&bytes);
        serde_bench::deserialize::<Foo>(bytes).unwrap()
    });
}

#[bench]
fn serde_serialize(b: &mut Bencher) {
    let foo = Foo::default();
    let mut bytes = Vec::with_capacity(128);

    b.iter(|| {
        let foo = black_box(&foo);
        bytes.clear();
        serde_bench::serialize(&mut bytes, foo).unwrap();
    });
}
