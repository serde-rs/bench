#[macro_use]
extern crate serde_derive;

extern crate bincode;
extern crate byteorder;
extern crate serde;
extern crate serde_bench;

use bincode::Infinite;
use byteorder::NetworkEndian;
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

    let mut bincode_bytes = Vec::new();
    type BincodeSerializer<W> = bincode::internal::Serializer<W, NetworkEndian>;
    foo.serialize(&mut BincodeSerializer::new(&mut bincode_bytes))
        .unwrap();

    let mut serde_bytes = Vec::new();
    serde_bench::serialize(&mut serde_bytes, &foo).unwrap();

    assert_eq!(bincode_bytes, serde_bytes);
}

#[test]
fn test_de() {
    let foo = Foo::default();
    let mut bytes = Vec::new();
    serde_bench::serialize(&mut bytes, &foo).unwrap();

    type BincodeDeserializer<R, S> = bincode::internal::Deserializer<R, S, NetworkEndian>;
    let bincode_read = bincode::read_types::SliceReader::new(&bytes);
    let mut bincode_de = BincodeDeserializer::new(bincode_read, Infinite);
    let bincode_foo = Foo::deserialize(&mut bincode_de).unwrap();
    assert_eq!(bincode_foo, foo);

    let serde_foo = serde_bench::deserialize::<Foo>(&bytes).unwrap();
    assert_eq!(serde_foo, foo);
}
