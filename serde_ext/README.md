# serde_ext_core

Shared configuration and low-level serializer and deserializer support for the
workspace crates.

## Overview

- `BytesFormat` selects the byte representation.
- `Config` controls hex prefix and hex checksum behavior.
- `ser` and `de` expose the building blocks used by the format-specific crates.

## Installation

```toml
[dependencies]
serde_ext_core = { version = "0.1.0" }
serde = { version = "1", features = ["derive"] }
serde_bytes = "0.11"
```

## Usage

```rust
use serde_ext_core::{BytesFormat, Config};

let config = Config::default()
    .set_bytes_hex()
    .enable_hex_prefix();

assert_eq!(config.bytes_format, BytesFormat::Hex);
assert!(config.hex_prefix);
```

## API

- `BytesFormat`
- `Config`
- `ser`
- `de`

## Notes

- This crate is usually consumed indirectly through
  [`serde_json_ext`](../serde_json_ext/README.md) or
  [`toml_ext`](../toml_ext/README.md).
- Serialization and deserialization helpers in the sibling crates use the same
  `Config` values.
