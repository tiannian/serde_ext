use serde::Deserialize;
use std::io::Cursor;
use toml_ext::{Config, from_reader};

#[derive(Debug, Deserialize)]
struct Packet {
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

fn main() {
    let input = Cursor::new(r#"data = "AQID/w==""#);
    let packet: Packet = from_reader(input, &Config::default().set_bytes_base64()).unwrap();

    println!("{:?}", packet.data);
}
