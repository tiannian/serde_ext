/// Bytes encoding format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BytesFormat {
    /// Default TOML array of numbers.
    Default,
    /// Hexadecimal string.
    Hex,
    /// Base64 string.
    Base64,
    /// URL-safe Base64 string.
    Base64UrlSafe,
}

/// Configuration for `toml_ext`.
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) bytes_format: BytesFormat,
    pub(crate) hex_prefix: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bytes_format: BytesFormat::Default,
            hex_prefix: false,
        }
    }
}

impl Config {
    /// Sets bytes format to the TOML default array representation.
    pub fn set_bytes_default(mut self) -> Self {
        self.bytes_format = BytesFormat::Default;
        self
    }

    /// Sets bytes format to hexadecimal.
    pub fn set_bytes_hex(mut self) -> Self {
        self.bytes_format = BytesFormat::Hex;
        self
    }

    /// Sets bytes format to base64.
    pub fn set_bytes_base64(mut self) -> Self {
        self.bytes_format = BytesFormat::Base64;
        self
    }

    /// Sets bytes format to URL-safe base64.
    pub fn set_bytes_base64_url_safe(mut self) -> Self {
        self.bytes_format = BytesFormat::Base64UrlSafe;
        self
    }

    /// Enables `0x` prefix for hexadecimal strings.
    pub fn enable_hex_prefix(mut self) -> Self {
        self.hex_prefix = true;
        self
    }

    /// Disables `0x` prefix for hexadecimal strings.
    pub fn disable_hex_prefix(mut self) -> Self {
        self.hex_prefix = false;
        self
    }
}

pub(crate) fn encode_hex(config: &Config, value: &[u8]) -> String {
    let encoded = hex::encode(value);
    if config.hex_prefix {
        format!("0x{encoded}")
    } else {
        encoded
    }
}

pub(crate) fn encode_base64(value: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};

    general_purpose::STANDARD.encode(value)
}

pub(crate) fn encode_base64_url_safe(value: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};

    general_purpose::URL_SAFE.encode(value)
}
