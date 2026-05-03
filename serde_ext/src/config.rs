/// Bytes encoding format shared across serde extension crates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BytesFormat {
    /// Default format (array of numbers).
    Default,
    /// Hexadecimal encoding.
    Hex,
    /// Base64 encoding.
    Base64,
    /// Base64 URL-safe encoding.
    Base64UrlSafe,
}

/// Configuration shared by `serde_json_ext` and `toml_ext`.
#[derive(Debug, Clone)]
pub struct Config {
    /// Bytes encoding format.
    pub bytes_format: BytesFormat,
    /// Enable `0x` prefix for hex values.
    pub hex_prefix: bool,
    /// Enable EIP-55 checksum encoding for hex addresses.
    pub hex_eip55: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bytes_format: BytesFormat::Default,
            hex_prefix: false,
            hex_eip55: false,
        }
    }
}

impl Config {
    /// Sets bytes format to default (array of numbers).
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

    /// Sets bytes format to base64 URL-safe.
    pub fn set_bytes_base64_url_safe(mut self) -> Self {
        self.bytes_format = BytesFormat::Base64UrlSafe;
        self
    }

    /// Enables `0x` prefix for hex values.
    pub fn enable_hex_prefix(mut self) -> Self {
        self.hex_prefix = true;
        self
    }

    /// Disables `0x` prefix for hex values.
    pub fn disable_hex_prefix(mut self) -> Self {
        self.hex_prefix = false;
        self
    }

    /// Enables EIP-55 checksum encoding for hex addresses.
    pub fn enable_hex_eip55(mut self) -> Self {
        self.hex_eip55 = true;
        self
    }

    /// Disables EIP-55 checksum encoding for hex addresses.
    pub fn disable_hex_eip55(mut self) -> Self {
        self.hex_eip55 = false;
        self
    }
}

/// Encodes bytes as hex, applying the configured prefix if requested.
pub fn encode_hex(config: &Config, value: &[u8]) -> String {
    let encoded = hex::encode(value);
    if config.hex_prefix {
        format!("0x{encoded}")
    } else {
        encoded
    }
}

/// Encodes bytes as standard base64.
pub fn encode_base64(value: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};

    general_purpose::STANDARD.encode(value)
}

/// Encodes bytes as URL-safe base64.
pub fn encode_base64_url_safe(value: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};

    general_purpose::URL_SAFE.encode(value)
}
