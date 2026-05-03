// Serializer wrapper for serde_json::value::Serializer

use crate::{
    BytesFormat, Config,
    ser::{
        map::WrapSerializeMap,
        seq::WrapSerializeSeq,
        ser_bytes::{ser_bytes_base64, ser_bytes_base64_url_safe, ser_bytes_hex},
        r#struct::WrapSerializeStruct,
        struct_variant::WrapSerializeStructVariant,
        tuple::WrapSerializeTuple,
        tuple_struct::WrapSerializeTupleStruct,
        tuple_variant::WrapSerializeTupleVariant,
        value::WrapValue,
    },
};

/// A wrapper around an inner `serde::Serializer` that implements `Serializer`
pub struct Serializer<'a, S> {
    /// The internal serializer
    pub inner: S,
    /// Configuration for serialization
    pub config: &'a Config,
}

impl<'a, S> Serializer<'a, S>
where
    S: serde::Serializer,
{
    /// Creates a new `Serializer` with custom config
    pub fn new(inner: S, config: &'a Config) -> Self {
        Serializer { inner, config }
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
            BytesFormat::Hex => {
                let s = ser_bytes_hex(self.config, v);
                self.inner.serialize_str(&s)
            }
            BytesFormat::Base64 => {
                let s = ser_bytes_base64(v);
                self.inner.serialize_str(&s)
            }
            BytesFormat::Base64UrlSafe => {
                let s = ser_bytes_base64_url_safe(v);
                self.inner.serialize_str(&s)
            }
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
        self.inner.serialize_newtype_struct(name, value)
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
        self.inner
            .serialize_newtype_variant(name, variant_index, variant, value)
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
        T: ?Sized + std::fmt::Display,
    {
        self.inner.collect_str(value)
    }
}
