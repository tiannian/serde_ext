use serde::de::{EnumAccess, MapAccess, SeqAccess, Visitor};
use std::fmt;

use crate::{
    BytesFormat, Config,
    de::{
        Deserializer, enum_access::WrapEnumAccess, map_access::WrapMapAccess,
        seq_access::WrapSeqAccess,
    },
};

fn try_decode_bytes(config: &Config, value: &str) -> Option<Vec<u8>> {
    match config.bytes_format {
        BytesFormat::Default => None,
        BytesFormat::Hex => {
            let hex_str = if value.starts_with("0x") || value.starts_with("0X") {
                &value[2..]
            } else {
                value
            };
            hex::decode(hex_str).ok()
        }
        BytesFormat::Base64 | BytesFormat::Base64UrlSafe => {
            use base64::{Engine as _, engine::general_purpose};
            let engine = if matches!(config.bytes_format, BytesFormat::Base64UrlSafe) {
                &general_purpose::URL_SAFE
            } else {
                &general_purpose::STANDARD
            };
            engine.decode(value).ok()
        }
    }
}

pub struct WrapVisitor<'a, V> {
    pub visitor: V,
    pub config: &'a Config,
}

impl<'de, V> Visitor<'de> for WrapVisitor<'de, V>
where
    V: Visitor<'de>,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.visitor.expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_bool(v)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_i8(v)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_i16(v)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_i32(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_i64(v)
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_i128(v)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_u8(v)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_u16(v)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_u32(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_u64(v)
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_u128(v)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_f32(v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_f64(v)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_char(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Some(bytes) = try_decode_bytes(self.config, v) {
            return self.visitor.visit_byte_buf(bytes);
        }
        self.visitor.visit_str(v)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Some(bytes) = try_decode_bytes(self.config, v) {
            return self.visitor.visit_byte_buf(bytes);
        }
        self.visitor.visit_borrowed_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if let Some(bytes) = try_decode_bytes(self.config, &v) {
            return self.visitor.visit_byte_buf(bytes);
        }
        self.visitor.visit_string(v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_bytes(v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_borrowed_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_byte_buf(v)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_none()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let de = Deserializer::with_config(deserializer, self.config);

        self.visitor.visit_some(de)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_unit()
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let de = Deserializer::with_config(deserializer, self.config);
        self.visitor.visit_newtype_struct(de)
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        self.visitor.visit_seq(WrapSeqAccess {
            inner: seq,
            config: self.config,
        })
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        self.visitor.visit_map(WrapMapAccess {
            inner: map,
            config: self.config,
        })
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        self.visitor.visit_enum(WrapEnumAccess {
            inner: data,
            config: self.config,
        })
    }
}
