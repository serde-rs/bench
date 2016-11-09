use byteorder::{NativeEndian, WriteBytesExt};
use serde;
use std::io::Write;
use {Error, Result};

pub struct Serializer<'a, W: ?Sized>
    where W: 'a
{
    writer: &'a mut W,
}

impl<'a, W: ?Sized> Serializer<'a, W>
    where W: Write
{
    pub fn new(w: &'a mut W) -> Self {
        Serializer { writer: w }
    }

    fn serialize_enum_tag(&mut self, tag: usize) -> Result<()> {
        serde::Serializer::serialize_u32(self, tag as u32)
    }
}

impl<'a, W: ?Sized> serde::Serializer for Serializer<'a, W>
    where W: Write
{
    type Error = Error;
    type SeqState = ();
    type TupleState = ();
    type TupleStructState = ();
    type TupleVariantState = ();
    type MapState = ();
    type StructState = ();
    type StructVariantState = ();

    fn serialize_unit(&mut self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(&mut self, _: &'static str) -> Result<()> {
        Ok(())
    }

    fn serialize_bool(&mut self, v: bool) -> Result<()> {
        self.writer.write_u8(if v { 1 } else { 0 }).map_err(From::from)
    }

    fn serialize_u8(&mut self, v: u8) -> Result<()> {
        self.writer.write_u8(v).map_err(From::from)
    }

    fn serialize_u16(&mut self, v: u16) -> Result<()> {
        self.writer.write_u16::<NativeEndian>(v).map_err(From::from)
    }

    fn serialize_u32(&mut self, v: u32) -> Result<()> {
        self.writer.write_u32::<NativeEndian>(v).map_err(From::from)
    }

    fn serialize_u64(&mut self, v: u64) -> Result<()> {
        self.writer.write_u64::<NativeEndian>(v).map_err(From::from)
    }

    fn serialize_usize(&mut self, v: usize) -> Result<()> {
        self.serialize_u64(v as u64)
    }

    fn serialize_i8(&mut self, v: i8) -> Result<()> {
        self.writer.write_i8(v).map_err(From::from)
    }

    fn serialize_i16(&mut self, v: i16) -> Result<()> {
        self.writer.write_i16::<NativeEndian>(v).map_err(From::from)
    }

    fn serialize_i32(&mut self, v: i32) -> Result<()> {
        self.writer.write_i32::<NativeEndian>(v).map_err(From::from)
    }

    fn serialize_i64(&mut self, v: i64) -> Result<()> {
        self.writer.write_i64::<NativeEndian>(v).map_err(From::from)
    }

    fn serialize_isize(&mut self, v: isize) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(&mut self, v: f32) -> Result<()> {
        self.writer.write_f32::<NativeEndian>(v).map_err(From::from)
    }

    fn serialize_f64(&mut self, v: f64) -> Result<()> {
        self.writer.write_f64::<NativeEndian>(v).map_err(From::from)
    }

    fn serialize_str(&mut self, v: &str) -> Result<()> {
        try!(self.serialize_usize(v.len()));
        self.writer.write_all(v.as_bytes()).map_err(From::from)
    }

    fn serialize_char(&mut self, c: char) -> Result<()> {
        self.writer.write_all(encode_utf8(c).as_slice()).map_err(From::from)
    }

    fn serialize_bytes(&mut self, v: &[u8]) -> Result<()> {
        try!(self.serialize_usize(v.len()));
        self.writer.write_all(v).map_err(From::from)
    }

    fn serialize_none(&mut self) -> Result<()> {
        self.writer.write_u8(0).map_err(From::from)
    }

    fn serialize_some<T>(&mut self, v: T) -> Result<()>
        where T: serde::Serialize
    {
        try!(self.writer.write_u8(1));
        v.serialize(self)
    }

    fn serialize_seq(&mut self, len: Option<usize>) -> Result<()> {
        let len = len.expect("do not know how to serialize a sequence with no length");
        self.serialize_usize(len)
    }

    fn serialize_seq_elt<V>(&mut self, _: &mut (), value: V) -> Result<()>
        where V: serde::Serialize
    {
        value.serialize(self)
    }

    fn serialize_seq_end(&mut self, _: ()) -> Result<()> {
        Ok(())
    }

    fn serialize_seq_fixed_size(&mut self, _len: usize) -> Result<()> {
        Ok(())
    }

    fn serialize_tuple(&mut self, _len: usize) -> Result<()> {
        Ok(())
    }

    fn serialize_tuple_elt<V>(&mut self, _: &mut (), value: V) -> Result<()>
        where V: serde::Serialize
    {
        value.serialize(self)
    }

    fn serialize_tuple_end(&mut self, _: ()) -> Result<()> {
        Ok(())
    }

    fn serialize_tuple_struct(&mut self, _name: &'static str, _len: usize) -> Result<()> {
        Ok(())
    }

    fn serialize_tuple_struct_elt<V>(&mut self, _: &mut (), value: V) -> Result<()>
        where V: serde::Serialize
    {
        value.serialize(self)
    }

    fn serialize_tuple_struct_end(&mut self, _: ()) -> Result<()> {
        Ok(())
    }

    fn serialize_tuple_variant(&mut self,
                               _name: &'static str,
                               variant_index: usize,
                               _variant: &'static str,
                               _len: usize)
                               -> Result<()> {
        self.serialize_enum_tag(variant_index)
    }

    fn serialize_tuple_variant_elt<V>(&mut self, _: &mut (), value: V) -> Result<()>
        where V: serde::Serialize
    {
        value.serialize(self)
    }

    fn serialize_tuple_variant_end(&mut self, _: ()) -> Result<()> {
        Ok(())
    }

    fn serialize_map(&mut self, len: Option<usize>) -> Result<()> {
        let len = len.expect("do not know how to serialize a map with no length");
        self.serialize_usize(len)
    }

    fn serialize_map_key<K>(&mut self, _: &mut (), key: K) -> Result<()>
        where K: serde::Serialize
    {
        key.serialize(self)
    }

    fn serialize_map_value<V>(&mut self, _: &mut (), value: V) -> Result<()>
        where V: serde::Serialize
    {
        value.serialize(self)
    }

    fn serialize_map_end(&mut self, _: ()) -> Result<()> {
        Ok(())
    }

    fn serialize_struct(&mut self, _name: &'static str, _len: usize) -> Result<()> {
        Ok(())
    }

    fn serialize_struct_elt<V>(&mut self, _: &mut (), _key: &'static str, value: V) -> Result<()>
        where V: serde::Serialize
    {
        value.serialize(self)
    }

    fn serialize_struct_end(&mut self, _: ()) -> Result<()> {
        Ok(())
    }

    fn serialize_struct_variant(&mut self,
                                _name: &'static str,
                                variant_index: usize,
                                _variant: &'static str,
                                _len: usize)
                                -> Result<()> {
        self.serialize_enum_tag(variant_index)
    }

    fn serialize_struct_variant_elt<V>(&mut self,
                                       _: &mut (),
                                       _key: &'static str,
                                       value: V)
                                       -> Result<()>
        where V: serde::Serialize
    {
        value.serialize(self)
    }

    fn serialize_struct_variant_end(&mut self, _: ()) -> Result<()> {
        Ok(())
    }

    fn serialize_newtype_struct<T>(&mut self, _name: &'static str, value: T) -> Result<()>
        where T: serde::ser::Serialize
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(&mut self,
                                    _name: &'static str,
                                    variant_index: usize,
                                    _variant: &'static str,
                                    value: T)
                                    -> Result<()>
        where T: serde::ser::Serialize
    {
        try!(self.serialize_enum_tag(variant_index));
        value.serialize(self)
    }

    fn serialize_unit_variant(&mut self,
                              _name: &'static str,
                              variant_index: usize,
                              _variant: &'static str)
                              -> Result<()> {
        self.serialize_enum_tag(variant_index)
    }
}

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
        buf[2] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        2
    } else if code < MAX_THREE_B {
        buf[1] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
        buf[2] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        1
    } else {
        buf[0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
        buf[1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
        buf[2] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        0
    };
    EncodeUtf8 {
        buf: buf,
        pos: pos,
    }
}

struct EncodeUtf8 {
    buf: [u8; 4],
    pos: usize,
}

impl EncodeUtf8 {
    fn as_slice(&self) -> &[u8] {
        &self.buf[self.pos..]
    }
}
