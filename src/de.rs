use byteorder::{NativeEndian, ReadBytesExt};
use num_traits;
use serde::de::value::{self, ValueDeserializer};
use serde::de::{self, Visitor, EnumVisitor, SeqVisitor, VariantVisitor};
use serde::{self, Deserialize};
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

    fn read_string(&mut self) -> Result<String> {
        let len = try!(Deserialize::deserialize(self));
        let mut buffer = Vec::new();
        try!(self.reader.by_ref().take(len).read_to_end(&mut buffer));
        String::from_utf8(buffer).map_err(From::from)
    }
}

macro_rules! impl_nums {
    ($ty:ty, $dser_method:ident, $visitor_method:ident, $reader_method:ident) => {
        #[inline]
        fn $dser_method<V>(&mut self, mut visitor: V) -> Result<V::Value>
            where V: Visitor
        {
            let value = try!(self.reader.$reader_method::<NativeEndian>());
            visitor.$visitor_method(value)
        }
    };
}


impl<R: Read> serde::Deserializer for Deserializer<R> {
    type Error = Error;

    fn deserialize<V>(&mut self, _visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        // not supported
        Err(Error)
    }

    fn deserialize_bool<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        match try!(self.reader.read_u8()) {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            _ => Err(Error),
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
    fn deserialize_u8<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_u8(try!(self.reader.read_u8()))
    }

    #[inline]
    fn deserialize_usize<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        let value = try!(self.reader.read_u64::<NativeEndian>());
        match num_traits::cast(value) {
            Some(value) => visitor.visit_usize(value),
            None => Err(Error),
        }
    }

    #[inline]
    fn deserialize_i8<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_i8(try!(self.reader.read_i8()))
    }

    #[inline]
    fn deserialize_isize<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        let value = try!(self.reader.read_i64::<NativeEndian>());
        match num_traits::cast(value) {
            Some(value) => visitor.visit_isize(value),
            None => Err(Error),
        }
    }

    fn deserialize_unit<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_unit()
    }

    fn deserialize_char<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        let mut buf: [u8; 4] = unsafe { mem::uninitialized() };
        try!(self.reader.read_exact(&mut buf[..1]));
        let width = utf8_char_width(buf[0]);
        if width == 1 {
            return visitor.visit_char(buf[0] as char);
        }
        if width == 0 {
            return Err(Error);
        }
        try!(self.reader.read_exact(&mut buf[1..width]));
        let res = try!(match str::from_utf8(&buf[..width]) {
            Ok(s) => Ok(s.chars().next().unwrap()),
            Err(_) => Err(Error),
        });
        visitor.visit_char(res)
    }

    fn deserialize_str<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_str(&try!(self.read_string()))
    }

    fn deserialize_string<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_string(try!(self.read_string()))
    }

    fn deserialize_bytes<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        let len = try!(Deserialize::deserialize(self));
        let mut buf = vec![0; len];
        try!(self.reader.read_exact(&mut buf[..]));
        visitor.visit_byte_buf(buf)
    }

    fn deserialize_enum<V>(&mut self,
                           _enum: &'static str,
                           _variants: &'static [&'static str],
                           mut visitor: V)
                           -> Result<V::Value>
        where V: EnumVisitor
    {
        visitor.visit(self)
    }

    fn deserialize_tuple<V>(&mut self, _len: usize, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_seq(self)
    }

    fn deserialize_seq_fixed_size<V>(&mut self, _: usize, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_seq(self)
    }

    fn deserialize_option<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        match try!(self.reader.read_u8()) {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => Err(Error),
        }
    }

    fn deserialize_seq<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        struct SeqVisitor<'a, R: Read + 'a> {
            deserializer: &'a mut Deserializer<R>,
            remaining: usize,
        }

        impl<'a, R: Read> de::SeqVisitor for SeqVisitor<'a, R> {
            type Error = Error;

            fn visit<T>(&mut self) -> Result<Option<T>>
                where T: Deserialize
            {
                if self.remaining > 0 {
                    self.remaining -= 1;
                    Deserialize::deserialize(self.deserializer).map(Some)
                } else {
                    Ok(None)
                }
            }

            fn end(&mut self) -> Result<()> {
                if self.remaining == 0 {
                    Ok(())
                } else {
                    Err(Error)
                }
            }
        }

        let len = try!(Deserialize::deserialize(self));

        visitor.visit_seq(SeqVisitor {
            deserializer: self,
            remaining: len,
        })
    }

    fn deserialize_map<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        struct MapVisitor<'a, R: Read + 'a> {
            deserializer: &'a mut Deserializer<R>,
            remaining: usize,
        }

        impl<'a, R: Read> de::MapVisitor for MapVisitor<'a, R> {
            type Error = Error;

            fn visit_key<K>(&mut self) -> Result<Option<K>>
                where K: Deserialize
            {
                if self.remaining > 0 {
                    self.remaining -= 1;
                    Deserialize::deserialize(self.deserializer).map(Some)
                } else {
                    Ok(None)
                }
            }

            fn visit_value<V>(&mut self) -> Result<V>
                where V: Deserialize
            {
                Deserialize::deserialize(self.deserializer)
            }

            fn end(&mut self) -> Result<()> {
                if self.remaining == 0 {
                    Ok(())
                } else {
                    Err(Error)
                }
            }
        }

        let len = try!(Deserialize::deserialize(self));

        visitor.visit_map(MapVisitor {
            deserializer: self,
            remaining: len,
        })
    }

    fn deserialize_struct<V>(&mut self,
                             _name: &str,
                             _fields: &'static [&'static str],
                             mut visitor: V)
                             -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_seq(self)
    }

    fn deserialize_struct_field<V>(&mut self, _visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        // not supported
        Err(Error)
    }

    fn deserialize_newtype_struct<V>(&mut self, _name: &str, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_unit_struct<V>(&mut self,
                                  _name: &'static str,
                                  mut visitor: V)
                                  -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_unit()
    }

    fn deserialize_tuple_struct<V>(&mut self,
                                   _name: &'static str,
                                   _len: usize,
                                   mut visitor: V)
                                   -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_seq(self)
    }

    fn deserialize_ignored_any<V>(&mut self, _visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        // not supported
        Err(Error)
    }
}

// For tuples, structs, tuple structs, and fixed size seqs.
impl<R: Read> SeqVisitor for Deserializer<R> {
    type Error = Error;

    fn visit<T>(&mut self) -> Result<Option<T>>
        where T: Deserialize
    {
        Deserialize::deserialize(self).map(Some)
    }

    fn end(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<R: Read> VariantVisitor for Deserializer<R> {
    type Error = Error;

    fn visit_variant<V>(&mut self) -> Result<V>
        where V: Deserialize
    {
        let index: u32 = try!(Deserialize::deserialize(self));
        let mut deserializer = (index as usize).into_deserializer();
        let attempt: result::Result<V, value::Error> = Deserialize::deserialize(&mut deserializer);
        Ok(try!(attempt))
    }

    fn visit_unit(&mut self) -> Result<()> {
        Ok(())
    }

    fn visit_newtype<T>(&mut self) -> Result<T>
        where T: Deserialize
    {
        Deserialize::deserialize(self)
    }

    fn visit_tuple<V>(&mut self, _len: usize, mut visitor: V) -> Result<V::Value>
        where V: Visitor
    {
        visitor.visit_seq(self)
    }

    fn visit_struct<V>(&mut self,
                       _fields: &'static [&'static str],
                       mut visitor: V)
                       -> Result<V::Value>
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

fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}
