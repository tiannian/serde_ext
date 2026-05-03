// Bytes serialization utilities

use crate::Config;

/// Serializes bytes as a hexadecimal string "0x1234..." or "1234..."
pub(crate) fn ser_bytes_hex(config: &Config, value: &[u8]) -> String {
    let hex_str = hex::encode(value);

    if config.hex_prefix {
        format!("0x{}", hex_str)
    } else {
        hex_str
    }
}

/// Serializes bytes as a Base64 string
///
/// # Arguments
///
/// * `url_safe` - If true, uses URL-safe Base64 encoding, otherwise uses standard Base64
pub(crate) fn ser_bytes_base64(value: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(value)
}

pub(crate) fn ser_bytes_base64_url_safe(value: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::URL_SAFE.encode(value)
}
