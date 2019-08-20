extern crate byteorder;

extern crate serde;
use serde::{Deserialize, Serialize};

mod ser;
use ser::Serializer;
mod de;
use de::Deserializer;
mod error;
pub use error::{Error, Result};

pub fn serialize<T>(out: &mut Vec<u8>, value: &T) -> Result<()>
where
    T: Serialize,
{
    let mut ser = Serializer::new(out);
    Serialize::serialize(value, &mut ser)
}

pub fn deserialize<'de, T>(bytes: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut de = Deserializer::new(bytes);
    Deserialize::deserialize(&mut de)
}
