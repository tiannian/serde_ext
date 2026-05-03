use serde::Deserialize;
use serde_toml_ext::{Config, from_str};

#[derive(Debug, Deserialize)]
struct Packet {
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

fn main() {
    let config = Config::default().set_bytes_hex().disable_hex_prefix();
    let packet: Packet = from_str(r#"data = "000102ff""#, &config).unwrap();

    println!("{:?}", packet.data);
}
