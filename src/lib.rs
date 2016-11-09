extern crate byteorder;
extern crate num_traits;

extern crate serde;
use serde::{Serialize, Deserialize};

mod ser;
use ser::Serializer;
mod de;
use de::Deserializer;
mod error;
pub use error::{Error, Result};

use std::io::{Write, Read};

pub fn serialize<W: ?Sized, T>(writer: &mut W, value: &T) -> Result<()>
    where W: Write,
          T: Serialize
{
    let mut ser = Serializer::new(writer);
    Serialize::serialize(value, &mut ser)
}

pub fn deserialize<R, T>(reader: R) -> Result<T>
    where R: Read,
          T: Deserialize
{
    let mut de = Deserializer::new(reader);
    Deserialize::deserialize(&mut de)
}
