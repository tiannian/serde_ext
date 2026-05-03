# serde_json_ext

An extension library for `serde_json` that provides configurable bytes serialization formats.

## Features

- **Configurable bytes serialization formats**: Supports multiple byte serialization methods
  - Default format: Array format `[1, 2, 3]`
  - Hexadecimal: `"0x010203"` or `"010203"`
  - Base64: Standard Base64 encoding
  - Base64 URL-safe: URL-safe Base64 encoding
- **Flexible configuration options**:
  - Support for hexadecimal prefix (`0x`)
  - Support for EIP-55 checksum encoding
- **Complete serialization/deserialization support**: Provides the same API as `serde_json` but with custom configuration support

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
serde_json_ext = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_bytes = "0.11"  # For marking byte fields
```

## Usage

### Serialization Example

```rust
use serde_json_ext::{to_string, Config};
use serde::Serialize;

#[derive(Serialize)]
struct MyStruct {
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    name: String,
}

fn main() {
    let data = MyStruct {
        data: vec![0x48, 0x65, 0x6c, 0x6c, 0x6f], // "Hello"
        name: "test".to_string(),
    };

    // Use default format (array)
    let config_default = Config::default().set_bytes_default();
    let json = to_string(&data, &config_default).unwrap();
    println!("Default: {}", json);
    // Output: {"data":[72,101,108,108,111],"name":"test"}

    // Use hexadecimal format (with 0x prefix)
    let config_hex = Config::default().set_bytes_hex().enable_hex_prefix();
    let json = to_string(&data, &config_hex).unwrap();
    println!("Hex: {}", json);
    // Output: {"data":"0x48656c6c6f","name":"test"}

    // Use Base64 format
    let config_base64 = Config::default().set_bytes_base64();
    let json = to_string(&data, &config_base64).unwrap();
    println!("Base64: {}", json);
    // Output: {"data":"SGVsbG8=","name":"test"}

    // Use Base64 URL-safe format
    let config_base64_url = Config::default().set_bytes_base64_url_safe();
    let json = to_string(&data, &config_base64_url).unwrap();
    println!("Base64 URL-safe: {}", json);
}
```

### Deserialization Example

```rust
use serde_json_ext::{from_str, Config};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyStruct {
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    name: String,
}

fn main() {
    // Deserialize from hexadecimal string
    let json = r#"{"data":"0x48656c6c6f","name":"test"}"#;
    let config = Config::default().set_bytes_hex();
    let data: MyStruct = from_str(json, &config).unwrap();
    println!("{:?}", data);
    // Output: MyStruct { data: [72, 101, 108, 108, 111], name: "test" }

    // Deserialize from Base64 string
    let json = r#"{"data":"SGVsbG8=","name":"test"}"#;
    let config = Config::default().set_bytes_base64();
    let data: MyStruct = from_str(json, &config).unwrap();
    println!("{:?}", data);
}
```

### Configuration Options

```rust
use serde_json_ext::Config;

// Create configuration and set byte format
let config = Config::default()
    .set_bytes_hex()           // Set to hexadecimal format
    .enable_hex_prefix()        // Enable 0x prefix
    .enable_hex_eip55();        // Enable EIP-55 checksum encoding

// Or use other formats
let config = Config::default()
    .set_bytes_base64();        // Base64 format

let config = Config::default()
    .set_bytes_base64_url_safe(); // Base64 URL-safe format

let config = Config::default()
    .set_bytes_default();       // Default array format
```

## API Documentation

### Serialization Functions

- `to_string<T>(value: &T, config: &Config) -> Result<String>` - Serialize to string
- `to_string_pretty<T>(value: &T, config: &Config) -> Result<String>` - Serialize to formatted string
- `to_vec<T>(value: &T, config: &Config) -> Result<Vec<u8>>` - Serialize to byte vector
- `to_vec_pretty<T>(value: &T, config: &Config) -> Result<Vec<u8>>` - Serialize to formatted byte vector
- `to_writer<W, T>(writer: &mut W, value: &T, config: &Config) -> Result<()>` - Serialize to writer
- `to_writer_pretty<W, T>(writer: &mut W, value: &T, config: &Config) -> Result<()>` - Serialize to writer with formatting

### Deserialization Functions

- `from_str<'a, T>(s: &'a str, config: &'a Config) -> Result<T>` - Deserialize from string
- `from_slice<'a, T>(v: &'a [u8], config: &'a Config) -> Result<T>` - Deserialize from byte slice
- `from_reader<R, T>(rdr: R, config: &Config) -> Result<T>` - Deserialize from reader

### Configuration Methods

- `set_bytes_default()` - Set byte format to default array format
- `set_bytes_hex()` - Set byte format to hexadecimal
- `set_bytes_base64()` - Set byte format to Base64
- `set_bytes_base64_url_safe()` - Set byte format to Base64 URL-safe
- `enable_hex_prefix()` / `disable_hex_prefix()` - Enable/disable hexadecimal prefix
- `enable_hex_eip55()` / `disable_hex_eip55()` - Enable/disable EIP-55 checksum encoding

## Supported Formats

### Default Format (Array)
```json
{"data": [72, 101, 108, 108, 111]}
```

### Hexadecimal Format
```json
{"data": "0x48656c6c6f"}  // With prefix
{"data": "48656c6c6f"}    // Without prefix
```

### Base64 Format
```json
{"data": "SGVsbG8="}
```

### Base64 URL-Safe Format
```json
{"data": "SGVsbG8="}
```

## Notes

- Use `#[serde(with = "serde_bytes")]` attribute to mark byte fields that need special serialization
- Serialization and deserialization must use the same configuration format
- Hexadecimal strings can be with or without `0x` prefix, both are handled correctly during deserialization

## License

MIT
