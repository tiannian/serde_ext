// Bytes deserialization utilities

use crate::{BytesFormat, Config};
use serde::de::Visitor;

/// Deserializes bytes from JSON format based on the configuration
///
/// # Arguments
///
/// * `deserializer` - A `serde_json::de::Deserializer` (consumed)
/// * `config` - Configuration that determines the deserialization format
/// * `visitor` - A visitor that implements `Visitor<'de>`
///
/// # Returns
///
/// Returns `Result<V::Value, serde_json::Error>` indicating success or failure
pub(crate) fn de_bytes<'de, D, V>(
    deserializer: D,
    config: &Config,
    visitor: V,
) -> Result<V::Value, D::Error>
where
    D: serde::de::Deserializer<'de>,
    V: Visitor<'de>,
{
    match config.bytes_format {
        BytesFormat::Default => de_bytes_array(deserializer, visitor),
        BytesFormat::Hex => de_bytes_hex(deserializer, config, visitor),
        BytesFormat::Base64 => de_bytes_base64(deserializer, false, visitor),
        BytesFormat::Base64UrlSafe => de_bytes_base64(deserializer, true, visitor),
    }
}

/// Deserializes bytes from a JSON array of numbers [1, 2, 3]
pub(crate) fn de_bytes_array<'de, D, V>(deserializer: D, visitor: V) -> Result<V::Value, D::Error>
where
    D: serde::de::Deserializer<'de>,
    V: Visitor<'de>,
{
    deserializer.deserialize_bytes(visitor)
}

/// Deserializes bytes from a hexadecimal string "0x1234..." or "1234..."
pub(crate) fn de_bytes_hex<'de, D, V>(
    deserializer: D,
    _config: &Config,
    visitor: V,
) -> Result<V::Value, D::Error>
where
    D: serde::de::Deserializer<'de>,
    V: Visitor<'de>,
{
    struct HexBytesVisitor<V> {
        visitor: V,
    }

    impl<'de, V> Visitor<'de> for HexBytesVisitor<V>
    where
        V: Visitor<'de>,
    {
        type Value = V::Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a hexadecimal string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let hex_str = if v.starts_with("0x") || v.starts_with("0X") {
                &v[2..]
            } else {
                v
            };
            let bytes = hex::decode(hex_str)
                .map_err(|e| E::custom(format!("invalid hex string: {}", e)))?;
            self.visitor.visit_bytes(&bytes)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_str(&v)
        }
    }

    deserializer.deserialize_str(HexBytesVisitor { visitor })
}

/// Deserializes bytes from a Base64 string
///
/// # Arguments
///
/// * `url_safe` - If true, uses URL-safe Base64 decoding, otherwise uses standard Base64
pub(crate) fn de_bytes_base64<'de, D, V>(
    deserializer: D,
    url_safe: bool,
    visitor: V,
) -> Result<V::Value, D::Error>
where
    D: serde::de::Deserializer<'de>,
    V: Visitor<'de>,
{
    struct Base64BytesVisitor<V> {
        url_safe: bool,
        visitor: V,
    }

    impl<'de, V> Visitor<'de> for Base64BytesVisitor<V>
    where
        V: Visitor<'de>,
    {
        type Value = V::Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a base64 string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            use base64::{Engine as _, engine::general_purpose};
            let engine = if self.url_safe {
                &general_purpose::URL_SAFE
            } else {
                &general_purpose::STANDARD
            };
            let bytes = engine
                .decode(v)
                .map_err(|e| E::custom(format!("invalid base64 string: {}", e)))?;
            self.visitor.visit_bytes(&bytes)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_str(&v)
        }
    }

    deserializer.deserialize_str(Base64BytesVisitor { url_safe, visitor })
}
