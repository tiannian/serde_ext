use serde::Serialize;
use serde_toml_ext::{Config, to_string_pretty};

#[derive(Serialize)]
struct Packet {
    name: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

fn main() {
    let packet = Packet {
        name: "example".to_string(),
        data: vec![1, 2, 3, 255],
    };

    let pretty = to_string_pretty(&packet, &Config::default().set_bytes_hex().enable_hex_prefix())
        .unwrap();

    println!("{pretty}");
}
