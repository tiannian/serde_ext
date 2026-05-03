// Serialization functions with configuration

use serde_json::ser::{CompactFormatter, PrettyFormatter};

use crate::Config;
// use crate::formatter::{CompactFormatter, PrettyFormatter};
use crate::ser::serializer::Serializer;
use std::io::Write;

/// Serializes a value to a JSON string with the given configuration.
///
/// This function uses a compact formatter (no pretty printing).
///
/// # Example
///
/// ```
/// use serde_json_ext::{to_string, Config};
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let json = to_string(&vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_string<T>(value: &T, config: &Config) -> serde_json::Result<String>
where
    T: ?Sized + serde::Serialize,
{
    let bytes = to_vec(value, config)?;
    Ok(String::from_utf8(bytes).unwrap())
}

/// Serializes a value to a pretty-printed JSON string with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_ext::{to_string_pretty, Config};
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let json = to_string_pretty(&vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_string_pretty<T>(value: &T, config: &Config) -> serde_json::Result<String>
where
    T: ?Sized + serde::Serialize,
{
    let bytes = to_vec_pretty(value, config)?;
    Ok(String::from_utf8(bytes).unwrap())
}

/// Serializes a value to a JSON byte vector with the given configuration.
///
/// This function uses a compact formatter (no pretty printing).
///
/// # Example
///
/// ```
/// use serde_json_ext::{to_vec, Config};
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let json = to_vec(&vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_vec<T>(value: &T, config: &Config) -> serde_json::Result<Vec<u8>>
where
    T: ?Sized + serde::Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value, config)?;
    Ok(writer)
}

/// Serializes a value to a pretty-printed JSON byte vector with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_ext::{to_vec_pretty, Config};
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let json = to_vec_pretty(&vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_vec_pretty<T>(value: &T, config: &Config) -> serde_json::Result<Vec<u8>>
where
    T: ?Sized + serde::Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer_pretty(&mut writer, value, config)?;
    Ok(writer)
}

/// Serializes a value to a JSON writer with the given configuration.
///
/// This function uses a compact formatter (no pretty printing).
///
/// # Example
///
/// ```
/// use serde_json_ext::{to_writer, Config};
/// use std::io::stdout;
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// to_writer(&mut stdout(), &vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_writer<W, T>(writer: &mut W, value: &T, config: &Config) -> serde_json::Result<()>
where
    W: ?Sized + Write,
    T: ?Sized + serde::Serialize,
{
    let formatter = CompactFormatter;
    let mut ser = serde_json::Serializer::with_formatter(writer, formatter);
    let serializer = Serializer::new(&mut ser, config);
    value.serialize(serializer)
}

/// Serializes a value to a pretty-printed JSON writer with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_ext::{to_writer_pretty, Config};
/// use std::io::stdout;
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// to_writer_pretty(&mut stdout(), &vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_writer_pretty<W, T>(writer: &mut W, value: &T, config: &Config) -> serde_json::Result<()>
where
    W: ?Sized + Write,
    T: ?Sized + serde::Serialize,
{
    let formatter = PrettyFormatter::new();
    let mut ser = serde_json::Serializer::with_formatter(writer, formatter);
    let serializer = Serializer::new(&mut ser, config);
    value.serialize(serializer)
}

/// Serializes a value to a `serde_json::Value` with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_ext::{to_value, Config};
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let value = to_value(&vec![1u8, 2u8, 3u8], &config).unwrap();
/// ```
pub fn to_value<T>(value: &T, config: &Config) -> serde_json::Result<serde_json::Value>
where
    T: ?Sized + serde::Serialize,
{
    let ser = serde_json::value::Serializer;

    let serializer = Serializer::new(ser, config);
    value.serialize(serializer)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_to_string_bytes_default() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![1u8, 2u8, 3u8, 255u8],
        };

        let config = Config::default().set_bytes_default();
        let result = to_string(&test_data, &config).unwrap();
        assert_eq!(result, r#"{"data":[1,2,3,255]}"#);
    }

    #[test]
    fn test_to_string_bytes_hex_without_prefix() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![1u8, 2u8, 3u8, 255u8],
        };

        let config = Config::default().set_bytes_hex().disable_hex_prefix();
        let result = to_string(&test_data, &config).unwrap();
        assert_eq!(result, r#"{"data":"010203ff"}"#);
    }

    #[test]
    fn test_to_string_bytes_hex_with_prefix() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![1u8, 2u8, 3u8, 255u8],
        };

        let config = Config::default().set_bytes_hex().enable_hex_prefix();
        let result = to_string(&test_data, &config).unwrap();
        assert_eq!(result, r#"{"data":"0x010203ff"}"#);
    }

    #[test]
    fn test_to_string_bytes_base64() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![1u8, 2u8, 3u8, 255u8],
        };

        let config = Config::default().set_bytes_base64();
        let result = to_string(&test_data, &config).unwrap();
        assert_eq!(result, r#"{"data":"AQID/w=="}"#);
    }

    #[test]
    fn test_to_string_bytes_base64_url_safe() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![1u8, 2u8, 3u8, 255u8],
        };

        let config = Config::default().set_bytes_base64_url_safe();
        let result = to_string(&test_data, &config).unwrap();
        assert_eq!(result, r#"{"data":"AQID_w=="}"#);
    }

    #[test]
    fn test_to_string_bytes_empty() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct { data: vec![] };

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&test_data, &config_default).unwrap();
        assert_eq!(result_default, r#"{"data":[]}"#);

        let config_hex = Config::default().set_bytes_hex().disable_hex_prefix();
        let result_hex = to_string(&test_data, &config_hex).unwrap();
        assert_eq!(result_hex, r#"{"data":""}"#);

        let config_hex_prefix = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex_prefix = to_string(&test_data, &config_hex_prefix).unwrap();
        assert_eq!(result_hex_prefix, r#"{"data":"0x"}"#);

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        assert_eq!(result_base64, r#"{"data":""}"#);

        let config_base64_url = Config::default().set_bytes_base64_url_safe();
        let result_base64_url = to_string(&test_data, &config_base64_url).unwrap();
        assert_eq!(result_base64_url, r#"{"data":""}"#);
    }

    #[test]
    fn test_to_string_bytes_single_byte() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct { data: vec![0u8] };

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&test_data, &config_default).unwrap();
        assert_eq!(result_default, r#"{"data":[0]}"#);

        let config_hex_no_prefix = Config::default().set_bytes_hex().disable_hex_prefix();
        let result_hex_no_prefix = to_string(&test_data, &config_hex_no_prefix).unwrap();
        assert_eq!(result_hex_no_prefix, r#"{"data":"00"}"#);

        let config_hex_prefix = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex_prefix = to_string(&test_data, &config_hex_prefix).unwrap();
        assert_eq!(result_hex_prefix, r#"{"data":"0x00"}"#);

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        assert_eq!(result_base64, r#"{"data":"AA=="}"#);
    }

    #[test]
    fn test_to_string_bytes_large() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: (0u8..=255u8).collect(),
        };

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&test_data, &config_default).unwrap();
        assert!(result_default.starts_with(r#"{"data":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33"#));
        assert!(result_default.ends_with(r#",255]}"#));
        assert_eq!(result_default.len(), 924);

        let config_hex = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex = to_string(&test_data, &config_hex).unwrap();
        assert_eq!(
            result_hex,
            r#"{"data":"0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff"}"#
        );

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        assert_eq!(
            result_base64,
            r#"{"data":"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8gISIjJCUmJygpKissLS4vMDEyMzQ1Njc4OTo7PD0+P0BBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWltcXV5fYGFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6e3x9fn+AgYKDhIWGh4iJiouMjY6PkJGSk5SVlpeYmZqbnJ2en6ChoqOkpaanqKmqq6ytrq+wsbKztLW2t7i5uru8vb6/wMHCw8TFxsfIycrLzM3Oz9DR0tPU1dbX2Nna29zd3t/g4eLj5OXm5+jp6uvs7e7v8PHy8/T19vf4+fr7/P3+/w=="}"#
        );

        let config_base64_url = Config::default().set_bytes_base64_url_safe();
        let result_base64_url = to_string(&test_data, &config_base64_url).unwrap();
        assert_eq!(
            result_base64_url,
            r#"{"data":"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8gISIjJCUmJygpKissLS4vMDEyMzQ1Njc4OTo7PD0-P0BBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWltcXV5fYGFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6e3x9fn-AgYKDhIWGh4iJiouMjY6PkJGSk5SVlpeYmZqbnJ2en6ChoqOkpaanqKmqq6ytrq-wsbKztLW2t7i5uru8vb6_wMHCw8TFxsfIycrLzM3Oz9DR0tPU1dbX2Nna29zd3t_g4eLj5OXm5-jp6uvs7e7v8PHy8_T19vf4-fr7_P3-_w=="}"#
        );
    }

    #[test]
    fn test_to_string_bytes_in_struct() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
            name: String,
        }

        let test_data = TestStruct {
            data: vec![0x48, 0x65, 0x6c, 0x6c, 0x6f], // "Hello" in ASCII
            name: "test".to_string(),
        };

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&test_data, &config_default).unwrap();
        assert_eq!(
            result_default,
            r#"{"data":[72,101,108,108,111],"name":"test"}"#
        );

        let config_hex = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex = to_string(&test_data, &config_hex).unwrap();
        assert_eq!(result_hex, r#"{"data":"0x48656c6c6f","name":"test"}"#);

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        assert_eq!(result_base64, r#"{"data":"SGVsbG8=","name":"test"}"#);
    }

    #[test]
    fn test_to_string_bytes_in_nested_structure() {
        #[derive(serde::Serialize)]
        struct Nested {
            #[serde(with = "serde_bytes")]
            inner: Vec<u8>,
        }

        #[derive(serde::Serialize)]
        struct Outer {
            nested: Nested,
            #[serde(with = "serde_bytes")]
            bytes: Vec<u8>,
        }

        let data = Outer {
            nested: Nested {
                inner: vec![1, 2, 3],
            },
            bytes: vec![4, 5, 6],
        };

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&data, &config_default).unwrap();
        assert_eq!(
            result_default,
            r#"{"nested":{"inner":[1,2,3]},"bytes":[4,5,6]}"#
        );

        let config_hex = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex = to_string(&data, &config_hex).unwrap();
        assert_eq!(
            result_hex,
            r#"{"nested":{"inner":"0x010203"},"bytes":"0x040506"}"#
        );

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&data, &config_base64).unwrap();
        assert_eq!(
            result_base64,
            r#"{"nested":{"inner":"AQID"},"bytes":"BAUG"}"#
        );
    }

    #[test]
    fn test_to_string_bytes_special_values() {
        let test_cases = vec![
            (
                vec![0u8],
                r#"{"data":[0]}"#,
                r#"{"data":"00"}"#,
                r#"{"data":"0x00"}"#,
                r#"{"data":"AA=="}"#,
                r#"{"data":"AA=="}"#,
            ),
            (
                vec![255u8],
                r#"{"data":[255]}"#,
                r#"{"data":"ff"}"#,
                r#"{"data":"0xff"}"#,
                r#"{"data":"/w=="}"#,
                r#"{"data":"_w=="}"#,
            ),
            (
                vec![0u8, 255u8],
                r#"{"data":[0,255]}"#,
                r#"{"data":"00ff"}"#,
                r#"{"data":"0x00ff"}"#,
                r#"{"data":"AP8="}"#,
                r#"{"data":"AP8="}"#,
            ),
            (
                vec![0x00, 0xFF, 0x7F, 0x80],
                r#"{"data":[0,255,127,128]}"#,
                r#"{"data":"00ff7f80"}"#,
                r#"{"data":"0x00ff7f80"}"#,
                r#"{"data":"AP9/gA=="}"#,
                r#"{"data":"AP9_gA=="}"#,
            ),
            (
                vec![0x01, 0x02, 0x03, 0x04, 0x05],
                r#"{"data":[1,2,3,4,5]}"#,
                r#"{"data":"0102030405"}"#,
                r#"{"data":"0x0102030405"}"#,
                r#"{"data":"AQIDBAU="}"#,
                r#"{"data":"AQIDBAU="}"#,
            ),
        ];

        for (
            bytes,
            expected_default,
            expected_hex_no_prefix,
            expected_hex_prefix,
            expected_base64,
            expected_base64_url,
        ) in test_cases
        {
            #[derive(serde::Serialize)]
            struct TestStruct {
                #[serde(with = "serde_bytes")]
                data: Vec<u8>,
            }

            let test_data = TestStruct { data: bytes };

            let config_default = Config::default().set_bytes_default();
            let result_default = to_string(&test_data, &config_default).unwrap();
            assert_eq!(result_default, expected_default);

            let config_hex_no_prefix = Config::default().set_bytes_hex().disable_hex_prefix();
            let result_hex_no_prefix = to_string(&test_data, &config_hex_no_prefix).unwrap();
            assert_eq!(result_hex_no_prefix, expected_hex_no_prefix);

            let config_hex_prefix = Config::default().set_bytes_hex().enable_hex_prefix();
            let result_hex_prefix = to_string(&test_data, &config_hex_prefix).unwrap();
            assert_eq!(result_hex_prefix, expected_hex_prefix);

            let config_base64 = Config::default().set_bytes_base64();
            let result_base64 = to_string(&test_data, &config_base64).unwrap();
            assert_eq!(result_base64, expected_base64);

            let config_base64_url = Config::default().set_bytes_base64_url_safe();
            let result_base64_url = to_string(&test_data, &config_base64_url).unwrap();
            assert_eq!(result_base64_url, expected_base64_url);
        }
    }

    #[test]
    fn test_to_string_bytes_all_formats() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
        };

        let config_default = Config::default().set_bytes_default();
        let result_default = to_string(&test_data, &config_default).unwrap();
        assert_eq!(result_default, r#"{"data":[18,52,86,120,154,188,222,240]}"#);

        let config_hex_no_prefix = Config::default().set_bytes_hex().disable_hex_prefix();
        let result_hex_no_prefix = to_string(&test_data, &config_hex_no_prefix).unwrap();
        assert_eq!(result_hex_no_prefix, r#"{"data":"123456789abcdef0"}"#);

        let config_hex_prefix = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex_prefix = to_string(&test_data, &config_hex_prefix).unwrap();
        assert_eq!(result_hex_prefix, r#"{"data":"0x123456789abcdef0"}"#);

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        assert_eq!(result_base64, r#"{"data":"EjRWeJq83vA="}"#);

        let config_base64_url = Config::default().set_bytes_base64_url_safe();
        let result_base64_url = to_string(&test_data, &config_base64_url).unwrap();
        assert_eq!(result_base64_url, r#"{"data":"EjRWeJq83vA="}"#);
    }

    #[test]
    fn test_to_string_bytes_multiple_fields() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            field1: Vec<u8>,
            #[serde(with = "serde_bytes")]
            field2: Vec<u8>,
            name: String,
        }

        let test_data = TestStruct {
            field1: vec![1, 2, 3],
            field2: vec![4, 5, 6],
            name: "test".to_string(),
        };

        let config_hex = Config::default().set_bytes_hex().enable_hex_prefix();
        let result_hex = to_string(&test_data, &config_hex).unwrap();
        assert_eq!(
            result_hex,
            r#"{"field1":"0x010203","field2":"0x040506","name":"test"}"#
        );

        let config_base64 = Config::default().set_bytes_base64();
        let result_base64 = to_string(&test_data, &config_base64).unwrap();
        assert_eq!(
            result_base64,
            r#"{"field1":"AQID","field2":"BAUG","name":"test"}"#
        );
    }

    #[test]
    fn test_to_value_bytes_default() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            #[serde(with = "serde_bytes")]
            data: Vec<u8>,
        }

        let test_data = TestStruct {
            data: vec![1, 2, 3],
        };
        let config = Config::default().set_bytes_hex().enable_hex_prefix();

        let value = to_value(&test_data, &config).unwrap();

        let expect = json!({
            "data": "0x010203"
        });
        assert_eq!(value, expect);
    }

    #[test]
    fn test_to_value_bytes_hex_newtype_const_array() {
        #[derive(serde::Serialize)]
        #[serde(transparent)]
        struct Address<const N: usize>(#[serde(with = "serde_bytes")] [u8; N]);

        #[derive(serde::Serialize)]
        struct TestStruct {
            address: Address<20>,
        }

        let test_data = TestStruct {
            address: Address([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
            ]),
        };

        let config = Config::default().set_bytes_hex().enable_hex_prefix();

        let value = to_value(&test_data, &config).unwrap();

        let expect = json!({
            "address": "0x000102030405060708090a0b0c0d0e0f10111213"
        });
        assert_eq!(value, expect);
    }
}
