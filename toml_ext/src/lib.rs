mod config;
pub use config::*;

pub(crate) mod de;
pub use de::Deserializer;
pub use de::from::*;

pub(crate) mod ser;
pub use ser::Serializer;
pub use ser::to::*;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn to_string_hex_and_base64() {
        #[derive(Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let value = TestStruct {
            data: vec![1, 2, 3, 255],
        };

        let hex = to_string(&value, &Config::default().set_bytes_hex()).unwrap();
        assert!(hex.contains(r#"data = "010203ff""#) || hex.contains(r#"data = "0x010203ff""#));

        let base64 = to_string(&value, &Config::default().set_bytes_base64()).unwrap();
        assert!(base64.contains(r#"data = "AQID/w==""#));

        let base64_url = to_string(&value, &Config::default().set_bytes_base64_url_safe()).unwrap();
        assert!(base64_url.contains(r#"data = "AQID_w==""#));
    }

    #[test]
    fn to_string_default_array() {
        #[derive(Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let value = TestStruct {
            data: vec![1, 2, 3, 255],
        };

        let text = to_string(&value, &Config::default().set_bytes_default()).unwrap();
        assert!(text.contains("data = [1, 2, 3, 255]"));
    }

    #[test]
    fn from_str_hex_and_base64() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let hex = r#"data = "0x0000ff""#;
        let value: TestStruct = from_str(hex, &Config::default().set_bytes_hex()).unwrap();
        assert_eq!(value.data, vec![0, 0, 255]);

        let base64 = r#"data = "AQID/w==""#;
        let value: TestStruct = from_str(base64, &Config::default().set_bytes_base64()).unwrap();
        assert_eq!(value.data, vec![1, 2, 3, 255]);

        let base64_url = r#"data = "AQID_w==""#;
        let value: TestStruct =
            from_str(base64_url, &Config::default().set_bytes_base64_url_safe()).unwrap();
        assert_eq!(value.data, vec![1, 2, 3, 255]);
    }

    #[test]
    fn from_reader_and_value() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let input = b"data = \"010203\"";
        let value: TestStruct =
            from_reader(&input[..], &Config::default().set_bytes_hex()).unwrap();
        assert_eq!(value.data, vec![1, 2, 3]);

        let parsed: toml::Value = toml::from_str(r#"data = "010203""#).unwrap();
        let value: TestStruct = from_value(parsed, &Config::default().set_bytes_hex()).unwrap();
        assert_eq!(value.data, vec![1, 2, 3]);
    }

    #[test]
    fn base64_bytebuf_from_value() {
        let config = Config::default().set_bytes_base64();
        let value = toml::Value::String("AQID/w==".to_string());
        let de = Deserializer::with_config(value, &config);
        let buf: serde_bytes::ByteBuf = serde::Deserialize::deserialize(de).unwrap();
        assert_eq!(buf.into_vec(), vec![1, 2, 3, 255]);
    }

    #[test]
    fn base64_struct_from_value() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let config = Config::default().set_bytes_base64();
        let value: toml::Value = toml::from_str(r#"data = "AQID/w==""#).unwrap();
        let data: TestStruct = from_value(value, &config).unwrap();
        assert_eq!(data.data, vec![1, 2, 3, 255]);
    }
}
