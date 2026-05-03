use serde::Deserialize;
use toml_ext::{Config, from_value};

#[derive(Debug, Deserialize)]
struct Packet {
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

fn main() {
    let value: toml::Value = toml::from_str(r#"data = "0x000102ff""#).unwrap();
    let packet: Packet = from_value(value, &Config::default().set_bytes_hex().enable_hex_prefix())
        .unwrap();

    println!("{:?}", packet.data);
}
