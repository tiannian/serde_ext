use crate::config::{BytesFormat, Config, encode_base64, encode_base64_url_safe, encode_hex};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use std::fmt;

/// Wrapper serializer that applies `Config` while delegating to TOML.
pub struct Serializer<'a, S> {
    pub(crate) inner: S,
    pub(crate) config: &'a Config,
}

impl<'a, S> Serializer<'a, S>
where
    S: serde::Serializer,
{
    pub fn new(inner: S, config: &'a Config) -> Self {
        Self { inner, config }
    }
}

pub(crate) struct WrapValue<'a, T: ?Sized> {
    pub value: &'a T,
    pub config: &'a Config,
}

impl<'a, T: ?Sized> serde::Serialize for WrapValue<'a, T>
where
    T: serde::Serialize,
{
    fn serialize<S2>(&self, serializer: S2) -> Result<S2::Ok, S2::Error>
    where
        S2: serde::Serializer,
    {
        self.value
            .serialize(Serializer::new(serializer, self.config))
    }
}

pub struct WrapSerializeSeq<'a, Seq> {
    pub inner: Seq,
    pub config: &'a Config,
}

impl<'a, Seq> SerializeSeq for WrapSerializeSeq<'a, Seq>
where
    Seq: SerializeSeq,
{
    type Ok = Seq::Ok;
    type Error = Seq::Error;

    fn serialize_element<T: ?Sized + serde::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_element(&WrapValue {
            value,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

pub struct WrapSerializeTuple<'a, Tuple> {
    pub inner: Tuple,
    pub config: &'a Config,
}

impl<'a, Tuple> SerializeTuple for WrapSerializeTuple<'a, Tuple>
where
    Tuple: SerializeTuple,
{
    type Ok = Tuple::Ok;
    type Error = Tuple::Error;

    fn serialize_element<T: ?Sized + serde::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_element(&WrapValue {
            value,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

pub struct WrapSerializeTupleStruct<'a, TupleStruct> {
    pub inner: TupleStruct,
    pub config: &'a Config,
}

impl<'a, TupleStruct> SerializeTupleStruct for WrapSerializeTupleStruct<'a, TupleStruct>
where
    TupleStruct: SerializeTupleStruct,
{
    type Ok = TupleStruct::Ok;
    type Error = TupleStruct::Error;

    fn serialize_field<T: ?Sized + serde::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_field(&WrapValue {
            value,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

pub struct WrapSerializeTupleVariant<'a, TupleVariant> {
    pub inner: TupleVariant,
    pub config: &'a Config,
}

impl<'a, TupleVariant> SerializeTupleVariant for WrapSerializeTupleVariant<'a, TupleVariant>
where
    TupleVariant: SerializeTupleVariant,
{
    type Ok = TupleVariant::Ok;
    type Error = TupleVariant::Error;

    fn serialize_field<T: ?Sized + serde::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_field(&WrapValue {
            value,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

pub struct WrapSerializeMap<'a, Map> {
    pub inner: Map,
    pub config: &'a Config,
}

impl<'a, Map> SerializeMap for WrapSerializeMap<'a, Map>
where
    Map: SerializeMap,
{
    type Ok = Map::Ok;
    type Error = Map::Error;

    fn serialize_key<T: ?Sized + serde::Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        self.inner.serialize_key(&WrapValue {
            value: key,
            config: self.config,
        })
    }

    fn serialize_value<T: ?Sized + serde::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_value(&WrapValue {
            value,
            config: self.config,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

pub struct WrapSerializeStruct<'a, Struct> {
    pub inner: Struct,
    pub config: &'a Config,
}

impl<'a, Struct> SerializeStruct for WrapSerializeStruct<'a, Struct>
where
    Struct: SerializeStruct,
{
    type Ok = Struct::Ok;
    type Error = Struct::Error;

    fn serialize_field<T: ?Sized + serde::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_field(
            key,
            &WrapValue {
                value,
                config: self.config,
            },
        )
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

pub struct WrapSerializeStructVariant<'a, StructVariant> {
    pub inner: StructVariant,
    pub config: &'a Config,
}

impl<'a, StructVariant> SerializeStructVariant for WrapSerializeStructVariant<'a, StructVariant>
where
    StructVariant: SerializeStructVariant,
{
    type Ok = StructVariant::Ok;
    type Error = StructVariant::Error;

    fn serialize_field<T: ?Sized + serde::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.inner.serialize_field(
            key,
            &WrapValue {
                value,
                config: self.config,
            },
        )
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

impl<'a, S> serde::Serializer for Serializer<'a, S>
where
    S: serde::Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = WrapSerializeSeq<'a, S::SerializeSeq>;
    type SerializeTuple = WrapSerializeTuple<'a, S::SerializeTuple>;
    type SerializeTupleStruct = WrapSerializeTupleStruct<'a, S::SerializeTupleStruct>;
    type SerializeTupleVariant = WrapSerializeTupleVariant<'a, S::SerializeTupleVariant>;
    type SerializeMap = WrapSerializeMap<'a, S::SerializeMap>;
    type SerializeStruct = WrapSerializeStruct<'a, S::SerializeStruct>;
    type SerializeStructVariant = WrapSerializeStructVariant<'a, S::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i64(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i128(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u64(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u128(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        match self.config.bytes_format {
            BytesFormat::Default => self.inner.serialize_bytes(v),
            BytesFormat::Hex => self.inner.serialize_str(&encode_hex(self.config, v)),
            BytesFormat::Base64 => self.inner.serialize_str(&encode_base64(v)),
            BytesFormat::Base64UrlSafe => self.inner.serialize_str(&encode_base64_url_safe(v)),
        }
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner.serialize_some(&WrapValue {
            value,
            config: self.config,
        })
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.inner
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner.serialize_newtype_struct(
            name,
            &WrapValue {
                value,
                config: self.config,
            },
        )
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner.serialize_newtype_variant(
            name,
            variant_index,
            variant,
            &WrapValue {
                value,
                config: self.config,
            },
        )
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let inner = self.inner.serialize_seq(len)?;
        Ok(WrapSerializeSeq {
            inner,
            config: self.config,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let inner = self.inner.serialize_tuple(len)?;
        Ok(WrapSerializeTuple {
            inner,
            config: self.config,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let inner = self.inner.serialize_tuple_struct(name, len)?;
        Ok(WrapSerializeTupleStruct {
            inner,
            config: self.config,
        })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let inner = self
            .inner
            .serialize_tuple_variant(name, variant_index, variant, len)?;
        Ok(WrapSerializeTupleVariant {
            inner,
            config: self.config,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let inner = self.inner.serialize_map(len)?;
        Ok(WrapSerializeMap {
            inner,
            config: self.config,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let inner = self.inner.serialize_struct(name, len)?;
        Ok(WrapSerializeStruct {
            inner,
            config: self.config,
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let inner = self
            .inner
            .serialize_struct_variant(name, variant_index, variant, len)?;
        Ok(WrapSerializeStructVariant {
            inner,
            config: self.config,
        })
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + fmt::Display,
    {
        self.inner.collect_str(value)
    }
}
