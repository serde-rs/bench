#[macro_use]
extern crate serde_derive;

extern crate bincode;
extern crate serde;
extern crate serde_bench;

use bincode::SizeLimit::Infinite;

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
    let bincode_bytes = bincode::serialize(&foo, Infinite).unwrap();
    let serde_bytes = serde_bench::serialize(&foo).unwrap();
    assert_eq!(bincode_bytes, serde_bytes);
}

#[test]
fn test_de() {
    let foo = Foo::default();
    let bytes = serde_bench::serialize(&foo).unwrap();

    let bincode_de = bincode::deserialize::<Foo>(&bytes).unwrap();
    assert_eq!(bincode_de, foo);

    let serde_de = serde_bench::deserialize::<Foo>(&bytes).unwrap();
    assert_eq!(serde_de, foo);
}
