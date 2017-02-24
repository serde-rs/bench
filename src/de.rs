use byteorder::{NetworkEndian, ReadBytesExt};
use serde::de::value::{self, ValueDeserializer};
use serde::de::{self, Deserialize, DeserializeSeed, Visitor, EnumVisitor, SeqVisitor, VariantVisitor};
use serde;
use std::io::Read;
use std::{mem, result, str};
use {Error, Result};

pub struct Deserializer<R> {
    reader: R,
}

impl<R: Read> Deserializer<R> {
    pub fn new(reader: R) -> Self {
        Deserializer { reader: reader }
    }

    #[inline]
    fn read_vec(&mut self) -> Result<Vec<u8>> {
        let len = try!(Deserialize::deserialize(&mut *self));
        let mut bytes = Vec::with_capacity(len);
        unsafe { bytes.set_len(len); }
        try!(self.reader.read_exact(&mut bytes));
        Ok(bytes)
    }

    #[inline]
    fn read_string(&mut self) -> Result<String> {
        String::from_utf8(try!(self.read_vec())).map_err(Into::into)
    }
}

macro_rules! impl_nums {
    ($ty:ty, $dser_method:ident, $visitor_method:ident, $reader_method:ident) => {
        #[inline]
        fn $dser_method<V>(self, visitor: V) -> Result<V::Value>
            where V: Visitor
        {
            let value = try!(self.reader.$reader_method::<NetworkEndian>());
            visitor.$visitor_method(value)
        }
    };
}

impl<'a, R: Read> serde::Deserializer for &'a mut Deserializer<R> {
    type Error = Error;

    fn deserialize<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        Err(Error::new("`deserialize` is not supported"))
    }

    #[inline]
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        match try!(self.reader.read_u8()) {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            _ => Err(Error::new("invalid boolean")),
        }
    }

    impl_nums!(u16, deserialize_u16, visit_u16, read_u16);
    impl_nums!(u32, deserialize_u32, visit_u32, read_u32);
    impl_nums!(u64, deserialize_u64, visit_u64, read_u64);
    impl_nums!(i16, deserialize_i16, visit_i16, read_i16);
    impl_nums!(i32, deserialize_i32, visit_i32, read_i32);
    impl_nums!(i64, deserialize_i64, visit_i64, read_i64);
    impl_nums!(f32, deserialize_f32, visit_f32, read_f32);
    impl_nums!(f64, deserialize_f64, visit_f64, read_f64);

    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_u8(try!(self.reader.read_u8()))
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_i8(try!(self.reader.read_i8()))
    }

    #[inline]
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_unit()
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        let mut buf: [u8; 4] = unsafe { mem::uninitialized() };
        try!(self.reader.read_exact(&mut buf[..1]));
        let width = utf8_char_width(buf[0]);
        if width == 1 {
            return visitor.visit_char(buf[0] as char);
        }
        if width == 0 {
            return Err(Error::new("invalid char"));
        }
        try!(self.reader.read_exact(&mut buf[1..width]));
        let res = match str::from_utf8(&buf[..width]) {
            Ok(s) => s.chars().next().unwrap(),
            Err(err) => {
                return Err(err.into());
            }
        };
        visitor.visit_char(res)
    }

    #[inline]
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_str(&try!(self.read_string()))
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_string(try!(self.read_string()))
    }

    #[inline]
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_bytes(&try!(self.read_vec()))
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_byte_buf(try!(self.read_vec()))
    }

    #[inline]
    fn deserialize_enum<V>(self,
                           _enum: &'static str,
                           _variants: &'static [&'static str],
                           visitor: V)
                           -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_enum(self)
    }

    #[inline]
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_seq(self)
    }

    #[inline]
    fn deserialize_seq_fixed_size<V>(self, _: usize, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_seq(self)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        match try!(self.reader.read_u8()) {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => Err(Error::new("invalid Option")),
        }
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        struct SeqVisitor<'a, R: Read + 'a> {
            deserializer: &'a mut Deserializer<R>,
            remaining: usize,
        }

        impl<'a, R: Read> de::SeqVisitor for SeqVisitor<'a, R> {
            type Error = Error;

            #[inline]
            fn visit_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
                where T: DeserializeSeed
            {
                if self.remaining > 0 {
                    self.remaining -= 1;
                    seed.deserialize(&mut *self.deserializer).map(Some)
                } else {
                    Ok(None)
                }
            }
        }

        let len = try!(Deserialize::deserialize(&mut *self));

        visitor.visit_seq(SeqVisitor {
            deserializer: self,
            remaining: len,
        })
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        struct MapVisitor<'a, R: Read + 'a> {
            deserializer: &'a mut Deserializer<R>,
            remaining: usize,
        }

        impl<'a, R: Read> de::MapVisitor for MapVisitor<'a, R> {
            type Error = Error;

            #[inline]
            fn visit_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
                where K: DeserializeSeed
            {
                if self.remaining > 0 {
                    self.remaining -= 1;
                    seed.deserialize(&mut *self.deserializer).map(Some)
                } else {
                    Ok(None)
                }
            }

            #[inline]
            fn visit_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
                where V: DeserializeSeed
            {
                seed.deserialize(&mut *self.deserializer)
            }
        }

        let len = try!(Deserialize::deserialize(&mut *self));

        visitor.visit_map(MapVisitor {
            deserializer: self,
            remaining: len,
        })
    }

    #[inline]
    fn deserialize_struct<V>(self,
                             _name: &str,
                             _fields: &'static [&'static str],
                             visitor: V)
                             -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_seq(self)
    }

    fn deserialize_struct_field<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        Err(Error::new("`deserialize_struct_field` is not supported"))
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_unit()
    }

    #[inline]
    fn deserialize_tuple_struct<V>(self,
                                   _name: &'static str,
                                   _len: usize,
                                   visitor: V)
                                   -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_seq(self)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        Err(Error::new("`deserialize_ignored_any` is not supported"))
    }
}

// For tuples, structs, tuple structs, and fixed size seqs.
impl<R: Read> SeqVisitor for Deserializer<R> {
    type Error = Error;

    #[inline]
    fn visit_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
        where T: DeserializeSeed
    {
        seed.deserialize(self).map(Some)
    }
}

impl<'a, R: Read> EnumVisitor for &'a mut Deserializer<R> {
    type Error = Error;
    type Variant = Self;

    #[inline]
    fn visit_variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
        where V: DeserializeSeed
    {
        let index: u32 = try!(Deserialize::deserialize(&mut *self));
        let deserializer = index.into_deserializer();
        let attempt: result::Result<V::Value, value::Error> = seed.deserialize(deserializer);
        Ok((try!(attempt), self))
    }
}

impl<'a, R: Read> VariantVisitor for &'a mut Deserializer<R> {
    type Error = Error;

    #[inline]
    fn visit_unit(self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn visit_newtype_seed<T>(self, seed: T) -> Result<T::Value>
        where T: DeserializeSeed
    {
        seed.deserialize(self)
    }

    #[inline]
    fn visit_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_seq(self)
    }

    #[inline]
    fn visit_struct<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_seq(self)
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
static UTF8_CHAR_WIDTH: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x1F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x3F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x5F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x7F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x9F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xBF
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // 0xDF
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
];

#[inline]
fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}
