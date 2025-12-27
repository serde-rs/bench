#![feature(test)]
#![allow(clippy::disallowed_names, clippy::elidable_lifetime_names)]

extern crate test;

mod flavor;

use crate::flavor::PreallocatedVec;
use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use serde::{Deserialize, Serialize};
use std::hint::black_box;

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

fn bincode_serde_deserialize(b: &mut Bencher) {
    let foo = Foo::default();
    let bytes = bincode::serde::encode_to_vec(&foo, bincode::config::standard()).unwrap();

    b.iter(|| {
        let bytes = black_box(&bytes);
        bincode::serde::decode_from_slice::<Foo, _>(bytes, bincode::config::standard()).unwrap()
    });
}

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

fn bincode_decode(b: &mut Bencher) {
    let foo = Foo::default();
    let bytes = bincode::encode_to_vec(&foo, bincode::config::standard()).unwrap();

    b.iter(|| {
        let bytes = black_box(&bytes);
        bincode::decode_from_slice::<Foo, _>(bytes, bincode::config::standard()).unwrap()
    });
}

fn bincode_encode(b: &mut Bencher) {
    let foo = Foo::default();
    let mut bytes = Vec::with_capacity(128);

    b.iter(|| {
        let foo = black_box(&foo);
        bytes.clear();
        bincode::encode_into_std_write(foo, &mut bytes, bincode::config::standard()).unwrap();
    });
}

fn postcard_deserialize(b: &mut Bencher) {
    let foo = Foo::default();
    let bytes = postcard::to_stdvec(&foo).unwrap();

    b.iter(|| {
        let bytes = black_box(&bytes);
        postcard::from_bytes::<Foo>(bytes).unwrap()
    });
}

fn postcard_serialize(b: &mut Bencher) {
    let foo = Foo::default();
    let mut bytes = Vec::with_capacity(128);

    b.iter(|| {
        let foo = black_box(&foo);
        bytes.clear();
        postcard::serialize_with_flavor(foo, PreallocatedVec::new(&mut bytes)).unwrap();
    });
}

fn postcard2_deserialize(b: &mut Bencher) {
    let foo = Foo::default();
    let bytes = postcard2::to_vec(&foo).unwrap();

    b.iter(|| {
        let bytes = black_box(&bytes);
        postcard2::from_bytes::<Foo>(bytes).unwrap()
    });
}

fn postcard2_serialize(b: &mut Bencher) {
    let foo = Foo::default();
    let mut bytes = Vec::with_capacity(128);

    b.iter(|| {
        let foo = black_box(&foo);
        bytes.clear();
        postcard2::serialize_with_flavor(foo, PreallocatedVec::new(&mut bytes)).unwrap();
    });
}

fn serde_deserialize(b: &mut Bencher) {
    let foo = Foo::default();
    let mut bytes = Vec::new();
    serde_bench::serialize(&mut bytes, &foo).unwrap();

    b.iter(|| {
        let bytes = black_box(&bytes);
        serde_bench::deserialize::<Foo>(bytes).unwrap()
    });
}

fn serde_serialize(b: &mut Bencher) {
    let foo = Foo::default();
    let mut bytes = Vec::with_capacity(128);

    b.iter(|| {
        let foo = black_box(&foo);
        bytes.clear();
        serde_bench::serialize(&mut bytes, foo).unwrap();
    });
}

fn bench(c: &mut Criterion) {
    c.bench_function("bincode_serde_deserialize", bincode_serde_deserialize);
    c.bench_function("bincode_serde_serialize", bincode_serde_serialize);
    c.bench_function("bincode_decode", bincode_decode);
    c.bench_function("bincode_encode", bincode_encode);
    c.bench_function("postcard_deserialize", postcard_deserialize);
    c.bench_function("postcard_serialize", postcard_serialize);
    c.bench_function("postcard2_deserialize", postcard2_deserialize);
    c.bench_function("postcard2_serialize", postcard2_serialize);
    c.bench_function("serde_deserialize", serde_deserialize);
    c.bench_function("serde_serialize", serde_serialize);
}

criterion_group!(benches, bench);
criterion_main!(benches);
