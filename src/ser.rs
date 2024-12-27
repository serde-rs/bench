use crate::{Error, Result};
use byteorder::{NativeEndian, WriteBytesExt};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use std::io::Write;

pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W>
where
    W: Write,
{
    pub fn new(w: W) -> Self {
        Serializer { writer: w }
    }
}

impl<W> serde::Serializer for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.writer.write_u8(v as u8).map_err(From::from)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.writer.write_u8(v).map_err(From::from)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.writer.write_u16::<NativeEndian>(v).map_err(From::from)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.writer.write_u32::<NativeEndian>(v).map_err(From::from)
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.writer.write_u64::<NativeEndian>(v).map_err(From::from)
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.writer.write_i8(v).map_err(From::from)
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.writer.write_i16::<NativeEndian>(v).map_err(From::from)
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.writer.write_i32::<NativeEndian>(v).map_err(From::from)
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.writer.write_i64::<NativeEndian>(v).map_err(From::from)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<()> {
        self.writer.write_f32::<NativeEndian>(v).map_err(From::from)
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<()> {
        self.writer.write_f64::<NativeEndian>(v).map_err(From::from)
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<()> {
        self.serialize_u64(v.len() as u64)?;
        self.writer.write_all(v.as_bytes()).map_err(From::from)
    }

    #[inline]
    fn serialize_char(self, c: char) -> Result<()> {
        self.writer
            .write_all(encode_utf8(c).as_slice())
            .map_err(From::from)
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.serialize_u64(v.len() as u64)?;
        self.writer.write_all(v).map_err(From::from)
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        self.writer.write_u8(0).map_err(From::from)
    }

    #[inline]
    fn serialize_some<T>(self, v: &T) -> Result<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.writer.write_u8(1)?;
        v.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let len = len.expect("do not know how to serialize a sequence with no length");
        self.serialize_u64(len as u64)?;
        Ok(self)
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_u32(variant_index)?;
        Ok(self)
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let len = len.expect("do not know how to serialize a map with no length");
        self.serialize_u64(len as u64)?;
        Ok(self)
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.serialize_u32(variant_index)?;
        Ok(self)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + serde::ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + serde::ser::Serialize,
    {
        self.serialize_u32(variant_index)?;
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.serialize_u32(variant_index)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<W> SerializeSeq for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<V>(&mut self, value: &V) -> Result<()>
    where
        V: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> SerializeTuple for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<V>(&mut self, value: &V) -> Result<()>
    where
        V: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> SerializeTupleStruct for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<V>(&mut self, value: &V) -> Result<()>
    where
        V: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> SerializeTupleVariant for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<V>(&mut self, value: &V) -> Result<()>
    where
        V: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> SerializeMap for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<K>(&mut self, key: &K) -> Result<()>
    where
        K: ?Sized + serde::Serialize,
    {
        key.serialize(&mut **self)
    }

    #[inline]
    fn serialize_value<V>(&mut self, value: &V) -> Result<()>
    where
        V: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> SerializeStruct for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<V>(&mut self, _key: &'static str, value: &V) -> Result<()>
    where
        V: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W> SerializeStructVariant for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<V>(&mut self, _key: &'static str, value: &V) -> Result<()>
    where
        V: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[inline]
fn encode_utf8(c: char) -> EncodeUtf8 {
    const TAG_CONT: u8 = 0b1000_0000;
    const TAG_TWO_B: u8 = 0b1100_0000;
    const TAG_THREE_B: u8 = 0b1110_0000;
    const TAG_FOUR_B: u8 = 0b1111_0000;
    const MAX_ONE_B: u32 = 0x80;
    const MAX_TWO_B: u32 = 0x800;
    const MAX_THREE_B: u32 = 0x10000;

    let code = c as u32;
    let mut buf = [0; 4];
    let pos = if code < MAX_ONE_B {
        buf[3] = code as u8;
        3
    } else if code < MAX_TWO_B {
        buf[2] = ((code >> 6) & 0x1F) as u8 | TAG_TWO_B;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        2
    } else if code < MAX_THREE_B {
        buf[1] = ((code >> 12) & 0x0F) as u8 | TAG_THREE_B;
        buf[2] = ((code >> 6) & 0x3F) as u8 | TAG_CONT;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        1
    } else {
        buf[0] = ((code >> 18) & 0x07) as u8 | TAG_FOUR_B;
        buf[1] = ((code >> 12) & 0x3F) as u8 | TAG_CONT;
        buf[2] = ((code >> 6) & 0x3F) as u8 | TAG_CONT;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        0
    };
    EncodeUtf8 { buf, pos }
}

struct EncodeUtf8 {
    buf: [u8; 4],
    pos: usize,
}

impl EncodeUtf8 {
    #[inline]
    fn as_slice(&self) -> &[u8] {
        &self.buf[self.pos..]
    }
}
