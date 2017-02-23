extern crate byteorder;

extern crate serde;
use serde::{Serialize, Deserialize};

mod ser;
use ser::Serializer;
mod de;
use de::Deserializer;
mod error;
pub use error::{Error, Result};

pub fn serialize<T>(value: &T) -> Result<Vec<u8>>
    where T: Serialize
{
    let mut bytes = Vec::with_capacity(128);
    {
        let mut ser = Serializer::new(&mut bytes);
        try!(Serialize::serialize(value, &mut ser));
    }
    Ok(bytes)
}

pub fn deserialize<T>(mut bytes: &[u8]) -> Result<T>
    where T: Deserialize
{
    let mut de = Deserializer::new(&mut bytes);
    Deserialize::deserialize(&mut de)
}
