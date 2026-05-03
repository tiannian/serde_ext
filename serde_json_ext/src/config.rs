/// Bytes encoding format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BytesFormat {
    /// Default format (array of numbers)
    Default,
    /// Hexadecimal encoding
    Hex,
    /// Base64 encoding
    Base64,
    /// Base64 URL-safe encoding
    Base64UrlSafe,
}

/// Configuration for serde_json operations
#[derive(Debug, Clone)]
pub struct Config {
    /// Bytes encoding format
    pub(crate) bytes_format: BytesFormat,
    /// Enable EIP-55 checksum encoding for hex addresses
    pub(crate) hex_eip55: bool,
    /// Enable 0x prefix for hex values
    pub(crate) hex_prefix: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bytes_format: BytesFormat::Default,
            hex_eip55: false,
            hex_prefix: false,
        }
    }
}

impl Config {
    /// Sets bytes format to default (array of numbers)
    pub fn set_bytes_default(mut self) -> Self {
        self.bytes_format = BytesFormat::Default;
        self
    }

    /// Sets bytes format to hexadecimal
    pub fn set_bytes_hex(mut self) -> Self {
        self.bytes_format = BytesFormat::Hex;
        self
    }

    /// Sets bytes format to base64
    pub fn set_bytes_base64(mut self) -> Self {
        self.bytes_format = BytesFormat::Base64;
        self
    }

    /// Sets bytes format to base64 URL-safe
    pub fn set_bytes_base64_url_safe(mut self) -> Self {
        self.bytes_format = BytesFormat::Base64UrlSafe;
        self
    }

    /// Enables EIP-55 checksum encoding for hex addresses
    pub fn enable_hex_eip55(mut self) -> Self {
        self.hex_eip55 = true;
        self
    }

    /// Disables EIP-55 checksum encoding for hex addresses
    pub fn disable_hex_eip55(mut self) -> Self {
        self.hex_eip55 = false;
        self
    }

    /// Enables 0x prefix for hex values
    pub fn enable_hex_prefix(mut self) -> Self {
        self.hex_prefix = true;
        self
    }

    /// Disables 0x prefix for hex values
    pub fn disable_hex_prefix(mut self) -> Self {
        self.hex_prefix = false;
        self
    }
}
