# toml_ext

TOML helpers built on top of `toml` with configurable byte serialization and
deserialization behavior.

## Overview

- default array encoding for byte fields
- hexadecimal encoding with optional `0x` prefix
- base64 and base64 URL-safe encoding
- shared `Config` and `BytesFormat` re-exported from `serde_ext`

## Installation

```toml
[dependencies]
toml_ext = "0.1.0"
serde = { version = "1", features = ["derive"] }
serde_bytes = "0.11"
```

## Usage

```rust
use serde::Serialize;
use toml_ext::{to_string, Config};

#[derive(Serialize)]
struct Example {
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

let value = Example { data: vec![1, 2, 3, 255] };
let config = Config::default().set_bytes_hex().enable_hex_prefix();

let toml = to_string(&value, &config).unwrap();
assert!(toml.contains(r#"data = "0x010203ff""#) || toml.contains(r#"data = "010203ff""#));
```

## API

- `to_string`
- `to_string_pretty`
- `from_str`
- `from_reader`
- `from_value`
- `Config`
- `BytesFormat`

## Notes

- Use `#[serde(with = "serde_bytes")]` on byte fields that should use the
  custom encoding rules.
- Serialization and deserialization must use the same `Config` values.

## License

MIT
