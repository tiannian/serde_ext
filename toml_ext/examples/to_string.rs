use serde::Serialize;
use toml_ext::{Config, to_string};

#[derive(Serialize)]
struct Packet {
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
}

fn main() {
    let packet = Packet {
        data: vec![1, 2, 3, 255],
    };

    let default = to_string(&packet, &Config::default().set_bytes_default()).unwrap();
    let hex = to_string(&packet, &Config::default().set_bytes_hex().enable_hex_prefix()).unwrap();
    let base64 = to_string(&packet, &Config::default().set_bytes_base64()).unwrap();
    let base64_url = to_string(&packet, &Config::default().set_bytes_base64_url_safe()).unwrap();

    println!("default:\n{default}");
    println!("hex:\n{hex}");
    println!("base64:\n{base64}");
    println!("base64_url_safe:\n{base64_url}");
}
