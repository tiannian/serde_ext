# serde_ext_core workspace

This workspace contains serde helper crates for configurable byte encoding and
format-specific serialization support.

## Packages

- [`serde_ext_core`](serde_ext/README.md): shared configuration and low-level
  serializer and deserializer support.
- [`serde_json_ext`](serde_json_ext/README.md): JSON helpers built on top of
  `serde_json`.
- [`toml_ext`](toml_ext/README.md): TOML helpers built on top of `toml`.

## Common behavior

All crates in this workspace share the same byte encoding options:

- default array encoding
- hexadecimal encoding
- base64 encoding
- base64 URL-safe encoding
- optional `0x` prefix for hex values
- optional EIP-55 checksum handling for hex data
