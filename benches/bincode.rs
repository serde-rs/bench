#![feature(test)]

extern crate byteorder;

#[macro_use]
extern crate serde_derive;

extern crate bincode;
extern crate serde;
extern crate serde_bench;
extern crate test;

use bincode::Infinite;
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
    let bytes = bincode::serialize(&foo, Infinite).unwrap();

    b.iter(|| {
        bincode::deserialize::<Foo>(&bytes).unwrap()
    })
}

#[bench]
fn bincode_serialize(b: &mut Bencher) {
    let foo = Foo::default();

    b.iter(|| {
        use serde::Serialize;
        let mut bytes = Vec::with_capacity(128);
        foo.serialize(&mut bincode::internal::Serializer::<_, byteorder::NetworkEndian>::new(&mut bytes)).unwrap()
    })
}

#[bench]
fn serde_deserialize(b: &mut Bencher) {
    let foo = Foo::default();
    let bytes = serde_bench::serialize(&foo).unwrap();

    b.iter(|| {
        serde_bench::deserialize::<Foo>(&bytes).unwrap()
    })
}

#[bench]
fn serde_serialize(b: &mut Bencher) {
    let foo = Foo::default();

    b.iter(|| {
        serde_bench::serialize(&foo).unwrap()
    })
}
